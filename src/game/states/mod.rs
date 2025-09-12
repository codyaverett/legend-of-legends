pub enum GameState {
    MainMenu,
    Playing(PlayState),
    Paused,
    GameOver,
}

pub enum PlayState {
    OnFoot,
    InMech,
    Transition,
    Cutscene,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing(PlayState::OnFoot)
    }
}
