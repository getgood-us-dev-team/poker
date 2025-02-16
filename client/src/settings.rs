use bevy::{ prelude::*};
use crate::assets::{GameAssets, GameState};
use bevy::window::{WindowMode, WindowResolution, MonitorSelection};

#[derive(Component)]
struct SettingsContainer;

#[derive(Component)]
struct BackButton;

pub struct SettingsPlugin;

const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const BACK_BUTTON_NORMAL: Color = Color::rgb(0.4, 0.2, 0.2);
const BACK_BUTTON_HOVERED: Color = Color::rgb(0.5, 0.3, 0.3);
const BACK_BUTTON_PRESSED: Color = Color::rgb(0.3, 0.1, 0.1);

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(OnExit(GameState::Settings), cleanup_settings)
            .add_systems(Update, (
                handle_resolution_change,
                handle_fps_change,
                handle_window_mode_change,
                handle_back_button,
                handle_button_colors,
            ).run_if(in_state(GameState::Settings)));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_desktop_settings(parent: &mut ChildBuilder, assets: &GameAssets) {
    // Resolution section
    parent.spawn((
        Text::new("Resolution"),
        TextFont {
            font: assets.font.clone(),
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.),
            left: Val::Px(0.),
            ..default()
        }
    ));

    let resolutions = ["2560x1440", "1920x1080", "1600x900", "1366x768", "1280x720"];
    for (i, resolution) in resolutions.iter().enumerate() {
        spawn_setting_button(parent, resolution, 100. + (i as f32 * 40.), assets);
    }

    // Window Mode section
    parent.spawn((
        Text::new("Display Mode"),
        TextFont {
                font: assets.font.clone(),
            font_size: 20.0,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(520.),
            left: Val::Px(0.),
            ..default()
        }
    ));

    let window_modes = ["Windowed", "Fullscreen", "Borderless"];
    for (i, mode) in window_modes.iter().enumerate() {
        spawn_setting_button(parent, mode, 560. + (i as f32 * 40.), assets);
    }
}

fn setup_settings(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(5.),
                top: Val::Percent(5.),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                width: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            SettingsContainer,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                        font: assets.font.clone(),
                    font_size: 40.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    ..default()
                }
            ));

            // Desktop-only settings
            #[cfg(not(target_arch = "wasm32"))]
            spawn_desktop_settings(parent, &assets);

            // FPS section (available on all platforms)
            parent.spawn((
                Text::new("FPS Limit"),
                TextFont {
                    font: assets.font.clone(),
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE.into()),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(310.),  // Move up when in WASM
                    left: Val::Px(0.),
                    ..default()
                }
            ));

            let fps_options = ["30 FPS", "60 FPS", "120 FPS", "Unlimited"];
            for (i, fps) in fps_options.iter().enumerate() {
                spawn_setting_button(parent, fps, 350. + (i as f32 * 40.), &assets);  // Move up when in WASM
            }

            // Add back button at the end
            spawn_back_button(parent, &assets);
        });
}

fn spawn_setting_button(parent: &mut ChildBuilder, text: &str, top: f32, assets: &GameAssets) {
    parent.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(top),
            left: Val::Px(0.),
            width: Val::Px(200.0),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::WHITE.into()),
        BorderRadius::MAX,
        BackgroundColor(NORMAL_BUTTON),
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                    font: assets.font.clone(),
                font_size: 16.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
        ));
    });
}

fn spawn_back_button(parent: &mut ChildBuilder, assets: &GameAssets) {
    parent.spawn((
        Button,
        BackButton,
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.),
            left: Val::Px(0.),
            width: Val::Px(100.0),
            height: Val::Px(30.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(Color::WHITE.into()),
        BorderRadius::MAX,
        BackgroundColor(BACK_BUTTON_NORMAL),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Back"),
            TextFont {
                font: assets.font.clone(),
                font_size: 16.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
        ));
    });
}

fn handle_resolution_change(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut settings: ResMut<GameAssets>,
    mut windows: Query<&mut Window>,
) {
    for (interaction, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get(children[0]) {
                if let Some(resolution) = text.0.split_once('x') {
                    if let (Ok(width), Ok(height)) = (resolution.0.parse::<f32>(), resolution.1.parse::<f32>()) {
                        settings.settings.resolution = WindowResolution::new(width, height);
                        if let Ok(mut window) = windows.get_single_mut() {
                            window.resolution = settings.settings.resolution.clone();
                        }
                    }
                }
            }
        }
    }
}

fn handle_fps_change(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut settings: ResMut<GameAssets>,
) {
    for (interaction, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get(children[0]) {
                let fps_text = text.0.as_str();
                settings.settings.fps_limit = if fps_text == "Unlimited" {
                    0
                } else {
                    fps_text.split_whitespace().next()
                        .and_then(|n| n.parse().ok())
                        .unwrap_or(60)
                };
            }
        }
    }
}

fn handle_window_mode_change(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut settings: ResMut<GameAssets>,
    mut windows: Query<&mut Window>,
) {
    for (interaction, children) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(text) = text_query.get(children[0]) {
                let mode_text = text.0.as_str();
                let new_mode = match mode_text {
                    "Fullscreen" => WindowMode::Fullscreen(MonitorSelection::Primary),
                    "Borderless" => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    _ => WindowMode::Windowed,
                };
                settings.settings.window_mode = new_mode;
                if let Ok(mut window) = windows.get_single_mut() {
                    window.mode = new_mode;
                }
            }
        }
    }
}

fn handle_back_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::MainMenu);
        }
    }
}

fn handle_button_colors(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, Option<&BackButton>),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, mut color, is_back) in buttons.iter_mut() {
        let (normal, hovered, pressed) = if is_back.is_some() {
            (BACK_BUTTON_NORMAL, BACK_BUTTON_HOVERED, BACK_BUTTON_PRESSED)
        } else {
            (NORMAL_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON)
        };

        *color = match *interaction {
            Interaction::Pressed => pressed.into(),
            Interaction::Hovered => hovered.into(),
            Interaction::None => normal.into(),
        };
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


