use bevy::prelude::*;
use bevy_renet::*;
use renet::*;
use local_ip_address::local_ip;
use public_ip::*;
use std::default;
use std::net::{UdpSocket, SocketAddr};
use renet_netcode::*;
use std::time::SystemTime;
use crate::asset_loader::GameAssets;
use crate::utils::client::PROTOCOL_ID;
use crate::utils::message::ServerMessage;
use crate::utils::*;


pub fn create_server(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    println!("Creating server at address: {}", game_assets.server_address);
    let server = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);
    let server_address: SocketAddr = game_assets.server_address;
    let socket = UdpSocket::bind(server_address).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_address],
        authentication: ServerAuthentication::Unsecure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);
}

pub fn send_message_system(mut server: ResMut<RenetServer>, mut lobby: ResMut<Lobby>, mut events: EventReader<Action>, game_assets: Res<GameAssets>) {
    if lobby.is_client_turn(game_assets.client_id) {
        for action in events.read() {
            server.broadcast_message(
                DefaultChannel::ReliableOrdered, 
                Into::<Bytes>::into(ServerMessage::Action(*action, game_assets.client_id))
            );
        }
    }
}

pub fn receive_message_system(mut server: ResMut<RenetServer>, mut lobby: ResMut<Lobby>) {
    for client_id in server.clients_id() {
        if lobby.is_client_turn(client_id) {
            while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                let client_message = ServerMessage::from(message.clone());
                match client_message {
                    ServerMessage::Action(action, _) => {
                        lobby.play_turn(action);
                    }
                    _ => {
                        println!("Unknown message from client {client_id}: {:?}", message);
                        break;
                    }
                }
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}

pub fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    //println!("Handling events");
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            },
            _ => {
                println!("Unknown event: {:?}", event);
            }
        }
    }
}