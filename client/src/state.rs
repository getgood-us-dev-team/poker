use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum GameState {
    #[default]
    Loading,// Loading screen
    MainMenu,// Main menu
    Settings,// Settings menu
    ServerSelect,// The menu where you pick if you want to host or join a server
    JoinServer,// Join server menu
    Lobby,// Lobby menu (if you are hosting a server, it automatically goes to the lobby)
    InGame,// In game
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum ServerMode {
    #[default]
    None,
    Host,
    Join,
}