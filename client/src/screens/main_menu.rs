use bevy::prelude::*;
use crate::button_manager::{ButtonAssets, spawn_button, ButtonAction};
use crate::GameState;
use crate::GameAssets;
use std::sync::Arc;
use crate::ButtonPosition;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
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
                    state.set(GameState::Game)
                })
            ),
        },
    );
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


fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuContainer>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}