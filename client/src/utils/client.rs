use bevy_renet::*;
use renet::*;
use renet_netcode::*;
use std::net::{UdpSocket, SocketAddr};
use std::time::SystemTime;
use bevy::prelude::*;
use crate::asset_loader::GameAssets;
use rand::{thread_rng, Rng};
use crate::utils::*;
pub const PROTOCOL_ID: u64 = 12478;

pub fn create_client(mut commands: Commands, game_assets: Res<GameAssets>, mut lobby: ResMut<Lobby>) {
    let client_id = rng.gen_range(0..u64::MAX);
    game_assets.client_id = client_id;
    let client_address = "127.0.0.1:0";
    println!("Creating client connected to server at address: {}, and making socket at address: {}", game_assets.server_address, client_address);
    let client = RenetClient::new(ConnectionConfig::default());
    commands.insert_resource(client);
    let mut rng = thread_rng();
    let authentication = ClientAuthentication::Unsecure {
        server_addr: game_assets.server_address,
        client_id: client_id,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };
    let socket = UdpSocket::bind(client_address).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    println!("Transport created");
    commands.insert_resource(transport);
    lobby = Lobby::new_from_deck(game_assets.deck);

    let player = Player{
        name: game_assets.player_name.clone(),
        client_id: client_id,
        ..Default::default()
    };

    lobby.add_player(player);

    client.send_message(DefaultChannel::ReliableOrdered, player.into());
}

pub fn send_message_system(mut client: ResMut<RenetClient>, lobby: Res<Lobby>, events: EventReader<Action>) {
    for action in events.read() {
        match lobby.play_turn(action) {
            ActionResult::Error(error, error_code) => {
                println!("Error: {:?}", error);
            }
            ActionResult::Success => {
                client.send_message(DefaultChannel::ReliableOrdered, ServerMessage::Action(action).into());
            }
        }
    }
}

pub fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<Lobby>,
    mut events: EventWriter<Action>,
    mut game_state: ResMut<GameState>
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // message could be a player or an action
        let server_message = ServerMessage::from(message);
        match server_message {
            ServerMessage::Player(player) => {
                lobby.add_player(player);
            }
            ServerMessage::Action(action, client_id) => {
                if client_id != game_assets.client_id {
                    events.send(action);
                    lobby.play_turn(action);
                }
            }
            ServerMessage::StartGame => {
                game_state.set(GameState::InGame);
            }
        }
    }
}

