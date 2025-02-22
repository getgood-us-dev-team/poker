use bevy::prelude::*;
use crate::GameState;
use crate::GameAssets;
use crate::ButtonAction;
use bevy::window::{WindowResolution, WindowMode};
use crate::button_manager::{spawn_button, ButtonAssets};
use std::sync::Arc;
use bevy_framepace::{Limiter, FramepaceSettings};
use crate::ButtonPosition;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(OnEnter(GameState::Settings), setup_settings)
        .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

#[derive(Component)]
struct SettingsContainer;

const NORMAL_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::srgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::srgb(0.1, 0.1, 0.1);
const BACK_BUTTON_NORMAL: Color = Color::srgb(0.4, 0.2, 0.2);
const BACK_BUTTON_HOVERED: Color = Color::srgb(0.5, 0.3, 0.3);
const BACK_BUTTON_PRESSED: Color = Color::srgb(0.3, 0.1, 0.1);
const SETTINGS_TITLE_FONT_SIZE: f32 = 40.0;
const SETTINGS_TITLE_TOP: f32 = 60.0;
const BUTTON_HEIGHT: f32 = 40.0;


fn setup_settings(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
){
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(5.),
            top: Val::Percent(5.),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            width: Val::Px(300.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        SettingsContainer,
    )).with_children(|parent|{

        spawn_button(
            parent,
            "Back",
            game_assets.font.clone(),
            ButtonPosition{
                top: Val::Px(20.),
                left: Val::Auto,
                width: Val::Px(100.0),
                height: Val::Px(30.0),
                ..Default::default()
            },
            ButtonAssets {
                normal: BACK_BUTTON_NORMAL,
                hovered: BACK_BUTTON_HOVERED,
                pressed: BACK_BUTTON_PRESSED,
                on_click: ButtonAction::ChangeState(Arc::new(move |state| {
                    state.set(GameState::MainMenu);
                })),
            },
        );

        parent.spawn((
            Text::new("Settings"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: SETTINGS_TITLE_FONT_SIZE,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(SETTINGS_TITLE_TOP),
                ..Default::default()
            }
        ));
        let mut top = SETTINGS_TITLE_TOP + BUTTON_HEIGHT + 20.0;
        setup_setting_section(
            parent,
            game_assets.font.clone(),
            "Resolution",
            top,
            vec!["3840x2160", "2560x1440", "1920x1080", "1600x900"],
            |resolution: &str| {
                let resolution = resolution.to_string();    
                ButtonAction::ChangeWindow(Arc::new(move |windows: &mut Query<&mut Window>| {
                    let res = resolution.split_once('x').unwrap();
                    let width = res.0.parse::<f32>().unwrap();
                    let height = res.1.parse::<f32>().unwrap();
                    if let Ok(mut window) = windows.get_single_mut() {
                        window.resolution = WindowResolution::new(width, height);
                    }
                }))
            }
        );
        top += (BUTTON_HEIGHT + 20.0) * 4.0 + 50.0;
        setup_setting_section(
            parent,
            game_assets.font.clone(),
            "FPS",
            top,
            vec!["60", "120", "240"],
            move|fps: &str| {
                let fps = Arc::new(fps.to_string());
                ButtonAction::ChangeFPS(Arc::new(move |framespace_settings: &mut ResMut<FramepaceSettings>| {
                    framespace_settings.limiter = Limiter::from_framerate(fps.parse::<f64>().unwrap());
                }))
            }
        );
        top += (BUTTON_HEIGHT + 20.0) * 3.0 + 50.0;
        setup_setting_section(
            parent,
            game_assets.font.clone(),
            "Window Mode",
            top,
            vec!["Windowed", "Borderless", "Fullscreen"],
            move|window_mode: &str| {
                let window_mode = Arc::new(window_mode.to_string());
                ButtonAction::ChangeWindow(Arc::new(move |windows: &mut Query<&mut Window>| {
                    let window_mode = match window_mode.as_str() {
                        "Windowed" => WindowMode::Windowed,
                        "Borderless" => WindowMode::SizedFullscreen(MonitorSelection::Primary),
                        "Fullscreen" => WindowMode::Fullscreen(MonitorSelection::Primary),
                        _ => WindowMode::Windowed,
                    };
                    if let Ok(mut window) = windows.get_single_mut() {
                        window.mode = window_mode;
                    }
                }))
            }
        );


    });
}

fn setup_setting_section(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    title: &str,
    top: f32,
    options: Vec<&str>,
    on_click_generator: impl Fn(&str) -> ButtonAction,
){
    parent.spawn((
        Text::new(title),
        TextFont {
            font: font.clone(),
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(top),
            left: Val::Auto,
            ..Default::default()
        }
    ));
    for (i, item) in options.iter().enumerate(){
        spawn_button(
            parent,
            item,
            font.clone(),
            ButtonPosition {
                top: Val::Px((i+1) as f32 * (BUTTON_HEIGHT + 20.0) + top),
                left: Val::Auto,
                width: Val::Px(180.0),
                font_size: 30.0,
                height: Val::Px(BUTTON_HEIGHT),
            },
            ButtonAssets {
                normal: NORMAL_BUTTON,
                hovered: HOVERED_BUTTON,
                pressed: PRESSED_BUTTON,
                on_click: on_click_generator(item),
            },
        );
    }
   
}


fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}