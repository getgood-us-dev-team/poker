use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, MonitorSelection};
use bevy_simple_text_input::*;

pub const GAME_NAME: &str = "Jack of Diamonds";

mod deck;
mod state;
pub use state::GameState;
mod asset_loader;
pub use asset_loader::AssetLoaderPlugin;
mod screens;
pub use screens::ScreenPlugin;
mod button_manager;
pub use button_manager::{ButtonManagerPlugin, ButtonAction};
mod server;
pub use server::CardServer;
/*
    Poker game written in Rust using Bevy
    This is the main file that will be used to run the game
    t will handle the game loop and the main menu
*/

// This is the main function that will be used to run the game fr fr
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: GAME_NAME.to_string(),
            resolution: (1920.0, 1080.0).into(),
            //mode: WindowMode::Fullscreen(MonitorSelection::Primary),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        }),
        close_when_requested: true,
        exit_condition: bevy::window::ExitCondition::OnAllClosed,
        ..Default::default() 
    }))
    .init_state::<GameState>()
    .add_plugins((TextInputPlugin))
    .add_plugins((AssetLoaderPlugin, ScreenPlugin, ButtonManagerPlugin))
    .add_systems(Startup, setup)
    .run();
}



fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

