use std::rc::Rc;

use glow::{
    Context,
    *,
};
use nalgebra_glm as glm;

use crate::{
    shader::Shader,
    texture::Texture2D,
};

pub struct SpriteRenderer {
    shader: Shader,
    gl: Rc<Context>,
    quad_vao: Option<NativeVertexArray>,
}

impl SpriteRenderer {
    pub fn new(gl: Rc<Context>, shader: Shader) -> Self {
        let mut result = Self {
            shader,
            gl,
            quad_vao: None,
        };

        result.init_rendering_data();

        result
    }

    fn init_rendering_data(&mut self) {
        #[rustfmt::skip]
        let vertices = [
            // pos      // tex
            0.0f32, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        unsafe {
            self.gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
            let vbo = self.gl.create_buffer().expect("Cannot create buffer");
            self.quad_vao = Some(
                self.gl
                    .create_vertex_array()
                    .expect("Cannot create vertex array"),
            );

            self.gl.bind_vertex_array(self.quad_vao);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices[..]),
                glow::STATIC_DRAW,
            );

            self.gl.vertex_attrib_pointer_f32(
                0,
                4,
                glow::FLOAT,
                false,
                4 * std::mem::size_of::<f32>() as i32,
                0,
            );

            self.gl.enable_vertex_attrib_array(0);

            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.bind_vertex_array(None);
        }
    }

    pub fn draw_sprite(
        &self,
        texture: &Texture2D,
        position: &glm::TVec2<f32>,
        size: &glm::TVec2<f32>,
        color: &glm::TVec3<f32>,
    ) {
        self.shader.use_program();

        let mut model = glm::Mat4::identity();

        model = glm::translate(&model, &glm::vec3(position.x, position.y, 0.0));
        model = glm::translate(&model, &glm::vec3(0.5 * size.x, 0.5 * size.y, 0.0));
        model = glm::rotate(&model, 0.0f32.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(&model, &glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));
        model = glm::scale(&model, &glm::vec3(size.x, size.y, 1.0));

        self.shader.matrix_4_f32("model", model.as_slice());

        self.shader.set_vector3f("spriteColor", &color);

        unsafe {
            self.gl.active_texture(glow::TEXTURE0);
            texture.bind();

            self.gl.bind_vertex_array(self.quad_vao);
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
            self.gl.bind_vertex_array(None);
        }
    }
}

impl Drop for SpriteRenderer {
    // Destructor
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.quad_vao.unwrap());
        }
    }
}
