use nalgebra_glm as glm;

use crate::sprite_renderer::SpriteRenderer;

pub struct GameObject {
    position: glm::TVec2<f32>,
    size: glm::TVec2<f32>,
}

impl GameObject {
    pub fn new(position: glm::TVec2<f32>, size: glm::TVec2<f32>) -> Self {
        Self { position, size }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        //renderer.draw_sprite(&self.position, &self.size);
    }
}
