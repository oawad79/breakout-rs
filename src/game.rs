use glow::Context;

enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}

pub struct Game {
    state: GameState,
    width: u32,
    height: u32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            state: GameState::GameMenu,
            width,
            height,
        }
    }

    pub fn init(&self, gl: &Context) {}
}
