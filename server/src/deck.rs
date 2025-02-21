use bevy::prelude::*;
use rand::seq::SliceRandom;
use cards::card::{Value, Suit, Card as CCard};

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
}

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


