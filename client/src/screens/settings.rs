use bevy::prelude::*;

struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(OnExit(GameState), cleanup_settings)
    }
}


const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const BACK_BUTTON_NORMAL: Color = Color::rgb(0.4, 0.2, 0.2);
const BACK_BUTTON_HOVERED: Color = Color::rgb(0.5, 0.3, 0.3);
const BACK_BUTTON_PRESSED: Color = Color::rgb(0.3, 0.1, 0.1);





fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}