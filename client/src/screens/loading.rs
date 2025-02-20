use bevy::prelude::*;
use crate::GameState;
use crate::GameAssets;
use crate::asset_loader::amount_loaded;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::Loading), setup_loading_screen)
            .add_systems(Update, update_loading_screen.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading_screen);
    }
}

#[derive(Component)]
struct LoadingText;

fn setup_loading_screen(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
){
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        ..Default::default()
    }))
    .with_children(|parent|{
        parent.spawn((
            Text::new("Loading..."),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 40.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            LoadingText,
        ));
    });
}

fn update_loading_screen(
    asset: Res<AssetServer>,
    mut text: Query<&mut Text, With<LoadingText>>,
    game_assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<GameState>>,
){
    let (loaded, total) = amount_loaded(asset, game_assets);
    if loaded == total {
        game_state.set(GameState::MainMenu);
    }
    for mut text in text.iter_mut(){
        text.0 = format!("Loading... {}/{}", loaded, total);
    }
}

fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingText>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}