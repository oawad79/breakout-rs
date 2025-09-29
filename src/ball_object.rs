use std::rc::Rc;

use nalgebra_glm as glm;

use crate::{
    game_object::GameObject,
    sprite_renderer::SpriteRenderer,
    texture::Texture2D,
};

pub struct BallObject {
    pub game_obj: GameObject,
    pub radius: f32,
    pub stuck: bool,
}

impl BallObject {
    pub fn new(position: glm::TVec2<f32>, sprite: Rc<Texture2D>, radius: f32) -> Self {
        Self {
            game_obj: GameObject::new(
                position,
                glm::vec2(radius * 2.0, radius * 2.0),
                glm::vec2(0.0, 0.0),
                sprite,
                glm::vec3(1.0, 1.0, 1.0),
            ),
            radius,
            stuck: true,
        }
    }

    pub fn move_ball(&mut self, dt: f32, window_width: u32) {
        if !self.stuck {
            // move the ball
            self.game_obj.position += self.game_obj.velocity * dt;
            // then check if outside window bounds and if so, reverse velocity and restore at
            // correct position
            if self.game_obj.position.x <= 0.0 {
                self.game_obj.velocity.x = -self.game_obj.velocity.x;
                self.game_obj.position.x = 0.0;
            } else if self.game_obj.position.x + self.game_obj.size.x >= window_width as f32 {
                self.game_obj.velocity.x = -self.game_obj.velocity.x;
                self.game_obj.position.x = window_width as f32 - self.game_obj.size.x;
            }
            if self.game_obj.position.y <= 0.0 {
                self.game_obj.velocity.y = -self.game_obj.velocity.y;
                self.game_obj.position.y = 0.0;
            }
        }
    }

    pub fn reset(&mut self, position: glm::TVec2<f32>, velocity: glm::TVec2<f32>) {
        self.game_obj.position = position;
        self.game_obj.velocity = velocity;
        self.stuck = true;
        //self.sticky = false;
        //self.pass_through = false;
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        self.game_obj.draw(renderer);
    }
}
