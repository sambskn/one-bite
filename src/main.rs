use bevy::{prelude::*, window::WindowResolution};
use bevy_rand::prelude::*;
use dialogue::{CurrentDialogue, DialogueEffect};
use floating_text::{NewText, handle_new_text_event, update_floating_text};
use player::{DEFAULT_GORB_COUNT, Player};
use std::f32::consts::PI;
use ui::{Arrow, SECONDS_IN_TIMER, SterbTime};
use world::{
    GorbHole, TILE_SIZE, Target, WORLD_HEIGHT, WORLD_WIDTH, WorldContent, WorldMap, generate_world,
};
mod dialogue;
mod floating_text;
mod menu;
mod player;
mod ui;
mod world;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Dialogue,
    Movement,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600.0, 500.0),
                title: "gorb transporter".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_event::<NewText>()
        .insert_resource(ClearColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)))
        .insert_resource(CurrentDialogue::empty())
        .insert_state(GameState::MainMenu)
        .add_systems(Startup, menu::menu_setup)
        .add_systems(OnEnter(GameState::MainMenu), menu::menu_setup)
        .add_systems(
            Update,
            handle_gamepad_input_menu.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), menu::clear_menu_content)
        .add_systems(OnExit(GameState::MainMenu), setup)
        .add_systems(OnExit(GameState::MainMenu), generate_world)
        .add_systems(OnExit(GameState::MainMenu), ui::setup_ui)
        .add_systems(OnEnter(GameState::Dialogue), dialogue::prepare_dialogue)
        .add_systems(OnExit(GameState::Dialogue), dialogue::cleanup_dialogue)
        .add_systems(OnEnter(GameState::MainMenu), world::clear_world)
        .add_systems(
            Update,
            camera_transform_update.run_if(in_state(GameState::Movement)),
        )
        .add_systems(Update, arrow_update.run_if(in_state(GameState::Movement)))
        .add_systems(
            Update,
            handle_gamepad_input.run_if(in_state(GameState::Movement)),
        )
        .add_systems(
            Update,
            dialogue::handle_gamepad_input.run_if(in_state(GameState::Dialogue)),
        )
        .add_systems(Update, handle_new_text_event)
        .add_systems(Update, update_floating_text)
        .add_systems(Update, ui::update_gorb_count)
        .add_systems(Update, timer_update.run_if(in_state(GameState::Movement)))
        .add_systems(
            Update,
            check_for_player_on_target.run_if(in_state(GameState::Movement)),
        )
        .add_systems(
            Update,
            check_for_player_on_gorb_hole.run_if(in_state(GameState::Movement)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2d::default();
    commands.spawn((camera, WorldContent));

    commands.spawn((
        WorldContent,
        Sprite::from_image(asset_server.load("dude.png")),
        Transform::from_xyz(
            (WORLD_WIDTH as f32 / 2.0).round() * TILE_SIZE,
            (WORLD_HEIGHT as f32 / 2.0).round() * TILE_SIZE,
            5.0,
        ),
        Player::new(Vec2::new(
            (WORLD_WIDTH as f32 / 2.0).round(),
            (WORLD_HEIGHT as f32 / 2.0).round(),
        )),
    ));
}

const PLAYER_SLOWDOWN: f32 = 0.25;
const MID_AIR_SLOWDOWN: f32 = 0.95;
const MIN_SPEED: f32 = 0.001;
const MAX_VEL: f32 = 120.0;
const MID_AIR_SPEED_MULT: usize = 1;

fn handle_gamepad_input(
    mut query: Query<(&mut Player, &mut Transform)>,
    mut ev_new_text: EventWriter<NewText>,
    world: Res<WorldMap>,
    gamepads: Query<&Gamepad>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_dir = Vec2::ZERO;
    let mut jump_just_pressed = false;
    for gamepad in gamepads.iter() {
        player_dir = gamepad.left_stick();
        jump_just_pressed = gamepad.just_pressed(GamepadButton::South);
    }
    if player_dir == Vec2::ZERO {
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player_dir.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player_dir.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player_dir.x += -1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player_dir.y += -1.0;
        }
        player_dir = player_dir.clamp_length_max(1.0);
    }
    if !jump_just_pressed {
        jump_just_pressed = keyboard_input.just_pressed(KeyCode::Space);
    }
    for (mut player, mut player_transform) in &mut query {
        if jump_just_pressed && player.jump_start.is_none() {
            player.jump_start = Some(time.elapsed_secs_f64())
        }
        let curr_height = player.get_height(time.elapsed_secs_f64());
        if curr_height == 0.0 && player.jump_start.is_some() {
            player.jump_start = None;
            if player.gorb_count != 0 {
                ev_new_text.write(NewText(
                    "gorb borked".to_string(),
                    player_transform.translation.x,
                    player_transform.translation.y,
                ));
            } else {
                ev_new_text.write(NewText(
                    "gorbless".to_string(),
                    player_transform.translation.x,
                    player_transform.translation.y,
                ));
            }
            player.gorb_count = if player.gorb_count == 0 {
                0
            } else {
                player.gorb_count - 1
            };
        }
        let grid_val =
            match world.get_val_at_coord(player.grid_pos.x as i32, player.grid_pos.y as i32) {
                Some(val) => val,
                None => 0,
            } + 1;
        player_transform.scale = Vec3::splat(1.0) * (curr_height + (grid_val as f32 / 16.0));
        let val_on_grid = if curr_height > 0.0 {
            MID_AIR_SPEED_MULT
        } else {
            grid_val
        };
        let delta_f32 = time.delta_secs_f64() as f32;
        let dir_times_speed_and_time = player_dir * player.speed / delta_f32;
        if dir_times_speed_and_time.length() > 0.0 {
            player.velocity += dir_times_speed_and_time * (val_on_grid as f32 / 16.0);
        }
        if player.velocity.length() > MIN_SPEED {
            player.velocity *= if curr_height > 0.0 {
                MID_AIR_SLOWDOWN
            } else {
                PLAYER_SLOWDOWN
            };
            let new_vel = player.velocity.length();
            if new_vel > MAX_VEL {
                player.velocity *= 1.0 / (new_vel / MAX_VEL);
            }
            player_transform.translation += Vec3::new(
                player.velocity.x * delta_f32,
                player.velocity.y * delta_f32,
                0.0,
            );
            player.grid_pos.x = (player_transform.translation.x / TILE_SIZE).round();
            if player.grid_pos.x < 0.0 {
                player.grid_pos.x = 0.0
            }
            player.grid_pos.y = (player_transform.translation.y / TILE_SIZE).round();
            if player.grid_pos.y < 0.0 {
                player.grid_pos.y = 0.0
            }
        } else if player.velocity != Vec2::ZERO {
            player.velocity = Vec2::ZERO;
        }
    }
}

