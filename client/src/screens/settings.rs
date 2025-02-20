use bevy::ecs::component;
use bevy::prelude::*;
use crate::GameState;
use crate::GameAssets;
use crate::ButtonAction;
use bevy::window::WindowResolution;
use crate::button_manager::{spawn_button, ButtonAssets};
use std::sync::Arc;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App){
        app
        .add_systems(OnExit(GameState::Settings), cleanup_settings);
    }
}

#[derive(Component)]
struct SettingsContainer;

const NORMAL_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const BACK_BUTTON_NORMAL: Color = Color::rgb(0.4, 0.2, 0.2);
const BACK_BUTTON_HOVERED: Color = Color::rgb(0.5, 0.3, 0.3);
const BACK_BUTTON_PRESSED: Color = Color::rgb(0.3, 0.1, 0.1);

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
        parent.spawn((
            Text::new("Settings"),
            TextFont {
                font: game_assets.font.clone(),
                font_size: 40.0,
                ..Default::default()
            },
            TextColor(Color::WHITE.into()),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            }
        ));

        setup_setting_section(
            parent,
            game_assets.font.clone(),
            "Resolution",
            Val::Px(50.0),
            vec!["2560x1440", "1920x1080", "1600x900", "1366x768", "1280x720"],
            move|resolution: &str| {
                let resolution = Arc::new(resolution.to_string());
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

       
        
    });
}

fn setup_setting_section(
    parent: &mut ChildBuilder,
    font: Handle<Font>,
    title: &str,
    top: Val,
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
        Node {
            position_type: PositionType::Absolute,
            top,
            left: Val::Px(0.),
            ..Default::default()
        }
    )).with_children(|parent|{
        for (i, item) in options.iter().enumerate(){
            spawn_button(
                parent,
                Val::Px((i as f32) * 40.0),
                item,
                font.clone(),
                ButtonAssets {
                    normal: NORMAL_BUTTON,
                    hovered: HOVERED_BUTTON,
                    pressed: PRESSED_BUTTON,
                    on_click: on_click_generator(item),
                },
            );
        }
    });
   
}


fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsContainer>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}