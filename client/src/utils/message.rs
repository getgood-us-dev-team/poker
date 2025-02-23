use renet::Bytes;
use serde::{Deserialize, Serialize};
use crate::utils::lobby::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Player(Player),
    Action(Action, u64),
    StartGame,
}

impl From<Bytes> for ServerMessage {
    fn from(value: Bytes) -> Self {
        bincode::deserialize::<ServerMessage>(&value).unwrap()
    }
}

impl Into<Bytes> for ServerMessage {
    fn into(self) -> Bytes {
        Bytes::copy_from_slice(&bincode::serialize(&self).unwrap())
    }
}