fn handle_gamepad_input_menu(
    mut next_game_state: ResMut<NextState<GameState>>,
    gamepads: Query<&Gamepad>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::South) {
            next_game_state.set(GameState::Movement);
        }
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_game_state.set(GameState::Movement);
    }
}

const CAM_SPEED: f32 = 5.0;

fn camera_transform_update(
    mut cam_query: Query<&mut Transform, With<Camera>>,
    pleyer_t_query: Query<(&Transform, &Player), Without<Camera>>,

    time: Res<Time>,
) {
    let mut pos = Vec2::ZERO;
    for (player_t, _player) in pleyer_t_query.iter() {
        pos.x = player_t.translation.x;
        pos.y = player_t.translation.y;
    }

    for mut transform in cam_query.iter_mut() {
        let mut diff = Vec3::new(pos.x, pos.y, transform.translation.z) - transform.translation;
        diff = diff * time.delta_secs_f64() as f32 * CAM_SPEED;
        transform.translation += diff;
    }
}

pub fn arrow_update(
    mut arrow_query: Query<&mut Transform, With<Arrow>>,
    target_query: Query<&Target>,
    player_query: Query<&Player>,
) {
    let mut player_to_target = Vec2::ZERO;
    for player in &player_query {
        for target in &target_query {
            player_to_target = target.position - player.grid_pos;
        }
    }
    for mut arrow_transform in &mut arrow_query {
        let target_angle = player_to_target.to_angle();
        arrow_transform.rotation =
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, (PI / 2.0) - target_angle);
    }
}

