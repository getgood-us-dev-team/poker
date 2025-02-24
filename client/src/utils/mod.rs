use bevy::prelude::*;
use crate::state::ServerMode;
mod deck;
pub use deck::*;

mod server;

mod client;

pub mod lobby;
pub use lobby::*;

mod message;
pub use message::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        // Server systems
        app.add_systems(OnEnter(ServerMode::Host), server::create_server)
        .add_systems(Update, (server::send_message_system, server::receive_message_system, server::handle_events_system).run_if(in_state(ServerMode::Host)));


        // Client systems
        app.add_systems(OnEnter(ServerMode::Join), client::create_client)
        .add_systems(Update, (client::send_message_system, client::receive_message_system).run_if(in_state(ServerMode::Join)));
    }
}
