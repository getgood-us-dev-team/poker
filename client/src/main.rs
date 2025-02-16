use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, MonitorSelection};
mod assets;
use assets::{ AssetsPlugin, GameState};
mod main_menu;
use main_menu::{MainMenuPlugin};
mod settings;
use settings::{SettingsPlugin};
mod background_animation;
use background_animation::BackgroundAnimationPlugin;



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
    .add_plugins((AssetsPlugin, MainMenuPlugin, SettingsPlugin, BackgroundAnimationPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, (game_loop).run_if(in_state(GameState::Game)))
    .run();
}



fn setup(mut commands: Commands) {
    // Add 3D camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn game_loop() {
    println!("Game Loop");
}
