use std::rc::Rc;

use glow::Context;
use nalgebra_glm as glm;

use crate::{
    game_level::GameLevel,
    resource_manager::ResourceManager,
    sprite_renderer::SpriteRenderer,
};

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
    width: u32,
    height: u32,
    pub levels: Vec<GameLevel>,
    renderer: Option<Box<SpriteRenderer>>,
    pub current_level: usize,
    pub keys_processed: [bool; 1024],
}

impl Game {
    pub fn new(gl: Rc<Context>, width: u32, height: u32) -> Self {
        Self {
            resource_manager: ResourceManager { gl: gl.clone() },
            gl,
            state: GameState::Active,
            width,
            height,
            levels: Vec::new(),
            renderer: None,
            current_level: 0,
            keys_processed: [false; 1024],
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

        let mut game_level1 = GameLevel::new();
        game_level1.load("resources/levels/one.lvl", self.width, self.height / 2);
        let mut game_level2 = GameLevel::new();
        game_level2.load("resources/levels/two.lvl", self.width, self.height / 2);
        let mut game_level3 = GameLevel::new();
        game_level3.load("resources/levels/three.lvl", self.width, self.height / 2);
        let mut game_level4 = GameLevel::new();
        game_level4.load("resources/levels/four.lvl", self.width, self.height / 2);

        self.levels.push(game_level1);
        self.levels.push(game_level2);
        self.levels.push(game_level3);
        self.levels.push(game_level4);

        let renderer = SpriteRenderer::new(self.gl.clone(), shader);
        self.renderer = Some(Box::new(renderer));
    }

    pub fn update(&self) {}

    pub fn render(&self) {
        if self.state == GameState::Active {
            self.levels[self.current_level].draw(self.renderer.as_ref().unwrap());
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        if let Some(renderer) = self.renderer.take() {
            drop(renderer);
        }
    }
}
