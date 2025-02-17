use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy::asset::{LoadedFolder, LoadState};
use crate::deck::Deck;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum GameState {
    #[default]
    NotLoaded,
    MainMenu,
    Settings,
    Game,
}

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum Game {
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Component)]
pub struct LoadingText;

#[derive(Resource)]
pub struct LoadingProgress {
    total_assets: usize,
}

// load the assets plugin type
pub struct AssetsPlugin;


#[derive(Resource)]
pub struct GameSettings {
    pub resolution: WindowResolution,
    pub fps_limit: u32,
    pub window_mode: WindowMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: WindowResolution::new(1920.0, 1080.0),
            fps_limit: 60,
            window_mode: WindowMode::Windowed,
        }
    }
}

// Add new resource for card assets
#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub settings: GameSettings,
    pub deck: Deck,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            deck: Deck::new_empty(),
            settings: GameSettings::default(),
        }
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
           .add_systems(OnEnter(Game::NotLoaded), (load_assets, spawn_loading_screen))
           .add_systems(
               Update, 
               (check_assets,update_loading_screen).chain().run_if(in_state(Game::NotLoaded))
           )
           .add_systems(OnExit(Game::NotLoaded), cleanup_loading_screen);
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let font = asset_server.load("font1.ttf");
    let mut card_meshs: Vec<Handle<Scene>> = Vec::new();
    
    println!("Loading card meshes...");
    for i in 0..4 {
        for value in 1..=13 {
            let scene_name = format!("spade{}.glb#Scene0", value%2+1);
            println!("Loading scene: {}", scene_name);
            card_meshs.push(asset_server.load(&scene_name));
        }
    }

    println!("Loaded {} card meshes", card_meshs.len());
    game_assets.deck = Deck::new(card_meshs);
    game_assets.font = font;
    game_assets.settings = GameSettings::default();
}

pub fn spawn_loading_screen(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Px(0.),
            ..default()
        }))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font: game_assets.font.clone(),
                    font_size: 40.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
                LoadingText,
            ));
        });
}

pub fn update_loading_screen(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Text, With<LoadingText>>,
) {
    let mut loaded = 0;
    let total = 53; // 1 font + 52 card meshes
    
    // Check font
    if asset_server.is_loaded_with_dependencies(&game_assets.font) {
        loaded += 1;
    }

    // Check card scenes
    for card in &game_assets.deck.cards {
        if asset_server.is_loaded_with_dependencies(&card.model) {
            loaded += 1;
        }
    }

    let progress = (loaded as f32 / total as f32) * 100.0;
    
    if let Ok(mut text) = query.get_single_mut() {
        text.0 = format!("Loading... {:.0}%", progress);
    }
}


fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingText>>,
) {
    // Remove the loading screen
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn check_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut next_game_state: ResMut<NextState<Game>>,
    mut next_game: ResMut<NextState<GameState>>,
) {
    let mut all_loaded = true;

    // Check font loading
    if !asset_server.is_loaded_with_dependencies(&game_assets.font) {
        all_loaded = false;
    }

    // Check all card scenes
    for card in &game_assets.deck.cards {
        if !asset_server.is_loaded_with_dependencies(&card.model) {
            all_loaded = false;
            break;
        }
    }
    println!("All assets loaded: {}", all_loaded);

    if all_loaded {
        next_game_state.set(Game::Loaded);
        next_game.set(GameState::MainMenu);
    }
}



