use std::rc::Rc;

use nalgebra_glm as glm;

use crate::{
    sprite_renderer::SpriteRenderer,
    texture::Texture2D,
};

pub struct GameObject {
    pub position: glm::TVec2<f32>,
    pub size: glm::TVec2<f32>,
    pub velocity: glm::TVec2<f32>,
    sprite: Rc<Texture2D>,
    pub color: glm::TVec3<f32>,
    pub is_solid: bool,
    pub destroyed: bool,
}

impl GameObject {
    pub fn new(
        position: glm::TVec2<f32>,
        size: glm::TVec2<f32>,
        velocity: glm::TVec2<f32>,
        sprite: Rc<Texture2D>,
        color: glm::TVec3<f32>,
    ) -> Self {
        Self {
            position,
            size,
            velocity,
            sprite,
            color,
            is_solid: false,
            destroyed: false,
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        renderer.draw_sprite(&self.sprite, &self.position, &self.size, &self.color);
    }
}
