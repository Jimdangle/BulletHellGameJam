use bevy::prelude::*;
use crate::game::ScoreBoard;

use super::GameState;


pub fn menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(Update, (button_system).run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Sorry!".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                game_state.set(GameState::Game)
            }
            Interaction::Hovered => {
                text.sections[0].value = "Play".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Survive".to_string();
                *color = Color::SEA_GREEN.into();
                border_color.0 = Color::GOLD;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<ScoreBoard>) {

    let elapsed_secs = score.get_game_time();
    let minutes = elapsed_secs as u64 / 60;
    let seconds = elapsed_secs as u64 % 60;
    
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart, // Align items to the start (top) of the container
            align_items: AlignItems::FlexStart, // Align items to the start (top) of the container
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|button_parent| {
                button_parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Percent(33.3),
                            height: Val::Px(165.0),
                            position_type: PositionType::Absolute,
                            top: Val::Percent(70.),
                            left: Val::Percent(33.3),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .with_children(|button_text_parent| {
                        button_text_parent.spawn(TextBundle::from_section(
                            "Button",
                            TextStyle {
                                font: asset_server.load("fonts/EvilEmpire.otf"),
                                font_size: 60.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            });

        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Px(20.0), // Add top padding here
                    left: Val::Percent(2.),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|text_parent| {
            text_parent.spawn(TextBundle::from_section(
                "WASD to Move\nSpace to shoot\nE for Special\nHold Shift to boost\nUp and Down Arrow for Music Volume\nNo Objectives just Survive and Score!\n\nPowerups Spawn when a wave is cleared\nBlue = Bullets,\nGreen = Shapes,\nRed = Health",
                TextStyle {
                    font: asset_server.load("fonts/EvilEmpire.otf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                
            ));
        })
        .with_children(|text_parent| {
            text_parent.spawn(TextBundle::from_section(
                format!("Last Score: {}\nTime: {}:{}\nWaves: {}", score.get_score(), minutes, seconds , score.get_waves()),
                TextStyle {
                    font: asset_server.load("fonts/EvilEmpire.otf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                    
                },
                
            ));
        });
    });

     /*
     |parent| {
            parent.spawn(
                TextBundle::from_section(
                 "WASD to Move", 
                TextStyle {
                    font: asset_server.load("fonts/Swamp-Witch.ttf"),
                    font_size: 60.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                
                )
            );
        }
      */
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Node>>, cams: Query<Entity, With<Camera>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    for ent in &cams {
        commands.entity(ent).despawn();
    }
}
