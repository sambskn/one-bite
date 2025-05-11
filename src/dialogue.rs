use crate::GameState;
use crate::player::Player;
use crate::ui::SterbTime;
use crate::world::{TILE_SIZE, Target, WORLD_HEIGHT, WORLD_WIDTH};
use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentDialogue {
    pub message: String,
    pub effects: Vec<DialogueEffect>,
}

impl CurrentDialogue {
    pub fn empty() -> Self {
        CurrentDialogue {
            message: "".to_string(),
            effects: vec![],
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DialogueEffect {
    GorbEmpty,
    GameOver,
    IncreaseBlood(i32),
    ResetTargetLoc,
    BonusTime,
}

#[derive(Component)]
pub struct Dialogue;

pub fn prepare_dialogue(
    current_dialogue: Res<CurrentDialogue>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let frame_texture = asset_server.load("box_frame.png");
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands.spawn((
        Dialogue,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            height: Val::Vh(100.0),
            width: Val::Vw(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(18.0)),
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ImageNode {
                image: frame_texture.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                ..default()
            },
            children![(
                Node { ..default() },
                Text::new(current_dialogue.message.clone()),
                TextFont {
                    font: asset_server.load("castlevainia3nes.ttf"),
                    font_size: 18.0,
                    ..default()
                },
            ),],
        ),],
    ));
}

pub fn cleanup_dialogue(mut commands: Commands, dialogue_query: Query<Entity, With<Dialogue>>) {
    for dialogue_entity in &dialogue_query {
        commands.entity(dialogue_entity).despawn();
    }
}

pub fn handle_gamepad_input(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<&mut Player>,
    mut target_query: Query<(&mut Target, &mut Transform)>,
    mut sterb_query: Query<&mut SterbTime>,
    gamepads: Query<&Gamepad>,
    current_dialogue: Res<CurrentDialogue>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::South) {
            if current_dialogue.effects.contains(&DialogueEffect::GameOver) {
                next_game_state.set(GameState::MainMenu);
            } else {
                next_game_state.set(GameState::Movement);
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        if current_dialogue.effects.contains(&DialogueEffect::GameOver) {
            next_game_state.set(GameState::MainMenu);
        } else {
            // do whatever
            next_game_state.set(GameState::Movement);
            for effect in current_dialogue.effects.iter() {
                match effect {
                    DialogueEffect::IncreaseBlood(blood_add) => {
                        for mut player in &mut player_query {
                            player.blood_on_your_hands += blood_add;
                        }
                    }
                    DialogueEffect::GorbEmpty => {
                        for mut player in &mut player_query {
                            player.gorb_count = 0;
                        }
                    }
                    DialogueEffect::BonusTime => {
                        for mut sterb in &mut sterb_query {
                            sterb.0.reset();
                        }
                    }
                    DialogueEffect::ResetTargetLoc => {
                        for (mut target, mut transform) in &mut target_query {
                            let psuedo_rng_val_1 = time.elapsed_secs_f64() * 100.0
                                - (time.elapsed_secs_f64() * 100.0).floor();
                            let psuedo_rng_val_2 = time.elapsed_secs_f64() * 1234.0
                                - (time.elapsed_secs_f64() * 1234.0).floor();
                            target.position = Vec2::new(
                                (psuedo_rng_val_1 as f32 * WORLD_WIDTH as f32).floor(),
                                (psuedo_rng_val_2 as f32 * WORLD_HEIGHT as f32).floor(),
                            );
                            transform.translation.x = target.position.x * TILE_SIZE;
                            transform.translation.y = target.position.y * TILE_SIZE;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
