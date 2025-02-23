use bevy::prelude::*;
use crate::Deck;

// Implementation of a poker lobby
pub struct Lobby {
    pub players: Vec<Player>,
    pub turn: u8,
    pub deck: Deck,
}

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub money: i32,
    pub position: u8,
}



