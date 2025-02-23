use bevy::prelude::*;
use crate::Deck;

// Implementation of a poker lobby

// A lobby is a collection of players, a deck, and a turn
#[derive(Debug, Clone, Copy, Component)]
pub struct Lobby {
    pub players: Vec<Player>,
    pub turn: u8,
    pub deck: Deck,
    pub pot: i32,
    pub current_bet: i32,
}

impl Lobby {
    pub fn new() -> Self {
        Lobby {
            players: Vec::new(),
            turn: 0,
            deck: Deck::new_empty(),
            pot: 0,
            current_bet: 0,
        }
    }

    pub fn new_from_deck(deck: Deck) -> Self {
        Lobby {
            players: Vec::new(),
            turn: 0,
            deck,
            pot: 0,
            current_bet: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
    
    pub fn remove_player_by_id(&mut self, id: u8) {
        self.players.retain(|player| player.client_id != id);
    }

    pub fn play_turn(&mut self, action: Action) -> ActionResult {
        let player = self.players.get_mut(self.turn as usize).unwrap();
        match action {
            Action::Check => {
                if player.bet_this_turn < self.current_bet {
                    return ActionResult::Error("You must call the current bet".to_string(), ActionErrorCode::MustCallCurrentBet);
                }
            }
            Action::Call => {
                if player.money < self.current_bet - player.bet_this_turn {
                    return ActionResult::Error("You don't have enough money to call".to_string(), ActionErrorCode::NotEnoughMoney);
                }
                player.money -= self.current_bet - player.bet_this_turn;
                player.bet_this_turn += self.current_bet;
            }
            Action::Raise(amount) => {
                if player.money < amount {
                    return ActionResult::Error("You don't have enough money to raise".to_string(), ActionErrorCode::NotEnoughMoney);
                }
                if amount + player.bet_this_turn < self.current_bet {
                    return ActionResult::Error("You must raise to the current bet".to_string(), ActionErrorCode::MustRaiseToCurrentBet);
                }
                player.bet_this_turn += amount;
                player.money -= amount;
                self.current_bet = amount;
            }
            Action::Fold => {
                player.is_folded = true;
            }
            Action::AllIn => {
                player.is_all_in = true;
            }
        }
        while self.is_client_turn(self.get_client_id_from_position(self.turn)) {
            self.turn += 1;
        }
        if self.turn >= self.players.len() as u8 {
            self.turn = 0;
        }
        ActionResult::Success
    }
    pub fn is_client_turn(&self, client_id: u64) -> bool {
        self.players.iter().any(|player| player.client_id == client_id && !player.is_folded)
    }
    pub fn get_client_id_from_position(&self, position: u8) -> u64 {
        self.players.iter().find(|player| player.position == position).unwrap().client_id
    }
}

enum ActionResult {
    Success,
    Error(String, ActionErrorCode),
}
enum ActionErrorCode {
    NotEnoughMoney,
    MustCallCurrentBet,
    MustRaiseToCurrentBet,
}

// A player is a collection of a name, a hand, money, and a position
#[derive(Debug, Clone, Copy, Component)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub money: i32,
    pub position: u8,
    pub is_all_in: bool,
    pub is_folded: bool,
    pub bet_this_turn: i32,
    // ID used to identify the player from server to client
    pub client_id: u64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::new(),
            hand: Vec::new(),
            money: 5000,
            position: 0,
            is_all_in: false,
            is_folded: false,
            bet_this_turn: 0,
            client_id: 0,
        }
    }
}

// an action is a collection of a type, a value, and a player
#[derive(Debug, Clone, Copy, Event)]
pub enum Action {
    Check,
    Call,
    Raise(i32),
    Fold,
    AllIn,
}

impl From<Bytes> for Action {
    fn from(value: Bytes) -> Self {
        bincode::deserialize::<Action>(&value).unwrap()
    }
}
impl Into<Bytes> for Action {
    fn into(self) -> Bytes {
        bincode::serialize(&self).unwrap()
    }
}
impl From<Bytes> for Player {
    fn from(value: Bytes) -> Self {
        bincode::deserialize::<Player>(&value).unwrap()
    }
}
impl Into<Bytes> for Player {
    fn into(self) -> Bytes {
        bincode::serialize(&self).unwrap()
    }
}