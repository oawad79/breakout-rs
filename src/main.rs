// #![warn(
//     clippy::all,
//     clippy::pedantic,
//     // clippy::restriction,
//     clippy::nursery,
//     clippy::cargo,
// )]

use std::rc::Rc;

use glow::*;
use nalgebra_glm as glm;

// mod game;
// mod resource_manager;
mod shader;
use shader::Shader;

//use game::Game;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

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
                .with_title("Hello triangle!")
                .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0));

            let template = ConfigTemplateBuilder::new();

            let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

            let (window, gl_config) = display_builder
                .build(&event_loop, template, |configs| {
                    use glutin::config::Config;
                    configs
                        .reduce(|accum: Config, config: Config| {
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

        gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
        //let game = Game::new(SCR_WIDTH, SCR_HEIGHT);
        //game.init(&gl);

        let vertex_shader_source = r#"
            #version 410 core
            layout (location = 0) in vec3 aPos;

            uniform mat4 model;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * model * vec4(aPos.xy, 0.0, 1.0);
            }
        "#;

        let fragment_shader_source = r#"
            #version 410 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0, 0.5, 0.2, 1.0);
            }
        "#;

        let level: Vec<Vec<u32>> = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 1],
            vec![1, 3, 3, 3, 3, 3, 3, 3, 3, 1],
            vec![1, 4, 4, 4, 4, 4, 4, 4, 4, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ]; 

        let shader = Shader::new(gl.clone(), 
            vertex_shader_source.to_string(), 
            fragment_shader_source.to_string(), None);

        #[rustfmt::skip]
        let vertices: [f32; 12] = [
            // pos      // tex
            0.0, 1.0, 
            1.0, 0.0, 
            0.0, 0.0, 
            0.0, 1.0, 
            1.0, 1.0, 
            1.0, 0.0,
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

        gl.vertex_attrib_pointer_f32(
            0,
            2,
            glow::FLOAT,
            false,
            2 * std::mem::size_of::<f32>() as i32,
            0,
        );

        gl.enable_vertex_attrib_array(0);

        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);

        let mut current_width = SCR_WIDTH;
        let mut current_height = SCR_HEIGHT;

        let tile_width = current_width / level[0].len() as u32;
        let tile_height = (current_height as f32 / 2.0) / level.len() as f32;
        let num_tiles = level[0].len();
        let rows = level.len();

        shader.use_program();

        let projection = glm::ortho(0.0, SCR_WIDTH as f32, SCR_HEIGHT as f32, 0.0, -1.0, 1.0);
        shader.matrix_4_f32("projection", projection.as_slice());

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
                            gl.delete_buffer(vbo);
                            gl.delete_vertex_array(vao);
                            shader.clean();

                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            // DRAW HERE
                            gl.clear_color(0.2, 0.3, 0.3, 1.0);

                            // enable depth test and clear the color and depth buffer
                            //gl.enable(glow::DEPTH_TEST);
                            gl.clear(glow::COLOR_BUFFER_BIT);

                            //gl.use_program(Some(program));
                            shader.use_program();

                            for row in 0..rows {
                                for tile in 0..num_tiles {
                                    let mut model = glm::Mat4::identity();
                                    println!("x = {}, y = {}", (tile_width * tile as u32) as f32, (tile_height * row as f32) as f32);
                                    model = glm::translate(&model, &glm::vec3((tile_width * tile as u32) as f32, (tile_height * row as f32) as f32, 0.0));
                                    model = glm::translate(&model, &glm::vec3(0.5 * tile_width as f32, 0.5 * tile_height as f32, 0.0));
                                    model = glm::rotate(&model, 0.0f32.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
                                    model = glm::translate(&model, &glm::vec3(-0.5 * tile_width as f32, -0.5 * tile_height as f32, 0.0));
                                    model = glm::scale(&model, &glm::vec3(tile_width as f32, tile_height as f32, 1.0));
                                    shader.matrix_4_f32("model", model.as_slice());

                                    gl.bind_vertex_array(Some(vao));
                                    gl.draw_arrays(glow::TRIANGLES, 0, 6);
                                    gl.bind_vertex_array(None);

                                }
                            }

                            
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
