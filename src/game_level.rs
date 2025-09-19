use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
};

use nalgebra_glm as glm;

use crate::{
    game_object::GameObject,
    resource_manager::ResourceManager,
    sprite_renderer::SpriteRenderer,
};

pub struct GameLevel {
    bricks: Vec<GameObject>,
}

impl GameLevel {
    pub fn new() -> Self {
        Self { bricks: Vec::new() }
    }

    pub fn load(
        &mut self,
        file: &str,
        level_width: u32,
        level_height: u32,
        resource_manager: &ResourceManager,
    ) {
        let file = File::open(file).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut tile_data = Vec::<Vec<u32>>::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let nums = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            tile_data.push(nums);
        }

        if tile_data.len() > 0 {
            self.init(tile_data, level_width, level_height, resource_manager);
        }
    }

    fn init(
        &mut self,
        tile_data: Vec<Vec<u32>>,
        level_width: u32,
        level_height: u32,
        resource_manager: &ResourceManager,
    ) {
        let num_tiles_per_row = tile_data[0].len();
        let rows = tile_data.len();

        let unit_width = level_width as f32 / num_tiles_per_row as f32;
        let unit_height = level_height as f32 / rows as f32;

        for y in 0..rows {
            for x in 0..num_tiles_per_row {
                let pos = glm::vec2(unit_width * x as f32, unit_height * y as f32);
                let size = glm::vec2(unit_width, unit_height);

                if tile_data[y][x] == 1 {
                    //solid
                    self.bricks.push(GameObject::new(
                        pos,
                        size,
                        resource_manager.get_texture("block_solid"),
                    ));
                } else if tile_data[y][x] > 1 {
                    self.bricks.push(GameObject::new(
                        pos,
                        size,
                        resource_manager.get_texture("block"),
                    ));
                }

                //let brick = GameObject::new(pos, size);
                //self.bricks.push(brick);
            }
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        for brick in &self.bricks {
            brick.draw(renderer);
        }
    }
}
