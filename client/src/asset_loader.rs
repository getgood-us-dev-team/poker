use bevy::prelude::*;
use crate::Deck;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<GameAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub deck: Deck,
    pub player_name: String,
    pub server_address: SocketAddr,
    pub client_id: u64,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            deck: Deck::new_empty(),
            player_name: String::new(),
            server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2163),
            client_id: 0,
        }
    }
}

fn load_assets(
    asset: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
){
    game_assets.font = asset.load("font1.ttf");

    let mut cards: Vec<Handle<Scene>> = Vec::new();
    /*
    for i in ["spade", "club", "heart", "diamond"]{
        for j in 1..=13 {
            let card = asset.load(format!("{}{}.glb", i, j));
            cards.push(card);
        }
    }
    */
    // For now, I only have 2 cards, the ace of spades and the 2 of spades
    // fills the array all the way to 52 cards using only these 2 cards
    for i in 0..52 {
        cards.push(asset.load(format!("{}{}.glb#Scene0", "spade", i%2+1)));
    }

    game_assets.deck = Deck::new(cards); 
}

pub fn amount_loaded(
    asset: Res<AssetServer>,
    game_assets: Res<GameAssets>,
) -> (i32, i32){
    let mut loaded = 0;
    let mut total = 0;
    if asset.is_loaded_with_dependencies(&game_assets.font){
        loaded += 1;
    }
    total += 1;

    for card in game_assets.deck.cards.iter(){
        total += 1;
        if asset.is_loaded_with_dependencies(&card.model){
            loaded += 1;
        }
    }

    (loaded, total)
}