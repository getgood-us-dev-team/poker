use bevy::prelude::*;
use crate::button_manager::{ButtonAssets, spawn_button, ButtonAction};

struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

struct MainMenuContainer;

const PLAY_BUTTON: Color = Color::srgb(0.15, 0.45, 0.15);
const SETTINGS_BUTTON: Color = Color::srgb(0.15, 0.15, 0.45);
const QUIT_BUTTON: Color = Color::srgb(0.45, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_main_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
){
    commands.spawn((
        Node{
            position_type: PositionType::Absolute,
            top: Val::Percent(5.),
            left: Val::Percent(5.),
            ..Default::default()
        },
        MainMenuContainer,
    )).with_children(|parent|{
        parent.spawn((
            Text::new(crate::GAME_NAME),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 40.0,
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
        
    });
}


fn spawn_main_menu_buttons(
    parent: &mut ChildBuilder,
    game_assets: Res<GameAssets>,
){
    spawn_button(
        parent,
        Val::Px(100.),
        "Play",
        game_assets.font.clone(),
        ButtonAssets {
            normal: PLAY_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::ChangeState(
                |state: ResMut<NextState<GameState>>| 
                {state.set(GameState::Game)} 
            ),
        },
    );
    spawn_button(
        parent,
        Val::Px(200.),
        "Settings",
        game_assets.font.clone(),
        ButtonAssets {
            normal: SETTINGS_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::ChangeState(
                |state: ResMut<NextState<GameState>>| 
                {state.set(GameState::Settings)} 
            ),
        },
    );
    #[cfg(not(target_arch = "wasm32"))]
    spawn_button(
        parent,
        Val::Px(300.),
        "Quit",
        game_assets.font.clone(),
        ButtonAssets {
            normal: QUIT_BUTTON,
            hovered: HOVERED_BUTTON,
            pressed: PRESSED_BUTTON,
            on_click: ButtonAction::Other(
                || {
                    std::process::exit(0);
                }
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