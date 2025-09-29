use std::{
    collections,
    rc::Rc,
};

use glow::Context;
use lazy_static::lazy_static;
use nalgebra_glm as glm;

use crate::{
    ball_object::BallObject,
    game_level::GameLevel,
    game_object::GameObject,
    resource_manager::{
        self,
        ResourceManager,
    },
    sprite_renderer::SpriteRenderer,
};

pub type Collision = (bool, Direction, glm::TVec2<f32>);

static ROOT_PATH: &str = "C:/Users/Osama Awad/RustroverProjects/breakout-rs";

const BALL_RADIUS: f32 = 12.5;

lazy_static! {
    static ref INITIAL_BALL_VELOCITY: glm::TVec2<f32> = glm::vec2(100.0, -350.0);
    static ref PLAYER_SIZE: glm::TVec2<f32> = glm::vec2(100.0, 20.0);
}

#[derive(PartialEq)]
enum GameState {
    Active,
    Menu,
    Win,
}

#[derive(PartialEq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub struct Game {
    resource_manager: ResourceManager,
    gl: Rc<Context>,
    state: GameState,
    pub width: u32,
    pub height: u32,
    pub levels: Vec<GameLevel>,
    renderer: Option<Box<SpriteRenderer>>,
    pub current_level: usize,
    pub keys_processed: [bool; 1024],
    pub player: Option<Box<GameObject>>,
    pub ball: Option<Box<BallObject>>,
    pub lives: u32,
}

impl Game {
    pub fn new(gl: Rc<Context>, width: u32, height: u32) -> Self {
        Self {
            resource_manager: ResourceManager::new(gl.clone()),
            gl,
            state: GameState::Active,
            width,
            height,
            levels: Vec::new(),
            renderer: None,
            current_level: 0,
            keys_processed: [false; 1024],
            player: None,
            ball: None,
            lives: 3,
        }
    }

    pub fn init(&mut self) {
        // load shaders
        let shader = self.resource_manager.load_shader_from_file(
            "resources/shaders/sprite.vs",
            "resources/shaders/sprite.fs",
            None,
        );

        let projection = glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        shader
            .use_program()
            .matrix_4_f32("projection", projection.as_slice());

        self.resource_manager.load_texture_from_file(
            format!("{ROOT_PATH}/resources/textures/background.jpg").as_str(),
            "background",
        );
        self.resource_manager.load_texture_from_file(
            format!("{ROOT_PATH}/resources/textures/block.png").as_str(),
            "block",
        );
        self.resource_manager.load_texture_from_file(
            format!("{ROOT_PATH}/resources/textures/block_solid.png").as_str(),
            "block_solid",
        );

        self.resource_manager.load_texture_from_file(
            format!("{ROOT_PATH}/resources/textures/paddle.png").as_str(),
            "paddle",
        );

        self.resource_manager.load_texture_from_file(
            format!("{ROOT_PATH}/resources/textures/awesomeface.png").as_str(),
            "face",
        );

        let mut game_level1 = GameLevel::new();
        game_level1.load(
            "resources/levels/one.lvl",
            self.width,
            self.height / 2,
            &self.resource_manager,
        );
        let mut game_level2 = GameLevel::new();
        game_level2.load(
            "resources/levels/two.lvl",
            self.width,
            self.height / 2,
            &self.resource_manager,
        );
        let mut game_level3 = GameLevel::new();
        game_level3.load(
            "resources/levels/three.lvl",
            self.width,
            self.height / 2,
            &self.resource_manager,
        );
        let mut game_level4 = GameLevel::new();
        game_level4.load(
            "resources/levels/four.lvl",
            self.width,
            self.height / 2,
            &self.resource_manager,
        );

        self.levels.push(game_level1);
        self.levels.push(game_level2);
        self.levels.push(game_level3);
        self.levels.push(game_level4);

        let renderer = SpriteRenderer::new(self.gl.clone(), shader);
        self.renderer = Some(Box::new(renderer));

        let player_pos = glm::vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y,
        );
        let player = GameObject::new(
            player_pos,
            *PLAYER_SIZE,
            glm::vec2(0.0, 0.0),
            self.resource_manager.get_texture("paddle"),
            glm::vec3(1.0, 1.0, 1.0),
        );

        let player = Box::new(player);
        self.player = Some(player);

