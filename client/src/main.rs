use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode, MonitorSelection};
use bevy_simple_text_input::*;
use bevy_framepace::{FramepacePlugin, Limiter, FramepaceSettings};
use std::env;

pub const GAME_NAME: &str = "Jack of Diamonds";

// Recovery commit
mod state;
pub use state::GameState;
mod asset_loader;
pub use asset_loader::{AssetLoaderPlugin, GameAssets};
mod screens;
pub use screens::ScreenPlugin;
mod button_manager;
pub use button_manager::{ButtonManagerPlugin, ButtonAction, ButtonPosition};
mod utils;
pub use utils::*;
mod animations;
pub use animations::GameAnimationPlugin;
/*
    Poker game written in Rust using Bevy
    This is the main file that will be used to run the game
    t will handle the game loop and the main menu
*/

// This is the main function that will be used to run the game fr fr
fn main() {
    // Sets the enviroment variable for wgpu backend to use dx12
    #[cfg(target_os = "windows")]
    env::set_var("WGPU_BACKEND", "dx12");

    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: GAME_NAME.to_string(),
            resolution: (1920.0, 1080.0).into(),
            //mode: WindowMode::SizedFullscreen(MonitorSelection::Primary),
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        }),
        close_when_requested: true,
        exit_condition: bevy::window::ExitCondition::OnAllClosed,
        ..Default::default() 
    }))
    .init_state::<GameState>()
    .init_resource::<GameAssets>()
    .add_plugins((TextInputPlugin, FramepacePlugin))
    .add_plugins((AssetLoaderPlugin, ScreenPlugin, ButtonManagerPlugin, GameAnimationPlugin))
    .add_systems(Startup, setup)
    .run();
}



fn setup(mut commands: Commands, mut framespace_settings: ResMut<FramepaceSettings>,) {
    // Spawns the camera
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)));

    // Sets the framepace limiter to 60 fps
    framespace_settings.limiter = Limiter::from_framerate(60.0);

    // Spawns the point light
    commands.spawn((PointLight {
        intensity: 10000.0,
        color: Color::WHITE,
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

