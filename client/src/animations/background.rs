use bevy::prelude::*;
use crate::GameAssets;
use crate::GameState;
use crate::animations::*;
use rand::Rng;
pub struct BackgroundAnimationPlugin;

impl Plugin for BackgroundAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_feature_card)
            .add_systems(Update, spawn_background_cards.run_if(
                in_state(GameState::MainMenu)
                    .or(in_state(GameState::Settings))
            )
        );
    }
}

const BACKGROUND_CARD_SPEED: f32 = 100.0;
const FEATURED_CARD_ROTATION_SPEED: f32 = 1.0;
const MAX_BACKGROUND_CARDS: usize = 40;
const BACKGROUND_CARD_SCALE: f32 = 70.0;

#[derive(Component)]
struct FeatureCard;

#[derive(Component)]
struct BackgroundCard;

fn seeded_random_rotation(transform: &Transform, time: &Time) -> Vec3 {
    let mut rng = rand::thread_rng();
    let y = transform.rotation.y;
    let x = transform.rotation.x;
    let z = transform.rotation.z;
    Vec3::new(
        rng.gen_range(x-1.0..x+1.0) * time.delta_secs(),
        rng.gen_range(y-1.0..y+1.0) * time.delta_secs(),
        rng.gen_range(z-1.0..z+1.0) * time.delta_secs()
    )
}

fn spawn_feature_card(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn((
        FeatureCard,
        Transform{
            translation: Vec3::new(0., 0., -3.),
            scale: Vec3::splat(BACKGROUND_CARD_SCALE),
            ..default()
        },
        SceneRoot(game_assets.deck.cards[0].model.clone()),
        AnimatedObject {
            rotation: Vec3::new(rng.gen_range(0.0..1.5), rng.gen_range(0.0..1.5), rng.gen_range(0.0..1.5)),
            scale: Vec3::ZERO,
            translation: Vec3::ZERO,
            speed: FEATURED_CARD_ROTATION_SPEED,
            update_rotation: seeded_random_rotation,
            ..default()
        },
    ));
}

fn spawn_background_cards(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    background_cards: Query<Entity, With<BackgroundCard>>,
) {
    let mut rng = rand::thread_rng();
    let amount_to_spawn = MAX_BACKGROUND_CARDS - background_cards.iter().count();
    let random_amount_to_spawn = rng.gen_range(0..amount_to_spawn) as usize;
    for _ in 0..random_amount_to_spawn {
        let card_index = rng.gen_range(0..game_assets.deck.cards.len());
        let x = -59.9; // Offscreen is 60.0
        let y = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-30.0..-20.0);
        commands.spawn((
            BackgroundCard,
            Transform{
                translation: Vec3::new(x, y, z),
                scale: Vec3::splat(BACKGROUND_CARD_SCALE),
                ..default()
            },
            SceneRoot(game_assets.deck.cards[card_index].model.clone()),
            AnimatedObject {
                rotation: Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
                scale: Vec3::ZERO,
                translation: Vec3::new(rng.gen_range(2.0..4.0), 0., 0.),
                speed: 2.0,
                ..default()
            },
        ));
    }
}

