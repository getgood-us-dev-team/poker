use bevy::prelude::*;
use crate::{Card};
use cards::deck::Deck;

// Library used for networking
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

#[derive(Resource)]
pub struct CardServer {
    deck: Deck,
    players: Vec<Player>,
    current_player: u32,
    action_queue: Vec<Action>,
    server_connection: Option<TcpStream>,
    server_listener: Option<TcpListener>,
}

impl CardServer {
    pub fn new_empty() -> Self {
        Self {
            deck: Deck::new_unshuffled(),
            players: Vec::new(),
            current_player: 0,
            action_queue: Vec::new(),
            server_connection: None,
            server_listener: None,
        }
    }
    pub fn host_game(&mut self) {
        self.players.push(Player::new(0, "Host".to_string(), true));
        self.server_listener = Some(TcpListener::bind("127.0.0.1:8080").unwrap());
        self.server_connection = Some(self.server_listener.as_mut().unwrap().accept().unwrap().0);
        
        // on new connection, add player to players list
    }
}


struct Player {
    id: u32,
    name: String,
    hand: Vec<Card>,
    board: Vec<Card>,
    connection: Option<TcpStream>,
    is_host: bool,
}

impl Player {
    pub fn new(id: u32, name: String, is_host: bool) -> Self {
        Self { id, name, hand: Vec::new(), board: Vec::new(), connection: None, is_host }
    }
}

enum Action {
    Check,
    Call,
    Raise(u32),
    Fold,
}
