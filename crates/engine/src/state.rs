use bevy::prelude::*;

#[derive(Resource)]
pub struct GameResource(pub game::GameState);

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
    GameOver,
}
