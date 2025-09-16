use std::rc::Rc;

use glow::*;

#[derive(Clone)]
pub struct Texture2D {
    gl: Rc<Context>,
    texture: NativeTexture,
}

impl Texture2D {
    pub fn new(gl: Rc<Context>) -> Self {
        let texture;
        unsafe {
            texture = gl.create_texture().unwrap();
        }

        Self { gl, texture }
    }

    pub fn generate(&self, height: u32, width: u32, data: &[u8]) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(Some(data)),
            );

            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        }
    }
}
