enum GameState {
    GameActive,
    GameMenu,
    GameWin,
}

struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::GameMenu,
        }
    }
}
