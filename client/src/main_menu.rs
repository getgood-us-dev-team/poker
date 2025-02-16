use bevy::prelude::*;
use crate::assets::{GameAssets, GameState};

// Add distinct colors for each button
const PLAY_BUTTON: Color = Color::srgb(0.15, 0.45, 0.15);
const SETTINGS_BUTTON: Color = Color::srgb(0.15, 0.15, 0.45);
const QUIT_BUTTON: Color = Color::srgb(0.45, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
enum ButtonType {
    Play,
    Settings,
    Quit,
}

#[derive(Component)]
struct MainMenuContainer;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), main_menu)
            .add_systems(Update, update_button_colors.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_all_buttons(parent: &mut ChildBuilder, assets: &GameAssets) {
    // Play Button
    spawn_button(parent, "Play", 200., PLAY_BUTTON, assets, ButtonType::Play);
    // Settings Button
    spawn_button(parent, "Settings", 300., SETTINGS_BUTTON, assets, ButtonType::Settings);
    // Quit Button (native only)
    spawn_button(parent, "Quit", 400., QUIT_BUTTON, assets, ButtonType::Quit);
}

#[cfg(target_arch = "wasm32")]
fn spawn_all_buttons(parent: &mut ChildBuilder, assets: &GameAssets) {
    // Play Button
    spawn_button(parent, "Play", 200., PLAY_BUTTON, assets, ButtonType::Play);
    // Settings Button
    spawn_button(parent, "Settings", 300., SETTINGS_BUTTON, assets, ButtonType::Settings);
}

fn main_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((Node {
        position_type: PositionType::Absolute,
        top: Val::Percent(5.),
        left: Val::Percent(5.),
        ..default()
    },
    MainMenuContainer)).with_children(|parent| {
        parent.spawn((
            Text::new("Poker Game"),
            TextFont {
                font: assets.font.clone(),
                font_size: 60.0,
            ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            TextLayout::new(JustifyText::Right, LineBreak::WordBoundary),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..default()
            }
        ));
        
        spawn_all_buttons(parent, &assets);
    });
    commands.insert_resource(GameState::MainMenu);
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, top: f32, color: Color, assets: &GameAssets, button_type: ButtonType) {
    parent.spawn((
        Button,
        button_type,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(top),
            left: Val::Px(0.),
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BorderColor(Color::WHITE.into()),
        BorderRadius::MAX,
        BackgroundColor(color),
    )).with_child((
        Text::new(text),
        TextFont {
            font: assets.font.clone(),
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(Color::WHITE.into()),
    ));
}

fn update_button_colors(
    mut commands: Commands,
    mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &ButtonType,
    ),
    (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>
    ) {
    for (interaction, mut background_color, mut border_color, button_type) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                *border_color = HOVERED_BUTTON.into();
            },
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                *border_color = PRESSED_BUTTON.into();
                match button_type {
                    ButtonType::Play => game_state.set(GameState::Game),
                    ButtonType::Settings => game_state.set(GameState::Settings),
                    ButtonType::Quit => std::process::exit(0),
                }
            },
            Interaction::None => {
                let default_color = match button_type {
                    ButtonType::Play => PLAY_BUTTON,
                    ButtonType::Settings => SETTINGS_BUTTON,
                    ButtonType::Quit => QUIT_BUTTON,
                };
                *background_color = default_color.into();
                *border_color = default_color.into();
            }
        }
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