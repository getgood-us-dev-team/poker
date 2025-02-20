use bevy::prelude::*;
use deck::Deck;

struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App){
        app.init_resource::<GameAssets>()
    }
}

struct GameAssets {
    pub font: Handle<Font>,
    pub deck: Deck,

}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            deck: Deck::new_empty(),
        }
    }
}

fn load_assets(
    asset: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
){
    game_assets.font = asset.load("font1.ttf");

    let mut cards: Vec<Handle<Scene>> = Vec::new();
    for i in ["spade", "club", "heart", "diamond"]{
        for j in 1..=13 {
            let card = asset.load(format!("{}{}.glb", i, j));
            cards.push(card);
        }
    }

    game_assets.deck = Deck::new(cards); 
}

pub fn amount_loaded(
    asset: Res<AssetServer>,
    game_assets: Res<GameAssets>,
) -> (i32, i32){
    let mut loaded = 0;
    let mut total = 0;
    if !asset.is_loaded_with_dependencies(&game_assets.font){
        loaded += 1;
        total += 1;
    }

    for card in game_assets.deck.cards.iter(){
        total += 1;
        if !asset.is_loaded_with_dependencies(&card.model){
            loaded += 1;
        }
    }

    return (loaded, total);
}