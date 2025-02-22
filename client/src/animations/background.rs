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
const FEATURED_CARD_ROTATION_SPEED: f32 = 0.5;
const MAX_BACKGROUND_CARDS: usize = 40;
const BACKGROUND_CARD_SCALE: f32 = 70.0;

#[derive(Component)]
struct FeatureCard;

#[derive(Component)]
struct BackgroundCard;

fn spawn_feature_card(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn((
        FeatureCard,
        Transform{
            translation: Vec3::ZERO,
            scale: Vec3::splat(BACKGROUND_CARD_SCALE),
            ..default()
        },
        SceneRoot(game_assets.deck.cards[0].model.clone()),
        AnimatedObject {
            rotation: Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            scale: Vec3::ZERO,
            translation: Vec3::ZERO,
            speed: 3.0,
        },
    ));
}

fn spawn_background_cards(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    background_cards: Query<Entity, With<BackgroundCard>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..(MAX_BACKGROUND_CARDS - background_cards.iter().count()) {
        let card_index = rng.gen_range(0..game_assets.deck.cards.len());
        let x = -60.0;
        let y = rng.gen_range(-30.0..30.0);
        let z = rng.gen_range(-10.0..10.0);
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
                translation: Vec3::new(rng.gen_range(-30.0..30.0)*BACKGROUND_CARD_SPEED, rng.gen_range(-1.0..1.0), 0.),
                speed: 2.0,
            },
        ));
    }
}

