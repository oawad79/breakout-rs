use std::rc::Rc;

use glow::{
    Context,
    *,
};
use glutin::{
    context::PossiblyCurrentContext,
    surface::{
        GlSurface,
        Surface,
    },
};
use winit::{
    event::{
        ElementState,
        Event,
        KeyEvent,
        WindowEvent,
    },
    event_loop::EventLoopWindowTarget,
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
    window::Window,
};

use crate::game::Game;

pub struct EventHandler {
    pub current_width: u32,
    pub current_height: u32,
}

impl EventHandler {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            current_width: width,
            current_height: height,
        }
    }
    pub fn handle_event(
        &mut self,
        event: Event<()>,
        elwt: &EventLoopWindowTarget<()>,
        game: &mut Game,
        gl: &Rc<Context>,
        gl_surface: &Surface<glutin::surface::WindowSurface>,
        gl_context: &PossiblyCurrentContext,
        window: &Window,
    ) {
        #[allow(clippy::too_many_arguments)]
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    self.handle_close_requested(elwt);
                }
                WindowEvent::RedrawRequested => {
                    self.handle_redraw_requested(game, gl, gl_surface, gl_context);
                }
                WindowEvent::Resized(physical_size) => {
                    self.handle_resize(physical_size, gl, gl_surface, gl_context, window);
                }
                WindowEvent::KeyboardInput {
                    event: key_event, ..
                } => {
                    self.handle_keyboard_input(key_event, game, window);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    self.handle_mouse_input(state, button, game);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    self.handle_mouse_wheel(delta, game);
                }
                _ => (),
            }
        }
    }

    fn handle_close_requested(&self, elwt: &EventLoopWindowTarget<()>) {
        elwt.exit();
    }

    fn handle_redraw_requested(
        &self,
        game: &mut Game,
        gl: &Rc<Context>,
        gl_surface: &Surface<glutin::surface::WindowSurface>,
        gl_context: &PossiblyCurrentContext,
    ) {
        game.update();

        unsafe {
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        game.render();
        gl_surface.swap_buffers(gl_context).unwrap();
    }

    fn handle_resize(
        &mut self,
        physical_size: winit::dpi::PhysicalSize<u32>,
        gl: &Rc<Context>,
        gl_surface: &Surface<glutin::surface::WindowSurface>,
        gl_context: &PossiblyCurrentContext,
        window: &Window,
    ) {
        self.current_width = physical_size.width;
        self.current_height = physical_size.height;

        // Update OpenGL viewport
        unsafe {
            gl.viewport(0, 0, self.current_width as i32, self.current_height as i32);
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        }

        // Resize the surface
        gl_surface.resize(
            gl_context,
            std::num::NonZeroU32::new(self.current_width.max(1)).unwrap(),
            std::num::NonZeroU32::new(self.current_height.max(1)).unwrap(),
        );

        // Request a redraw to update the scene with new dimensions
        window.request_redraw();
    }

    fn handle_keyboard_input(&self, key_event: KeyEvent, game: &mut Game, window: &Window) {
        if key_event.state.is_pressed() {
            match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW)
                    if !game.keys_processed[KeyCode::KeyW as usize] =>
                {
                    game.current_level = (game.current_level + 1) % game.levels.len();
                    game.keys_processed[KeyCode::KeyW as usize] = true;
                    window.request_redraw();
                }
                PhysicalKey::Code(KeyCode::KeyS)
                    if !game.keys_processed[KeyCode::KeyS as usize] =>
                {
                    if game.current_level > 0 {
                        game.current_level -= 1;
                    } else {
                        game.current_level = game.levels.len() - 1;
                    }
                    game.keys_processed[KeyCode::KeyS as usize] = true;
                    window.request_redraw();
                }
                _ => {}
            }
        } else if key_event.state == ElementState::Released {
            match key_event.physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    game.keys_processed[KeyCode::KeyW as usize] = false;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    game.keys_processed[KeyCode::KeyS as usize] = false;
                }
                _ => {}
            }
        }
    }

    fn handle_mouse_input(
        &self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
        game: &mut Game,
    ) {
        // TODO: Implement mouse input handling
    }

    fn handle_mouse_wheel(&self, delta: winit::event::MouseScrollDelta, game: &mut Game) {
        // TODO: Implement mouse wheel handling for zoom
    }
}
