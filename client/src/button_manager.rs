use bevy::prelude::*;

struct ButtonManagerPlugin;

impl Plugin for ButtonManagerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Update, update_buttons)
    }
}

pub struct ButtonAssets {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    // When the button is clicked, this function will be called, and game_state will be set to the value passed into this function.
    pub on_click: dyn fn(ResMut<NextState<GameState>>) -> (),
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
                (button_assets.on_click)(game_state);
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