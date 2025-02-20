use bevy::prelude::*;

mod loading;
use loading::LoadingScreenPlugin;
mod main_menu;
use main_menu::MainMenuPlugin;
mod settings;
use settings::SettingsPlugin;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App){
        app.add_plugins((LoadingScreenPlugin, MainMenuPlugin, SettingsPlugin));
    }
}

