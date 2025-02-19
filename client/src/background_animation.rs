use bevy::{prelude::*, time::Time};
use rand::Rng;
use crate::assets::{GameAssets, Game};
use crate::deck::{Card, Deck};

// Constants for card dimensions
const CARD_HEIGHT: f32 = 140.0;
const CARD_WIDTH: f32 = CARD_HEIGHT * 18.0 / 22.0; // Maintain aspect ratio
const CARD_DEPTH: f32 = 2.0;

// Animation constants
const BACKGROUND_CARD_SPEED: f32 = 100.0;
const FEATURED_CARD_ROTATION_SPEED: f32 = 0.5;
const MAX_BACKGROUND_CARDS: usize = 40;
const BACKGROUND_CARD_SCALE: f32 = 70.0;

#[derive(Component)]
struct FeaturedCard;

#[derive(Component)]
pub struct BackgroundCard {
    speed: f32,
    rotation_speed: Vec3,
}

pub struct BackgroundAnimationPlugin;

impl Plugin for BackgroundAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Game::Loaded), setup_background)
            .add_systems(Update, (animate_featured_card, animate_background_cards, spawn_background_cards).run_if(in_state(Game::Loaded))
    }
}

fn setup_background(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // Spawn featured card
    commands.spawn((
        SceneRoot (assets.deck.cards[0].model.clone()),
        Transform{
            translation: Vec3::ZERO,
            scale: Vec3::splat(BACKGROUND_CARD_SCALE),
            ..default()
        },
        FeaturedCard,
    ));
    commands.spawn((PointLight {
        intensity: 10000.0,
        color: Color::WHITE,
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn animate_featured_card(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<FeaturedCard>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(FEATURED_CARD_ROTATION_SPEED * time.delta_secs()));
        transform.rotate(Quat::from_rotation_x(FEATURED_CARD_ROTATION_SPEED * time.delta_secs()));
        transform.rotate(Quat::from_rotation_z(FEATURED_CARD_ROTATION_SPEED * time.delta_secs()));
    }
}

fn spawn_background_cards(
    mut commands: Commands,
    assets: Res<GameAssets>,
    query: Query<&BackgroundCard>,
) {
    if query.iter().count() < MAX_BACKGROUND_CARDS {
        let mut rng = rand::thread_rng();
        
        let y = rng.gen_range(-10.0..10.0);
        let z = rng.gen_range(-50.0..0.0);
        
        commands.spawn((
            SceneRoot (assets.deck.cards[rng.gen_range(0..52)].model.clone()),
            Transform{
                translation: Vec3::new(-600.0, y, z),
                scale: Vec3::splat(BACKGROUND_CARD_SCALE),
                ..default()
            },
            BackgroundCard {
                speed: rng.gen_range(10.0..30.0),
                rotation_speed: Vec3::new(
                    rng.gen_range(-2.0..2.0),
                    rng.gen_range(-2.0..2.0),
                    rng.gen_range(-2.0..2.0),
                ),
            },
        ));
    }
}

fn animate_background_cards(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &BackgroundCard)>,
) {
    for (entity, mut transform, card) in query.iter_mut() {
        transform.translation.x += card.speed * time.delta_secs();
        transform.rotate_x(card.rotation_speed.x * time.delta_secs());
        transform.rotate_y(card.rotation_speed.y * time.delta_secs());
        transform.rotate_z(card.rotation_speed.z * time.delta_secs());

        // Despawn cards that have moved off screen
        if transform.translation.x > 100.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}


