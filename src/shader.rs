use std::rc::Rc;

use glow::*;

pub struct Shader {
    gl: Rc<Context>,
    program: NativeProgram,
}

impl Shader {
    pub fn new(
        gl: Rc<Context>,
        vertex_source: String,
        fragment_source: String,
        geometry_source: Option<String>,
    ) -> Self {
        let program;
        unsafe {
            program = gl.create_program().expect("Cannot create program");
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex_shader, &vertex_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!(
                    "Vertex shader compilation failed: {}",
                    gl.get_shader_info_log(vertex_shader)
                );
            }

            gl.attach_shader(program, vertex_shader);

            let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment_shader, &fragment_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!(
                    "Fragment shader compilation failed: {}",
                    gl.get_shader_info_log(fragment_shader)
                );
            }

            gl.attach_shader(program, fragment_shader);

            let mut geometry_shader: Option<glow::NativeShader> = None;
            if let Some(geometry_source) = geometry_source {
                geometry_shader = Some(gl.create_shader(glow::GEOMETRY_SHADER).unwrap());
                gl.shader_source(geometry_shader.unwrap(), &geometry_source);
                gl.compile_shader(geometry_shader.unwrap());
                if !gl.get_shader_compile_status(geometry_shader.unwrap()) {
                    panic!(
                        "Geometry shader compilation failed: {}",
                        gl.get_shader_info_log(geometry_shader.unwrap())
                    );
                }
                gl.attach_shader(program, geometry_shader.unwrap());
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!(
                    "Program linking failed: {}",
                    gl.get_program_info_log(program)
                );
            }

            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);

            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);

            if let Some(geometry_shader) = geometry_shader {
                gl.detach_shader(program, geometry_shader);
                gl.delete_shader(geometry_shader);
            }
        }
        Self { gl, program }
    }

    pub fn use_program(&self) {
        unsafe {
            self.gl.use_program(Some(self.program));
        }
    }

    pub fn matrix_4_f32(&self, name: &str, matrix: &[f32]) {
        unsafe {
            let location = self.gl.get_uniform_location(self.program, name).unwrap();
            self.gl
                .uniform_matrix_4_f32_slice(Some(&location), false, matrix);
        }
    }

    pub fn clean(&self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}
