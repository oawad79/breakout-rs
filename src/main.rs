// #![warn(
//     clippy::all,
//     clippy::pedantic,
//     // clippy::restriction,
//     clippy::nursery,
//     clippy::cargo,
// )]

mod event_handler;
mod game;
mod game_level;
mod game_object;
mod resource_manager;
mod shader;
mod sprite_renderer;
mod texture;
mod window;

use game::Game;

use crate::{
    event_handler::EventHandler,
    window::Window,
};

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let Window {
        gl,
        gl_surface,
        gl_context,
        window,
        event_loop,
    } = Window::build();

    let mut game = Game::new(gl.clone(), SCR_WIDTH, SCR_HEIGHT);
    game.init();

    let mut event_handler = EventHandler::new(SCR_WIDTH, SCR_HEIGHT);

    let _ = event_loop.run(move |event, elwt| {
        event_handler.handle_event(
            event,
            elwt,
            &mut game,
            &gl,
            &gl_surface,
            &gl_context,
            &window,
        );
    });
}
