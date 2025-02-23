use bevy_renet::*;
use renet::*;
use renet_netcode::*;
use std::net::{UdpSocket, SocketAddr};
use std::time::SystemTime;
use bevy::prelude::*;
use crate::asset_loader::GameAssets;
use rand::{thread_rng, Rng};

pub const PROTOCOL_ID: u64 = 12478;

pub fn create_client(mut commands: Commands, game_assets: Res<GameAssets>) {
    let client_address = "127.0.0.1:0";
    println!("Creating client connected to server at address: {}, and making socket at address: {}", game_assets.server_address, client_address);
    let client = RenetClient::new(ConnectionConfig::default());
    commands.insert_resource(client);
    let mut rng = thread_rng();
    let authentication = ClientAuthentication::Unsecure {
        server_addr: game_assets.server_address,
        client_id: rng.gen_range(0..u64::MAX),
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };
    let socket = UdpSocket::bind(client_address).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    println!("Transport created");
    commands.insert_resource(transport);
}

pub fn send_message_system(mut client: ResMut<RenetClient>) {
    //println!("Sending message");
    // Send a text message to the server
    client.send_message(DefaultChannel::ReliableOrdered, "server message");
}

pub fn receive_message_system(mut client: ResMut<RenetClient>) {
    //println!("Receiving message");
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
        println!("Received message from server: {:?}", message);
    }
}