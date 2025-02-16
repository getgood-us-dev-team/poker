use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Resource)]
pub enum GameState {
    #[default]
    MainMenu,
    Settings,
    Game,
}


// load the assets plugin type
pub struct AssetsPlugin;


#[derive(Resource)]
pub struct GameSettings {
    pub resolution: WindowResolution,
    pub fps_limit: u32,
    pub window_mode: WindowMode,
    pub ui_scale: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: WindowResolution::new(1920.0, 1080.0),
            fps_limit: 60,
            window_mode: WindowMode::Windowed,
            ui_scale: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct GameAssets{
    pub font: Handle<Font>,
    pub card_back: Handle<Image>,
    pub card_texture: Handle<Image>,
    pub card_sprites: Handle<TextureAtlasLayout>,
    pub settings: GameSettings,
}

impl Default for GameAssets{
    fn default() -> Self {
        Self{
            font: Handle::default(),
            card_back: Handle::default(),
            card_texture: Handle::default(),
            card_sprites: Handle::default(),
            settings: GameSettings::default(),
        }
    }
}
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
           .add_systems(Startup, load_assets);
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut game_assets: ResMut<GameAssets>,
) {
    // Load font
    let font = asset_server.load("../fonts/font1.ttf");
    
    // Load card back (single image)
    let card_back = asset_server.load("../img/card-back.png");
    
    // Load card sprite sheet
    let card_texture: Handle<Image> = asset_server.load("../img/card-icon-sheet.png");
    let card_atlas = TextureAtlasLayout::from_grid(
        UVec2::new(18, 22),  // Card dimensions
        13,  // Columns (Ace through King)
        4,   // Rows (Suits)
        Some(UVec2::new(0, 0)), // No padding
        None,
    );
    let card_sprites = texture_atlases.add(card_atlas);

    
    game_assets.font = font;
    game_assets.card_back = card_back;
    game_assets.card_texture = card_texture;
    game_assets.card_sprites = card_sprites;
    game_assets.settings = GameSettings::default();
}





