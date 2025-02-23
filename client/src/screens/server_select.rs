use bevy::prelude::*;
use std::sync::Arc;
use crate::{GameState, ServerMode, ButtonAction, ButtonAssets, GameAssets};

pub struct ServerSelectPlugin;

impl Plugin for ServerSelectPlugin {
    fn build(&self, app: &mut App) {
        println!("ServerSelectPlugin building");
        app.add_systems(OnEnter(GameState::ServerSelect), setup_server_select)
            .add_systems(OnExit(GameState::ServerSelect), cleanup_server_select)
            .add_systems(OnEnter(ServerMode::None), change_game_state);
    }
}

#[derive(Component)]
struct ServerSelectContainer;

fn setup_server_select(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    println!("ServerSelectPlugin spawning");
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        top: Val::Percent(50.0),
        left: Val::Percent(50.0),
        ..Default::default()
    }, ServerSelectContainer
    )).with_children(|parent|{
        parent.spawn((
            Text::new("Server Select"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 40.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        ));
        parent.spawn((
            Button,
            TextColor(Color::WHITE.into()),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 20.0,
                ..Default::default()
            },
            Text::new("Host Server"),
            ButtonAssets {
                normal: Color::srgb(0.5, 0.5, 0.5),
                hovered: Color::srgb(0.5, 0.5, 0.5),
                pressed: Color::srgb(0.3, 0.3, 0.3),
                on_click: ButtonAction::ChangeServerMode(Arc::new(|server_mode| {
                    server_mode.set(ServerMode::Host);
                })),
            },
        ));
        parent.spawn((
            Button,
            TextColor(Color::WHITE.into()),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 20.0,
                ..Default::default()
            },
            Text::new("Join Server"),
            ButtonAssets {
                normal: Color::srgb(0.5, 0.5, 0.5),
                hovered: Color::srgb(0.5, 0.5, 0.5),
                pressed: Color::srgb(0.3, 0.3, 0.3),
                on_click: ButtonAction::ChangeServerMode(Arc::new(|server_mode| {
                    server_mode.set(ServerMode::Join);
                })),
            },
        ));
    });
}

fn change_game_state(
    mut game_state: ResMut<NextState<GameState>>,
    server_mode: Res<State<ServerMode>>,
) {
    match server_mode.get() {
        ServerMode::Host => {
            game_state.set(GameState::Lobby);
        }
        ServerMode::Join => {
            game_state.set(GameState::JoinServer);
        }
        _ => {
            panic!("Invalid server mode");
        }
    }
}

fn cleanup_server_select(
    mut commands: Commands,
    query: Query<Entity, With<ServerSelectContainer>>,
) {
    println!("ServerSelectPlugin cleaning up");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
