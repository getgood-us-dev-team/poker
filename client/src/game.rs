use bevy::prelude::*;
use bevy::input::keyboard::{KeyboardInput, KeyCode};
use crate::assets::{GameState, GameAssets};
use crate::background_animation::{BackgroundAnimationPlugin, BackgroundCard};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::io::{Read, Write};
use bevy_simple_text_input::*;
use url::Url;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameSetupState {
    #[default]
    None,
    Create,
    Join,
    Connecting,
    Connected,
    Error,
}

#[derive(Component)]
struct GameSetupUI;

#[derive(Component)]
struct IPInput;

#[derive(Component)]
struct IPInputBox;

#[derive(Resource)]
struct ServerInfo {
    ip: String,
    port: u16,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            ip: "127.0.0.1".to_string(),
            port: 7878,
        }
    }
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameRoomState {
    #[default]
    None,
    Hosting,
    Joining,
    Playing,
}

#[derive(Component)]
struct PlayerSpot {
    position: usize,
    occupied: bool,
    player_name: Option<String>,
}

#[derive(Resource)]
struct GameRoom {
    host: bool,
    players: Vec<String>,
    max_players: usize,
}

impl Default for GameRoom {
    fn default() -> Self {
        Self {
            host: false,
            players: Vec::new(),
            max_players: 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PlayerJoinMessage {
    name: String,
    position: usize,
}

#[derive(Component)]
struct PlayerName;

#[derive(Component)]
struct NetworkConnection(TcpStream);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameSetupState>()
            .init_state::<GameRoomState>()
            .init_resource::<ServerInfo>()
            .init_resource::<GameRoom>()
            .add_systems(OnEnter(GameState::Game), setup_game_choice)
            .add_systems(OnExit(GameState::Game), cleanup_game)
            .add_systems(OnEnter(GameSetupState::Create), (cleanup_game, cleanup_background, create_server, setup_game_room))
            .add_systems(OnEnter(GameRoomState::Playing), cleanup_background)
            .add_systems(OnEnter(GameSetupState::Join), (cleanup_game, setup_join_screen))
            .add_systems(Update, (
                handle_game_buttons,
                handle_button_colors,
                handle_join_input.run_if(in_state(GameSetupState::Join)),
                update_player_spots.run_if(in_state(GameRoomState::Hosting)),
                handle_network_messages.run_if(in_state(GameRoomState::Playing)),
            ));
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.45);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.55);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.65);
const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.5, 0.5, 1.0);
const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const TEXT_COLOR: Color = Color::WHITE;

fn setup_game_choice(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut game_state: ResMut<NextState<GameSetupState>>,
) {
    game_state.set(GameSetupState::None);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
        GameSetupUI,
    ))
    .with_children(|parent| {
        spawn_button(parent, "Create Game", &assets);
        spawn_button(parent, "Join Game", &assets);
    });
}

