use bevy::prelude::*;

mod background;
pub use background::*;
mod animator;
pub use animator::*;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BackgroundAnimationPlugin, AnimatorPlugin));
    }
}