pub fn timer_update(
    player_query: Query<&Player>,
    mut timer_text_query: Query<(&mut Text, &mut SterbTime)>,
    time: Res<Time>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_dialogue: ResMut<CurrentDialogue>,
) {
    let mut player_blood = 0;
    for player in &player_query {
        player_blood = player.blood_on_your_hands;
    }

    for (mut text, mut sterb) in &mut timer_text_query {
        sterb.0.tick(time.delta());
        if sterb.0.finished() {
            current_dialogue.message = format!(
                "worthless gorb transporter, blood of {} gorbless is on your hands. try again.",
                player_blood
            );
            current_dialogue.effects = vec![DialogueEffect::GorbEmpty, DialogueEffect::GameOver];
            next_game_state.set(GameState::Dialogue);
        } else {
            let seconds_left = SECONDS_IN_TIMER - sterb.0.elapsed_secs();
            let min_left = (seconds_left / 60.0).floor();
            let sec_remainder = (seconds_left - (min_left * 60.0)).floor();
            text.0 = format!("00:{:0>2}:{:0>2}", min_left, sec_remainder);
        }
    }
}

pub fn check_for_player_on_target(
    player_query: Query<&Player>,
    target_query: Query<&Target>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_dialogue: ResMut<CurrentDialogue>,
) {
    for player in &player_query {
        for target in &target_query {
            if player.grid_pos == target.position {
                match player.gorb_count {
                    0 => {
                        current_dialogue.message =
                            "you come bearing no gorbs, the children will perish, scoundrel"
                                .to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(300),
                            DialogueEffect::ResetTargetLoc,
                        ];
                    }
                    1 => {
                        current_dialogue.message =
                            "one gorb does not a savior make, how can we choose who will live?"
                                .to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(250),
                            DialogueEffect::ResetTargetLoc,
                            DialogueEffect::GorbEmpty,
                        ];
                    }
                    2 => {
                        current_dialogue.message =
                            "you did all you could to get these two gorbs - and it sucks, you suck"
                                .to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(100),
                            DialogueEffect::ResetTargetLoc,
                            DialogueEffect::GorbEmpty,
                        ];
                    }
                    3 => {
                        current_dialogue.message =
                            "they will fight each other for these gorbs...".to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(136),
                            DialogueEffect::GorbEmpty,
                            DialogueEffect::ResetTargetLoc,
                        ];
                    }
                    4 => {
                        current_dialogue.message =
                            "meager gorbage this is not, many thanks for your transport"
                                .to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(70),
                            DialogueEffect::GorbEmpty,
                            DialogueEffect::ResetTargetLoc,
                            DialogueEffect::BonusTime,
                        ];
                    }
                    5 => {
                        current_dialogue.message =
                            "a wealth of gorbs! some chirldren will live!".to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(15),
                            DialogueEffect::GorbEmpty,
                            DialogueEffect::ResetTargetLoc,
                            DialogueEffect::BonusTime,
                        ];
                    }
                    _ => {
                        current_dialogue.message =
                            "we know not how many gorbs were recieved".to_string();
                        current_dialogue.effects = vec![
                            DialogueEffect::IncreaseBlood(5),
                            DialogueEffect::GorbEmpty,
                            DialogueEffect::ResetTargetLoc,
                        ];
                    }
                }
                next_game_state.set(GameState::Dialogue);
            }
        }
    }
}

pub fn check_for_player_on_gorb_hole(
    mut player_query: Query<&mut Player>,
    gorb_hole_query: Query<&GorbHole>,
    mut ev_new_text: EventWriter<NewText>,
) {
    for mut player in &mut player_query {
        for gorb_hole in &gorb_hole_query {
            if player.grid_pos == gorb_hole.position && player.gorb_count != DEFAULT_GORB_COUNT {
                player.gorb_count = DEFAULT_GORB_COUNT;
                ev_new_text.write(NewText(
                    "gorbs chamber refill".to_string(),
                    player.grid_pos.x * TILE_SIZE,
                    player.grid_pos.y * TILE_SIZE,
                ));
            }
        }
    }
}