fn create_server(
    mut commands: Commands,
    assets: Res<GameAssets>,
    server_info: Res<ServerInfo>,
) {
    // Clone the values we need for the thread
    let ip = server_info.ip.clone();
    let port = server_info.port;

    // Spawn server info display
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameSetupUI,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(format!("Server running at: {}:{}", ip, port)),
            TextFont {
                font: assets.font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });

    // Start server in a separate thread
    thread::spawn(move || {
        let listener = TcpListener::bind(format!("{}:{}", ip, port))
            .expect("Failed to bind to address");
        
        println!("Server listening on {}:{}", ip, port);
        
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    
                    // Read player join message
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(n) => {
                            let msg_str = String::from_utf8_lossy(&buffer[..n]);
                            if let Ok(msg) = serde_json::from_str::<PlayerJoinMessage>(&msg_str) {
                                println!("Player joined: {}", msg.name);
                                // Here you would update the game room state
                            }
                        }
                        Err(e) => eprintln!("Error reading from connection: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    });
}

fn setup_join_screen(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameSetupUI,
    ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("Enter server IP:port"),
            TextFont {
                font: assets.font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        // Input box using the plugin
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                border: UiRect::all(Val::Px(5.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(BORDER_COLOR_ACTIVE),
            BackgroundColor(BACKGROUND_COLOR),
            TextInput,
            TextInputTextFont(TextFont {
                font_size: 34.,
                ..default()
            }),
            TextInputTextColor(TextColor(TEXT_COLOR)),
            IPInput,
        ));

        spawn_button(parent, "Connect", &assets);
    });
}

fn handle_join_input(
    mut commands: Commands,
    query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    ip_query: Query<&TextInputValue, With<IPInput>>,
    mut game_state: ResMut<NextState<GameSetupState>>,
    mut game_room_state: ResMut<NextState<GameRoomState>>,
    mut game_room: ResMut<GameRoom>,
    assets: Res<GameAssets>,
    server_info: Res<ServerInfo>,
) {
    for (interaction, children) in query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get(children[0]) {
                if text.0 == "Connect" {
                    if let Ok(ip_text) = ip_query.get_single() {
                        let addr = ip_text.0.trim();
                        let connection_string = if addr.starts_with("http") {
                            match Url::parse(addr) {
                                Ok(url) => format!("{}:{}", url.host_str().unwrap_or("127.0.0.1"), url.port().unwrap_or(7878)),
                                Err(_) => addr.to_string(),
                            }
                        } else {
                            addr.to_string()
                        };

                        match TcpStream::connect(&connection_string) {
                            Ok(stream) => {
                                println!("Connected to server!");
                                
                                let msg = PlayerJoinMessage {
                                    name: "Player".to_string(),
                                    position: game_room.players.len(),
                                };
                                
                                let msg_str = serde_json::to_string(&msg).unwrap();
                                stream.try_clone().unwrap().write_all(msg_str.as_bytes()).unwrap();
                                
                                commands.spawn(NetworkConnection(stream));
                                game_room.players.push(msg.name);
                                
                                game_state.set(GameSetupState::Connected);
                                game_room_state.set(GameRoomState::Playing);
                                
                                setup_game_room_ui(&mut commands, &game_room, &assets, &server_info);
                            }
                            Err(e) => {
                                println!("Failed to connect: {}", e);
                                game_state.set(GameSetupState::Error);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_game_buttons(
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>)
    >,
    text_query: Query<&Text>,
    mut game_state: ResMut<NextState<GameSetupState>>,
) {
    for (interaction, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get(children[0]) {
                match text.0.as_str() {
                    "Create Game" => game_state.set(GameSetupState::Create),
                    "Join Game" => game_state.set(GameSetupState::Join),
                    _ => {}
                }
            }
        }
    }
}

fn handle_button_colors(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, With<GameSetupUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_background(
    mut commands: Commands,
    query: Query<Entity, With<BackgroundCard>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_game_room(
    mut commands: Commands,
    mut game_room: ResMut<GameRoom>,
    mut game_room_state: ResMut<NextState<GameRoomState>>,
    assets: Res<GameAssets>,
    server_info: Res<ServerInfo>,
) {
    game_room.host = true;
    game_room_state.set(GameRoomState::Hosting);

    // Create player spots in a circle
    let center = Vec2::new(0.0, 0.0);
    let radius = 300.0;
    let player_count = game_room.max_players;

    for i in 0..player_count {
        let angle = (i as f32 / player_count as f32) * std::f32::consts::TAU;
        let position = center + Vec2::new(angle.cos(), angle.sin()) * radius;

        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x + 960.0),
                top: Val::Px(position.y + 540.0),
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
            PlayerSpot {
                position: i,
                occupied: false,
                player_name: None,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Empty"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::rgb(0.7, 0.7, 0.7)),
                PlayerName,
            ));
        });
    }

    // Add host to first position
    game_room.players.push("Host".to_string());
    
    // Display server info
    commands.spawn((
        Text::new(format!("Server IP: {}:{}", server_info.ip, server_info.port)),
        TextFont {
            font: assets.font.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn update_player_spots(
    mut spot_query: Query<(&mut BackgroundColor, &PlayerSpot, &Children)>,
    mut text_query: Query<&mut Text, With<PlayerName>>,
    game_room: Res<GameRoom>,
) {
    for (mut color, spot, children) in spot_query.iter_mut() {
        let occupied = spot.position < game_room.players.len();
        *color = if occupied {
            Color::rgb(0.4, 0.6, 0.4).into()
        } else {
            Color::rgb(0.2, 0.2, 0.2).into()
        };

        // Update player name text
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            text.0 = if occupied {
                game_room.players[spot.position].clone()
            } else {
                "Empty".to_string()
            };
        }
    }
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, assets: &GameAssets) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                font: assets.font.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

fn handle_network_messages(
    mut commands: Commands,
    connection_query: Query<(Entity, &NetworkConnection)>,
    mut game_room: ResMut<GameRoom>,
) {
    for (entity, connection) in connection_query.iter() {
        let mut buffer = [0; 1024];
        match connection.0.try_clone().unwrap().read(&mut buffer) {
            Ok(n) if n > 0 => {
                let msg_str = String::from_utf8_lossy(&buffer[..n]);
                if let Ok(msg) = serde_json::from_str::<PlayerJoinMessage>(&msg_str) {
                    println!("Player joined: {}", msg.name);
                    game_room.players.push(msg.name);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data available, continue
            }
            Err(_) => {
                // Connection lost, cleanup
                commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}

fn setup_game_room_ui(
    commands: &mut Commands,
    game_room: &GameRoom,
    assets: &GameAssets,
    server_info: &ServerInfo,
) {
    // Move the UI setup code here from setup_game_room
}