        let ball_pos =
            player_pos + glm::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -BALL_RADIUS * 2.0);
        let ball = BallObject::new(
            ball_pos,
            self.resource_manager.get_texture("face"),
            BALL_RADIUS,
        );

        let ball = Box::new(ball);
        self.ball = Some(ball);

        println!("Loaded textures....");
    }

    pub fn update(&mut self, dt: f32) {
        self.ball.as_mut().unwrap().move_ball(dt, self.width);

        self.do_collisions();

        // check loss condition
        if self.ball.as_ref().unwrap().game_obj.position.y >= self.height as f32 {
            // did ball reach bottom edge?
            self.lives -= 1;
            // did the player lose all his lives? : game over
            if self.lives == 0 {
                self.reset_level();
                self.state = GameState::Active;
            }
            self.reset_player();
        }
    }

    pub fn render(&self) {
        if self.state == GameState::Active {
            self.renderer.as_ref().unwrap().draw_sprite(
                &self.resource_manager.get_texture("background"),
                &glm::vec2(0.0, 0.0),
                &glm::vec2(self.width as _, self.height as _),
                &glm::vec3(1.0, 1.0, 1.0),
            );
            self.levels[self.current_level].draw(self.renderer.as_ref().unwrap());
            self.player
                .as_ref()
                .unwrap()
                .draw(self.renderer.as_ref().unwrap());

            self.ball
                .as_ref()
                .unwrap()
                .draw(self.renderer.as_ref().unwrap());
        }
    }

    pub fn reset_level(&mut self) {
        println!("current_level: {}", self.current_level);
        match self.current_level {
            0 => {
                self.levels[0].load(
                    "resources/levels/one.lvl",
                    self.width,
                    self.height / 2,
                    &self.resource_manager,
                );
            }
            1 => {
                self.levels[1].load(
                    "resources/levels/two.lvl",
                    self.width,
                    self.height / 2,
                    &self.resource_manager,
                );
            }
            2 => {
                self.levels[2].load(
                    "resources/levels/three.lvl",
                    self.width,
                    self.height / 2,
                    &self.resource_manager,
                );
            }
            3 => {
                self.levels[3].load(
                    "resources/levels/four.lvl",
                    self.width,
                    self.height / 2,
                    &self.resource_manager,
                );
            }
            _ => {}
        }

        self.lives = 3;
    }

    pub fn reset_player(&mut self) {
        // reset player/ball stats
        self.player.as_mut().unwrap().size = PLAYER_SIZE.clone();
        self.player.as_mut().unwrap().position = glm::vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y,
        );
        self.ball.as_mut().unwrap().reset(
            self.player.as_ref().unwrap().position
                + glm::vec2(PLAYER_SIZE.x / 2.0 - BALL_RADIUS, -(BALL_RADIUS * 2.0)),
            INITIAL_BALL_VELOCITY.clone(),
        );
        // also disable all active powerups
        //self.effects.as_mut().unwrap().chaos = false;
        //self.effects.as_mut().unwrap().confuse = false;
        //self.ball.as_mut().unwrap().pass_through = false;
        //self.ball.as_mut().unwrap().sticky = false;
        self.player.as_mut().unwrap().color = glm::vec3(1.0, 1.0, 1.0);
        self.ball.as_mut().unwrap().game_obj.color = glm::vec3(1.0, 1.0, 1.0);
    }

    fn do_collisions(&mut self) {
        for (index, box_obj) in self.levels[self.current_level]
            .bricks
            .iter_mut()
            .enumerate()
        {
            let collision = Game::check_collision(self.ball.as_ref().unwrap(), box_obj);
            if !box_obj.destroyed && collision.0 && !box_obj.is_solid {
                box_obj.destroyed = true;

                let direction = collision.1;
                let diff_vector = collision.2;
                if direction == Direction::Left || direction == Direction::Right {
                    // horizontal collision
                    self.ball.as_mut().unwrap().game_obj.velocity.x =
                        -self.ball.as_ref().unwrap().game_obj.velocity.x;
                    // relocate
                    let penetration = self.ball.as_ref().unwrap().radius - diff_vector.x.abs();
                    if direction == Direction::Left {
                        self.ball.as_mut().unwrap().game_obj.position.x += penetration; // move ball to right
                    } else {
                        self.ball.as_mut().unwrap().game_obj.position.x -= penetration; // move ball to left;
                    }
                } else {
                    // vertical collision
                    self.ball.as_mut().unwrap().game_obj.velocity.y =
                        -self.ball.as_ref().unwrap().game_obj.velocity.y; // reverse vertical velocity
                    // relocate
                    let penetration = self.ball.as_ref().unwrap().radius - diff_vector.y.abs();
                    if direction == Direction::Up {
                        self.ball.as_mut().unwrap().game_obj.position.y -= penetration; // move ball back up
                    } else {
                        self.ball.as_mut().unwrap().game_obj.position.y += penetration; // move ball back down
                    }
                }
            }
        }

        // and finally check collisions for player pad (unless stuck)
        let result =
            Game::check_collision(self.ball.as_ref().unwrap(), self.player.as_ref().unwrap());
        if !self.ball.as_ref().unwrap().stuck && result.0 {
            // check where it hit the board, and change velocity based on where it hit the board
            let center_board = self.player.as_ref().unwrap().position.x
                + self.player.as_ref().unwrap().size.x / 2.0;
            let distance = self.ball.as_ref().unwrap().game_obj.position.x
                + self.ball.as_ref().unwrap().radius
                - center_board;
            let percentage = distance / (self.player.as_ref().unwrap().size.x / 2.0);
            // then move accordingly
            let strength = 2.0f32;
            let old_velocity = self.ball.as_ref().unwrap().game_obj.velocity;
            self.ball.as_mut().unwrap().game_obj.velocity.x =
                INITIAL_BALL_VELOCITY.x * percentage * strength;
            // self.ball.as_mut().unwrap().game_obj.velocity.y =
            // -self.ball.as_ref().unwrap().game_obj.velocity.y;
            self.ball.as_mut().unwrap().game_obj.velocity =
                glm::normalize(&self.ball.as_ref().unwrap().game_obj.velocity)
                    * glm::length(&old_velocity); // keep speed consistent over both axes (multiply by length of old velocity, so total strength is not changed)
            // fix sticky paddle
            self.ball.as_mut().unwrap().game_obj.velocity.y =
                -1.0 * self.ball.as_ref().unwrap().game_obj.velocity.y.abs();

            // if Sticky powerup is activated, also stick ball to paddle once new velocity vectors
            // were calculated
            //self.ball.as_mut().unwrap().stuck = self.ball.as_ref().unwrap().sticky;
        }
    }

    // fn check_collision(one: &GameObject, two: &GameObject) -> bool {
    //     // AABB - AABB collision
    //     // collision x-axis?
    //     let collision_x = one.position.x + one.size.x >= two.position.x
    //         && two.position.x + two.size.x >= one.position.x;
    //     // collision y-axis?
    //     let collision_y = one.position.y + one.size.y >= two.position.y
    //         && two.position.y + two.size.y >= one.position.y;
    //     // collision only if on both axes
    //     collision_x && collision_y
    // }

    fn check_collision(one: &BallObject, two: &GameObject) -> Collision {
        // AABB - Circle collision
        // get center point circle first
        let center = glm::vec2(
            one.game_obj.position.x + one.radius,
            one.game_obj.position.y + one.radius,
        );
        // calculate AABB info (center, half-extents)
        let aabb_half_extents = glm::vec2(two.size.x / 2.0, two.size.y / 2.0);
        let aabb_center = glm::vec2(
            two.position.x + aabb_half_extents.x,
            two.position.y + aabb_half_extents.y,
        );
        // get difference vector between both centers
        let mut difference = center - aabb_center;
        let clamped = glm::clamp_vec(&difference, &(-aabb_half_extents), &aabb_half_extents);
        // now that we know the clamped values, add this to AABB_center and we get the value of box
        // closest to circle
        let closest = aabb_center + clamped;
        // now retrieve vector between center circle and closest point AABB and check if length <
        // radius
        difference = closest - center;

        if difference.x == 0.0 && difference.y == 0.0 {
            return (false, Direction::Up, glm::vec2(0.0, 0.0));
        }

        if glm::length(&difference) < one.radius {
            // not <= since in that case a collision also occurs when object one exactly touches
            // object two, which they are at the end of each collision resolution stage.
            (true, vector_direction(difference), difference)
        } else {
            (false, Direction::Up, glm::vec2(0.0, 0.0))
        }
    }
}

fn vector_direction(target: glm::TVec2<f32>) -> Direction {
    let compass = [
        glm::vec2(0.0f32, 1.0), // up
        glm::vec2(1.0, 0.0),    // right
        glm::vec2(0.0, -1.0),   // down
        glm::vec2(-1.0, 0.0),   // left
    ];
    let mut max = 0.0f32;
    let mut best_match = -1isize;
    for i in 0..4 {
        let dot_product = glm::dot(&glm::normalize(&target), &compass[i]);
        if dot_product > max {
            max = dot_product;
            best_match = i as isize;
        }
    }
    match best_match {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => panic!(
            "Wrong best_match value was produced within function vector_direction: {}",
            best_match
        ),
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        if let Some(renderer) = self.renderer.take() {
            drop(renderer);
        }

        if let Some(player) = self.player.take() {
            drop(player);
        }

        if let Some(ball) = self.ball.take() {
            drop(ball);
        }
    }
}
