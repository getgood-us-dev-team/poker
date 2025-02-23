use bevy::prelude::*;
use crate::button_manager::{ButtonAssets, spawn_button, ButtonAction};
use crate::GameState;
use crate::GameAssets;
use std::sync::Arc;
use crate::ButtonPosition;
use bevy_simple_text_input::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, input_listener.after(TextInputSystem).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), (input_grabber, cleanup_menu).chain() );        
    }
}

#[derive(Component)]
struct MainMenuContainer;

const PLAY_BUTTON: Color = Color::srgb(0.15, 0.45, 0.15);
const SETTINGS_BUTTON: Color = Color::srgb(0.15, 0.15, 0.45);
const QUIT_BUTTON: Color = Color::srgb(0.45, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const TITLE_FONT_SIZE: f32 = 50.0;
const BUTTON_FONT_SIZE: f32 = 30.0;
const BUTTON_HEIGHT: f32 = 65.0;
const BUTTON_INCREMENT: f32 = 100.0;
const TITLE_TOP: Val = Val::Percent(5.0);
const TITLE_LEFT: Val = Val::Percent(5.0);
const HEIGHT_FROM_TOP: f32 = 160.0;

fn setup_main_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
){
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            top: TITLE_TOP,
            left: TITLE_LEFT,
            ..Default::default()
        },
        MainMenuContainer,
    )).with_children(|parent|{
        parent.spawn((
            Text::new(crate::GAME_NAME),
            TextFont {
                font: game_assets.font.clone(),
                font_size: TITLE_FONT_SIZE,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            TextLayout::new(JustifyText::Right, LineBreak::WordBoundary),
            Node{
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            }
        ));
        spawn_main_menu_buttons(parent, game_assets);
    });
}


fn spawn_main_menu_buttons(
    parent: &mut ChildBuilder,
    game_assets: Res<GameAssets>,
){
    let mut button_height = HEIGHT_FROM_TOP;
    let increment = BUTTON_INCREMENT;
    spawn_button(
        parent,
        "Play",
        game_assets.font.clone(),
        ButtonPosition {
            top: Val::Px(button_height),
            left: Val::Px(0.),
            height: Val::Px(BUTTON_HEIGHT),
            font_size: BUTTON_FONT_SIZE,
            ..Default::default()
        },
        ButtonAssets {
            normal: PLAY_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::ChangeState(
                Arc::new(move |state| {
                    state.set(GameState::Lobby)
                })
            ),
        },
    );
    button_height += increment;
    // Spawn textbox for username
    parent.spawn((
        Node{
            position_type: PositionType::Absolute,
            top: Val::Px(button_height),
            left: Val::Px(0.),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            width: Val::Px(200.0),
            height: Val::Px(BUTTON_HEIGHT),
            ..Default::default()
        },
        BorderColor(PLAY_BUTTON),
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
        TextInput,
        TextInputTextFont ( TextFont {
            font: game_assets.font.clone(),
            font_size: BUTTON_FONT_SIZE+5.0,
            ..Default::default()
        }),
        TextInputPlaceholder{
            value: "Username".to_string(),
            text_font: Some(TextFont {
                font: game_assets.font.clone(),
                font_size: BUTTON_FONT_SIZE-5.0,
                ..Default::default()
            }),
            text_color: Some(TextColor(Color::WHITE.into())),
            ..Default::default()
        },
    ));


    button_height += increment;
    spawn_button(
        parent,
        "Settings",
        game_assets.font.clone(),
        ButtonPosition {
            top: Val::Px(button_height),
            left: Val::Px(0.),
            height: Val::Px(BUTTON_HEIGHT),
            font_size: BUTTON_FONT_SIZE,
            ..Default::default()
        },
        ButtonAssets {
            normal: SETTINGS_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::ChangeState(
                Arc::new(move |state| {
                    state.set(GameState::Settings)
                })
            ),
        },
    );
    button_height += increment;
    #[cfg(not(target_arch = "wasm32"))]
    spawn_button(
        parent,
        "Quit",
        game_assets.font.clone(),
        ButtonPosition {
            top: Val::Px(button_height),
            left: Val::Px(0.),
            height: Val::Px(BUTTON_HEIGHT),
            font_size: BUTTON_FONT_SIZE,
            ..Default::default()
        },
        ButtonAssets {
            normal: QUIT_BUTTON,
            hovered: HOVERED_BUTTON,    
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::Other(
                Arc::new(move || {
                    std::process::exit(0);
                })
            ),
        },
    );  
}

fn input_grabber(
    mut game_assets: ResMut<GameAssets>,
    text_input_query: Query<&TextInputValue, With<TextInput>>,
) {
    if let Ok(text_input) = text_input_query.get_single() {
        game_assets.player_name = text_input.0.to_string();
        println!("Player name: {}", game_assets.player_name);
    }
}

fn input_listener(
    mut game_assets: ResMut<GameAssets>,
    mut events: EventReader<TextInputSubmitEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        game_assets.player_name = event.value.clone();
        game_state.set(GameState::Lobby);
        println!("Player name: {}", game_assets.player_name);
    }
}

fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuContainer>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
