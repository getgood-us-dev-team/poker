use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Settings,
    Game,
}

