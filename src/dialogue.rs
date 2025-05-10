use crate::GameState;
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
                padding: UiRect::all(Val::Px(16.0)),
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
                    font_size: 24.0,
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
    gamepads: Query<&Gamepad>,
    current_dialogue: Res<CurrentDialogue>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
            next_game_state.set(GameState::Movement);
        }
    }
}
