
use crate::utils::lobby::*;

enum ServerMessage {
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
        bincode::serialize(&self).unwrap()
    }
}
