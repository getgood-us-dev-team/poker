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

pub fn send_message_system(mut server: ResMut<RenetServer>) {
    //println!("Sending message");
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}

pub fn receive_message_system(mut server: ResMut<RenetServer>) {
    //println!("Receiving message");
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            // Handle received message
            println!("Received message from client {client_id}: {:?}", message);
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