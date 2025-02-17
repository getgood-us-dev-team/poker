use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, test_scene0)
        .run();
}

fn test_scene0(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let scene: Handle<Scene> = asset_server.load("scene.gltf:Spades11");
    commands.spawn(Camera3dBundle{
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 10.0),
            ..default()
        },
        ..default()
    });
    commands.spawn(SceneBundle {
        scene,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(10000.0, 10000.0, 10000.0),
            ..default()
        },
        
        ..default()
    });
}

