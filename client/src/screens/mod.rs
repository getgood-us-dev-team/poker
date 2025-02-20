use bevy::prelude::*;

mod loading;
use loading::LoadingScreenPlugin;
mod main_menu;
mod settings;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App){
        app.add_plugins((LoadingScreenPlugin,));
    }
}

