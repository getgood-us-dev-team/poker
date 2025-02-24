use bevy::prelude::*;
use crate::GameState;
use crate::GameAssets;

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lobby), setup_lobby);
    }
}

#[derive(Component)]
struct LobbyContainer;

fn setup_lobby(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    // A circle that contains the usernames of the players in the lobby
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        ..Default::default()
    }, LobbyContainer));
}