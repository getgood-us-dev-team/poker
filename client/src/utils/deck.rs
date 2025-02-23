use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;
use cards::card::{Value, Suit, Card as CCard};

#[derive(Debug, Clone,)]
pub struct Card {
    pub model: Handle<Scene>,

    pub rank: Value,
    pub suit: Suit,
}

impl Card {
    pub fn new(model: Handle<Scene>, rank: Value, suit: Suit) -> Self {
        Self {
            model,
            rank,
            suit,
        }
    }
    pub fn to_cards_card(&self) -> CCard {
        CCard::new(self.rank, self.suit)
    }
    pub fn to_bytes_card(&self) -> BytesCard {
        BytesCard {
            rank: value_to_string(self.rank),
            suit: suit_to_string(self.suit),
        }
    }
    pub fn from_bytes_card(bytes_card: BytesCard, model: Handle<Scene>) -> Self {
        Self {
            model: model,
            rank: string_to_value(&bytes_card.rank),
            suit: string_to_suit(&bytes_card.suit),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BytesCard {
    pub rank: String,
    pub suit: String,
}

#[derive(Debug, Clone, Resource)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(models: Vec<Handle<Scene>>) -> Self {
        println!("Creating deck with {} models", models.len());
        let mut cards = Vec::with_capacity(52);
        
        for (i, rank) in [Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King].iter().enumerate() {
            for (j, suit) in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter().enumerate() {
                let index = i + (j * 13);
                cards.push(Card::new(models[index].clone(), *rank, *suit));
            }
        }

        println!("Deck created with {} cards", cards.len());
        Self { cards }
    }
    pub fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    pub fn find_model_from_bytes_card(&self, bytes_card: BytesCard) -> Handle<Scene> {
        self.cards.iter().find(|card| card.to_bytes_card() == bytes_card).map(|card| card.model.clone()).unwrap_or_default()
    }
}

fn string_to_value(s: &str) -> Value {
    match s {
        "01" => Value::Ace,
        "02" => Value::Two,
        "03" => Value::Three,
        "04" => Value::Four,
        "05" => Value::Five,
        "06" => Value::Six,
        "07" => Value::Seven,
        "08" => Value::Eight,
        "09" => Value::Nine,
        "10" => Value::Ten,
        "11" => Value::Jack,
        "12" => Value::Queen,
        "13" => Value::King,
        _ => panic!("Invalid value: {}", s),
    }
}

fn string_to_suit(s: &str) -> Suit {
    match s {
        "hearts" => Suit::Hearts,
        "diamonds" => Suit::Diamonds,
        "clubs" => Suit::Clubs,
        "spades" => Suit::Spades,
        _ => panic!("Invalid suit: {}", s),
    }
}


fn value_to_string(value: Value) -> String {
    match value {
        Value::Ace => "01",
        Value::Two => "02",
        Value::Three => "03",
        Value::Four => "04",
        Value::Five => "05",
        Value::Six => "06",
        Value::Seven => "07",
        Value::Eight => "08",
        Value::Nine => "09",
        Value::Ten => "10",
        Value::Jack => "11",
        Value::Queen => "12",
        Value::King => "13",
    }.to_string()
}

fn suit_to_string(suit: Suit) -> String {
    match suit {
        Suit::Hearts => "hearts",
        Suit::Diamonds => "diamonds",
        Suit::Clubs => "clubs",
        Suit::Spades => "spades",
    }.to_string()
}