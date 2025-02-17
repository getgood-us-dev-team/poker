use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, MonitorSelection};
mod assets;
use assets::{ AssetsPlugin, GameState, Game};
mod main_menu;
use main_menu::{MainMenuPlugin};
mod settings;
use settings::{SettingsPlugin};
mod background_animation;
use background_animation::BackgroundAnimationPlugin;
mod deck;
use deck::{Deck, Card};
mod game;       
use game::GamePlugin;
use bevy_simple_text_input::*;

// Poker game written in Rust using Bevy
// This is the main file that will be used to run the game
// It will handle the game loop and the main menu

// This is the main function that will be used to run the game
fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Poker Game".to_string(),
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
    .init_state::<Game>()
    .add_plugins((TextInputPlugin))
    .add_plugins((AssetsPlugin, MainMenuPlugin, SettingsPlugin, BackgroundAnimationPlugin, GamePlugin))
    .add_systems(Startup, setup)
    .run();
}



fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));
}

