use bevy::prelude::*;
use crate::CardServer;

struct ButtonManagerPlugin;

impl Plugin for ButtonManagerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, update_buttons)
    }
}

pub enum ButtonAction {
    ChangeState(fn(ResMut<NextState<GameState>>) -> ()),
    ChangeWindow(fn(Query<&mut Window>) -> ()),
    ChangeFPS(fn(ResMut<FramespaceSettings>) -> ()),
    CreateRequest(fn(ResMut<CardServer>) -> ()),
    Other(fn() -> ()),
}

impl ButtonAction {
   fn execute(
        &self,
        game_state: ResMut<NextState<GameState>>,
        window_query: Query<&mut Window>,
        framespace_settings: ResMut<FramespaceSettings>,
        card_server: ResMut<CardServer>
    ){
       match self {
           ButtonAction::ChangeState(f) => f(game_state),
           ButtonAction::ChangeWindow(f) => f(window_query),
           ButtonAction::ChangeFPS(f) => f(framespace_settings),
           ButtonAction::CreateRequest(f) => f(card_server),
           ButtonAction::Other(f) => f(),
       }
   }
}

pub struct ButtonAssets {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    // When the button is clicked, this function will be called, and game_state will be set to the value passed into this function.
    pub on_click: ButtonAction,
}

pub fn spawn_button(
    parent: &mut ChildBuilder,
    top: Val,
    text: &str,
    font: Handle<Font>,
    button_assets: ButtonAssets,
){
    parent.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            top,
            left: Val::Px(0.),
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(button_assets.normal.into()),
        BorderColor(button_assets.normal.into()),
        button_assets,
    )).with_child((
        Text::new(text),
        TextFont {
            font,
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
    ))
}

fn update_buttons(
    commands: &mut Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut framespace_settings: ResMut<FramespaceSettings>,
    mut card_server: ResMut<CardServer>,
    mut window_query: Query<&mut Window>,
    interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &ButtonAssets
    ), (Changed<Interaction>, With<Button>)>,
){
    for (interaction, mut background_color, mut border_color, button_assets) in interaction_query.iter_mut(){
        match *interaction{
            Interaction::Clicked => {
                background_color = button_assets.pressed.into();
                border_color = button_assets.pressed.into();
                match button_assets.on_click{
                    ButtonAction::ChangeState(_) => {
                        (button_assets.on_click)(game_state);
                    }
                    ButtonAction::ChangeWindow(mut window_query) => {
                        
                    }
                    ButtonAction::ChangeFPS(mut framespace_settings) => {
                        framespace_settings.fps = 60;
                    }
                    ButtonAction::CreateRequest(mut card_server) => {
                        card_server.create_request();
                    }
                    ButtonAction::Other() => {
                        println!("Other button action");
                    }
                }
            }
            Interaction::Hovered => {
                background_color = button_assets.hovered.into();
                border_color = button_assets.hovered.into();
            }
            Interaction::None => {
                background_color = button_assets.normal.into();
                border_color = button_assets.normal.into();
            }
        }
    }
}