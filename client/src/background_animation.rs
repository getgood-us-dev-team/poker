use bevy::{prelude::*};
use crate::assets::GameAssets;

// Constants for card dimensions
const CARD_HEIGHT: f32 = 140.0;
const CARD_WIDTH: f32 = CARD_HEIGHT * 18.0 / 22.0; // Maintain aspect ratio
const CARD_DEPTH: f32 = 2.0;

pub struct BackgroundAnimationPlugin;

impl Plugin for BackgroundAnimationPlugin { 
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
           .add_systems(Update, (
               rotate_cards,
               update_card_positions,
           ));
    }
}

#[derive(Component)]
struct BackgroundCard {
    rotation_speed: Vec3,
    movement_pattern: MovementPattern,
}

#[derive(Component)]
enum MovementPattern {
    Circle { center: Vec2, radius: f32, speed: f32, phase: f32 },
    Wave { amplitude: f32, frequency: f32, phase: f32 },
    Spiral { speed: f32, expansion_rate: f32, phase: f32 },
}

fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<GameAssets>,
) {
    // Create a basic card mesh
    let card_mesh = meshes.add(Cuboid::new(CARD_WIDTH, CARD_HEIGHT, CARD_DEPTH));

    // Create materials for card faces and edges
    let edge_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    // Create card back material
    let back_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.card_back.clone()),
        ..default()
    });

    // Create card front material
    let front_material = materials.add(StandardMaterial {
        base_color_texture: Some(assets.card_texture.clone()),
        ..default()
    });
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y)));
    // Spawn some initial cards with different patterns
    for i in 0..10 {
        let pattern = match i % 3 {
            0 => MovementPattern::Circle {
                center: Vec2::ZERO,
                radius: 300.0,
                speed: 0.5,
                phase: i as f32 * 0.5,
            },
            1 => MovementPattern::Wave {
                amplitude: 200.0,
                frequency: 1.0,
                phase: i as f32 * 0.7,
            },
            _ => MovementPattern::Spiral {
                speed: 0.3,
                expansion_rate: 0.1,
                phase: i as f32 * 0.3,
            },
        };

        commands.spawn((
            Mesh3d(card_mesh.clone().into()),   
            MeshMaterial3d(materials.add(StandardMaterial {
                
                base_color: Color::WHITE,
                ..default()
            })),
            
            BackgroundCard {
                rotation_speed: Vec3::new(
                    rand::random::<f32>() * 0.5,
                    rand::random::<f32>() * 0.5,
                    rand::random::<f32>() * 0.5,
                ),
                movement_pattern: pattern,
            },
        ));
    }

    // Add a light
    commands.spawn((PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        ..default()
    }, Transform::from_xyz(4.0, 8.0, 4.0)));
}

fn rotate_cards(
    time: Res<Time>,
    mut query: Query<(&BackgroundCard, &mut Transform)>,
) {
    for (card, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_euler(
            EulerRot::XYZ,
            card.rotation_speed.x * time.delta_secs(),
            card.rotation_speed.y * time.delta_secs(),
            card.rotation_speed.z * time.delta_secs(),
        ));
    }
}

fn update_card_positions(
    time: Res<Time>,
    mut query: Query<(&BackgroundCard, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    
    for (card, mut transform) in query.iter_mut() {
        match &card.movement_pattern {
            MovementPattern::Circle { center, radius, speed, phase } => {
                let angle = t * *speed + *phase;
                transform.translation.x = center.x + radius * angle.cos();
                transform.translation.y = center.y + radius * angle.sin();
            }
            MovementPattern::Wave { amplitude, frequency, phase } => {
                let x = t * frequency + phase;
                transform.translation.x = x * 100.0;
                transform.translation.y = amplitude * (x).sin();
            }
            MovementPattern::Spiral { speed, expansion_rate, phase } => {
                let angle = t * *speed + *phase;
                let radius = angle * expansion_rate;
                transform.translation.x = radius * angle.cos();
                transform.translation.y = radius * angle.sin();
            }
        }
    }
} 