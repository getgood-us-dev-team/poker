use bevy::prelude::*;
use crate::GameState;
use crate::GameAssets;
use bevy_simple_text_input::*;

pub struct JoinServerPlugin;

impl Plugin for JoinServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::JoinServer), setup_join_server)
            .add_systems(OnExit(GameState::JoinServer), (input_grabber,cleanup_join_server).chain())
            .add_systems(Update, input_listener.run_if(in_state(GameState::JoinServer)));
    }
}

#[derive(Component)]
struct JoinServerContainer;

fn setup_join_server(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    //Text box where the user can enter the server address
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        top: Val::Percent(40.0),
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        justify_items: JustifyItems::Center,
        justify_self: JustifySelf::Center,
        ..Default::default()
    }, 
    JoinServerContainer
    ))
    .with_children(|parent|{
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Auto,
                ..Default::default()
            },
            Text::new("Enter Server Address"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 40.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        ));
        parent.spawn((
            Node{
                position_type: PositionType::Absolute,
                top: Val::Px(50.0),
                left: Val::Px(0.0),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..Default::default()
            },
            BorderColor(Color::WHITE.into()),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
            TextInput,
            TextInputTextFont ( TextFont {
                font: game_assets.font.clone(),
                font_size: 20.0,
                ..Default::default()
            }),
            TextInputPlaceholder{
                value: "Username".to_string(),
                text_font: Some(TextFont {
                    font: game_assets.font.clone(),
                    font_size: 20.0,
                    ..Default::default()
                }),
                text_color: Some(TextColor(Color::WHITE.into())),
                ..Default::default()
            },
        ));
    });
}

fn input_grabber(
    mut game_assets: ResMut<GameAssets>,
    text_input_query: Query<&TextInputValue, With<TextInput>>,
) {
    if let Ok(text_input) = text_input_query.get_single() {
        game_assets.server_address = text_input.0.to_string().parse().unwrap();
    }
}

fn input_listener(
    mut game_assets: ResMut<GameAssets>,
    mut events: EventReader<TextInputSubmitEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        game_assets.server_address = event.value.clone().parse().unwrap();
        game_state.set(GameState::Lobby);
        println!("Server address: {}", game_assets.server_address);
    }
}

fn cleanup_join_server(
    mut commands: Commands,
    query: Query<Entity, With<JoinServerContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
