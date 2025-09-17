pub enum GameState {
    MainMenu,
    Playing(PlayState),
    Paused,
    GameOver,
    LevelComplete,
    LevelTransition,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
