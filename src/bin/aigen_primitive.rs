// #![warn(
//     clippy::all,
//     clippy::pedantic,
//     // clippy::restriction,
//     clippy::nursery,
//     clippy::cargo,
// )]

use std::rc::Rc;

use glow::*;
use image::GenericImageView;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn load_texture(gl: &glow::Context, path: &str) -> Result<glow::Texture, Box<dyn std::error::Error>> {
    unsafe {
        let texture = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));

        // Load image using image crate
        let img = image::open(path)?;
        let img = img.flipv(); // Flip vertically for OpenGL
        let img_rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            PixelUnpackData::Slice(Some(&img_rgba)),
        );

        // Set texture parameters
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);

        gl.bind_texture(glow::TEXTURE_2D, None);

        Ok(texture)
    }
}

fn main() {
    unsafe {
        // Create a context from a glutin window on non-wasm32 targets
        let (gl, gl_surface, gl_context, shader_version, window, event_loop) = {
            use std::num::NonZeroU32;

            use glutin::{
                config::{
                    ConfigTemplateBuilder,
                    GlConfig,
                },
                context::{
                    ContextApi,
                    ContextAttributesBuilder,
                    NotCurrentGlContext,
                },
                display::{
                    GetGlDisplay,
                    GlDisplay,
                },
                surface::{
                    GlSurface,
                    SwapInterval,
                },
            };
            use glutin_winit::{
                DisplayBuilder,
                GlWindow,
            };
            use raw_window_handle::HasRawWindowHandle;

            let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
            let window_builder = winit::window::WindowBuilder::new()
                .with_title("Textured Background!")
                .with_inner_size(winit::dpi::LogicalSize::new(SCR_WIDTH as f64, SCR_HEIGHT as f64));

            let template = ConfigTemplateBuilder::new();

            let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            if config.num_samples() > accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .unwrap();

            let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

            let gl_display = gl_config.display();
            let context_attributes = ContextAttributesBuilder::new()
                .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                    major: 4,
                    minor: 1,
                })))
                .build(raw_window_handle);

            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap();

            let window = window.unwrap();

            let attrs = window.build_surface_attributes(Default::default());
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();

            let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

            let gl = Rc::new(glow::Context::from_loader_function_cstr(|s| {
                gl_display.get_proc_address(s)
            }));

            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();

            (
                gl,
                gl_surface,
                gl_context,
                "#version 410",
                window,
                event_loop,
            )
        };

        let vertex_shader_source = r#"
            #version 410 core
            layout (location = 0) in vec4 vertex;  // <vec2 position, vec2 texCoords>

            out vec2 TexCoords;

            uniform mat4 model;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
                TexCoords = vertex.zw;
            }
        "#;

        let fragment_shader_source = r#"
            #version 410 core

            in vec2 TexCoords;
            out vec4 color;

            uniform sampler2D sprite;
            uniform vec3 spriteColor;
            
            void main() {
                color = vec4(spriteColor, 1.0) * texture(sprite, TexCoords);
            }
        "#;

        let program = gl.create_program().expect("Cannot create program");

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        gl.shader_source(vertex_shader, vertex_shader_source);
        gl.compile_shader(vertex_shader);
        if !gl.get_shader_compile_status(vertex_shader) {
            panic!(
                "Vertex shader compilation failed: {}",
                gl.get_shader_info_log(vertex_shader)
            );
        }

        gl.attach_shader(program, vertex_shader);

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        gl.shader_source(fragment_shader, fragment_shader_source);
        gl.compile_shader(fragment_shader);
        if !gl.get_shader_compile_status(fragment_shader) {
            panic!(
                "Fragment shader compilation failed: {}",
                gl.get_shader_info_log(fragment_shader)
            );
        }

        gl.attach_shader(program, fragment_shader);
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

        // Load background texture
        let background_texture = load_texture(&gl, "resources/textures/background.jpg")
            .or_else(|_| load_texture(&gl, "assets/background.png"))
            .or_else(|_| load_texture(&gl, "background.jpg"))
            .or_else(|_| load_texture(&gl, "background.png"))
            .expect("Failed to load background texture. Please ensure you have a background image at assets/background.jpg or assets/background.png");

        // Create a fullscreen quad for the background
        #[rustfmt::skip]
        let vertices = [
            // pos      // tex
            -1.0f32, -1.0, 0.0, 0.0,  // bottom left
             1.0,    -1.0, 1.0, 0.0,  // bottom right
             1.0,     1.0, 1.0, 1.0,  // top right
            
            -1.0,    -1.0, 0.0, 0.0,  // bottom left
             1.0,     1.0, 1.0, 1.0,  // top right
            -1.0,     1.0, 0.0, 1.0,  // top left
        ];
        
        let vbo = gl.create_buffer().expect("Cannot create buffer");
        let vao = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices[..]),
            glow::STATIC_DRAW,
        );

        // Position attribute (location = 0)
        gl.vertex_attrib_pointer_f32(
            0,
            4,  // 4 components: x, y, u, v
            glow::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        let mut current_width = SCR_WIDTH;
        let mut current_height = SCR_HEIGHT;

        // Get uniform locations
        let model_loc = gl.get_uniform_location(program, "model");
        let projection_loc = gl.get_uniform_location(program, "projection");
        let sprite_color_loc = gl.get_uniform_location(program, "spriteColor");

        // Create identity matrices
        let model_matrix: [f32; 16] = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        let projection_matrix: [f32; 16] = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        {
            use glutin::prelude::GlSurface;
            use winit::event::{
                Event,
                WindowEvent,
            };

            let _ = event_loop.run(move |event, elwt| {
                if let Event::WindowEvent { event, .. } = event {
                    use winit::event::KeyEvent;

                    match event {
                        WindowEvent::CloseRequested => {
                            gl.delete_texture(background_texture);
                            gl.delete_buffer(vbo);
                            gl.delete_vertex_array(vao);
                            gl.delete_program(program);

                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // DRAW HERE
                            gl.clear_color(0.0, 0.0, 0.0, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT);

                            // Enable blending for transparency
                            gl.enable(glow::BLEND);
                            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

                            gl.use_program(Some(program));

                            // Set uniforms
                            if let Some(loc) = model_loc {
                                gl.uniform_matrix_4_f32_slice(Some(&loc), false, &model_matrix);
                            }
                            if let Some(loc) = projection_loc {
                                gl.uniform_matrix_4_f32_slice(Some(&loc), false, &projection_matrix);
                            }
                            if let Some(loc) = sprite_color_loc {
                                gl.uniform_3_f32(Some(&loc), 1.0, 1.0, 1.0); // White color
                            }

                            // Bind texture
                            gl.active_texture(glow::TEXTURE0);
                            gl.bind_texture(glow::TEXTURE_2D, Some(background_texture));

                            // Draw background quad
                            gl.bind_vertex_array(Some(vao));
                            gl.draw_arrays(glow::TRIANGLES, 0, 6);
                            gl.bind_vertex_array(None);

                            gl_surface.swap_buffers(&gl_context).unwrap();
                        }
                        WindowEvent::Resized(physical_size) => {
                            current_width = physical_size.width;
                            current_height = physical_size.height;

                            // Update OpenGL viewport
                            gl.viewport(0, 0, current_width as i32, current_height as i32);
                            // Resize the surface
                            gl_surface.resize(
                                &gl_context,
                                std::num::NonZeroU32::new(current_width.max(1)).unwrap(),
                                std::num::NonZeroU32::new(current_height.max(1)).unwrap(),
                            );

                            // Request a redraw to update the scene with new dimensions
                            window.request_redraw();
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key,
                                    state,
                                    ..
                                },
                            ..
                        } => {}
                        WindowEvent::MouseInput { state, button, .. } => {}

                        // Add mouse wheel handling for zoom
                        WindowEvent::MouseWheel { delta, .. } => {}

                        _ => (),
                    }
                }
            });
        }
    }
}
