use bevy::prelude::*;
use crate::{GameState, ServerMode};
use bevy_framepace::FramepaceSettings;
use std::sync::Arc;

pub struct ButtonManagerPlugin;

impl Plugin for ButtonManagerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, update_buttons);
    }
}


#[derive(Resource, Clone)]
pub enum ButtonAction {
    ChangeState(Arc<dyn Fn(&mut ResMut<NextState<GameState>>) + Send + Sync>),
    ChangeWindow(Arc<dyn Fn(&mut Query<&mut Window>) + Send + Sync>),
    ChangeFPS(Arc<dyn Fn(&mut ResMut<FramepaceSettings>) + Send + Sync>),
    //CreateRequest(Arc<dyn Fn(&mut ResMut<CardServer>) + Send + Sync>),
    Other(Arc<dyn Fn() + Send + Sync>),
    ChangeServerMode(Arc<dyn Fn(&mut ResMut<NextState<ServerMode>>) + Send + Sync>),
}

impl ButtonAction {
    fn execute(
        &self,
        game_state: &mut ResMut<NextState<GameState>>,
        window_query: &mut Query<&mut Window>,
        framespace_settings: &mut ResMut<FramepaceSettings>,
       // card_server: &mut ResMut<CardServer>,
        server_mode: &mut ResMut<NextState<ServerMode>>,
    ) {
        match self {
            ButtonAction::ChangeState(f) => f(game_state),
            ButtonAction::ChangeWindow(f) => f(window_query),
            ButtonAction::ChangeFPS(f) => f(framespace_settings),
            ButtonAction::ChangeServerMode(f) => f(server_mode),
            //ButtonAction::CreateRequest(f) => f(card_server),
            ButtonAction::Other(f) => f(),
        }
    }
}

#[derive(Component)]
pub struct ButtonAssets {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    // When the button is clicked, this function will be called, and game_state will be set to the value passed into this function.
    pub on_click: ButtonAction,
}

pub struct ButtonPosition {
    pub top: Val,
    pub left: Val,
    pub right: Val,
    pub bottom: Val,
    pub width: Val,
    pub height: Val,
    pub font_size: f32,
}

impl Default for ButtonPosition {
    fn default() -> Self {
        ButtonPosition {
            top: Val::Auto,
            left: Val::Auto,
            right: Val::Auto,
            bottom: Val::Auto,
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            font_size: 30.0,
        }
    }
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    text: &str,
    font: Handle<Font>,
    button_position: ButtonPosition,
    button_assets: ButtonAssets,
){
    parent.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            top: button_position.top,
            left: button_position.left,
            right: button_position.right,
            bottom: button_position.bottom,
            width: button_position.width,
            height: button_position.height,
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(button_assets.normal.into()),
        BorderRadius::MAX,
        BorderColor(button_assets.normal.into()),
        button_assets,
    )).with_child((
        Text::new(text),
        TextFont {
            font,
            font_size: button_position.font_size,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
    ));
}

fn update_buttons(
    mut game_state: ResMut<NextState<GameState>>,
    mut framespace_settings: ResMut<FramepaceSettings>,
    mut server_mode: ResMut<NextState<ServerMode>>,
    //mut card_server: ResMut<CardServer>,
    mut window_query: Query<&mut Window>,
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &ButtonAssets
    ), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color, mut border_color, button_assets) in interaction_query.iter_mut(){
        match *interaction{
            Interaction::Pressed => {
                *background_color = BackgroundColor(button_assets.pressed);
                *border_color = BorderColor(button_assets.pressed);
                button_assets.on_click.execute(&mut game_state, &mut window_query, &mut framespace_settings, &mut server_mode);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(button_assets.hovered);
                *border_color = BorderColor(button_assets.hovered);
            }
            Interaction::None => {
                *background_color = BackgroundColor(button_assets.normal);
                *border_color = BorderColor(button_assets.normal);
            }
        }
    }
}