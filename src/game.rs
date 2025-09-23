use std::{
    rc::Rc,
    sync::OnceLock,
};

use glow::Context;
use nalgebra_glm as glm;

use crate::{
    game_level::GameLevel,
    game_object::GameObject,
    resource_manager::{
        self,
        ResourceManager,
    },
    sprite_renderer::SpriteRenderer,
};

static ROOT_PATH: &str = "C:/Users/Osama Awad/RustroverProjects/breakout-rs";

static PLAYER_SIZE: OnceLock<glm::TVec2<f32>> = OnceLock::new();

fn get_player_size() -> &'static glm::TVec2<f32> {
    PLAYER_SIZE.get_or_init(|| glm::vec2(100.0, 20.0))
}

#[derive(PartialEq)]
enum GameState {
    Active,
    Menu,
    Win,
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

        let player = GameObject::new(
            glm::vec2(
                self.width as f32 / 2.0 - get_player_size().x / 2.0,
                self.height as f32 - get_player_size().y,
            ),
            *get_player_size(),
            self.resource_manager.get_texture("paddle"),
            glm::vec3(1.0, 1.0, 1.0),
        );

        let player = Box::new(player);
        self.player = Some(player);

        println!("Loaded textures....");
    }

    pub fn update(&self) {}

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
        }
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
    }
}
