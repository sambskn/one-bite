use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowResolution};
use floating_text::{NewText, handle_new_text_event, update_floating_text};
use player::Player;
use ui::{Arrow, SECONDS_IN_TIMER, SterbTime};
use world::{TILE_SIZE, Target, WORLD_HEIGHT, WORLD_WIDTH, WorldMap, generate_world};
mod floating_text;
mod menu;
mod player;
mod ui;
mod world;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    Dialogue,
    Movement,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgba(0.0, 0.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600.0, 500.0),
                title: "gorb transporter".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_event::<NewText>()
        .insert_state(GameState::MainMenu)
        .add_systems(Startup, menu::menu_setup)
        .add_systems(
            Update,
            handle_gamepad_input_menu.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), menu::clear_menu_content)
        .add_systems(OnExit(GameState::MainMenu), setup)
        .add_systems(OnExit(GameState::MainMenu), generate_world)
        .add_systems(OnExit(GameState::MainMenu), ui::setup_ui)
        .add_systems(
            Update,
            camera_transform_update.run_if(in_state(GameState::Movement)),
        )
        .add_systems(Update, arrow_update.run_if(in_state(GameState::Movement)))
        .add_systems(
            Update,
            handle_gamepad_input.run_if(in_state(GameState::Movement)),
        )
        .add_systems(Update, handle_new_text_event)
        .add_systems(Update, update_floating_text)
        .add_systems(Update, timer_update.run_if(in_state(GameState::Movement)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2d::default();
    commands.spawn(camera);

    commands.spawn((
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
    time: Res<Time>,
) {
    for gamepad in gamepads.iter() {
        let gamepad_left_stick = gamepad.left_stick();
        for (mut player, mut player_transform) in &mut query {
            if gamepad.just_pressed(GamepadButton::South) && player.jump_start.is_none() {
                player.jump_start = Some(time.elapsed_secs_f64())
            }
            let curr_height = player.get_height(time.elapsed_secs_f64());
            if curr_height == 0.0 && player.jump_start.is_some() {
                player.jump_start = None;
                ev_new_text.write(NewText(
                    "gorb borked".to_string(),
                    player_transform.translation.x,
                    player_transform.translation.y,
                ));
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
            let dir_times_speed_and_time = gamepad_left_stick * player.speed / delta_f32;
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

            if gamepad.just_pressed(GamepadButton::DPadUp) {
                player.speed += 0.5;
            }
            if gamepad.just_pressed(GamepadButton::DPadDown) {
                player.speed += -0.5;
            }
        }
    }
}

fn handle_gamepad_input_menu(
    mut next_game_state: ResMut<NextState<GameState>>,
    gamepads: Query<&Gamepad>,
) {
    for gamepad in gamepads.iter() {
        if gamepad.just_pressed(GamepadButton::South) {
            next_game_state.set(GameState::Movement);
        }
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

pub fn timer_update(mut timer_text_query: Query<(&mut Text, &mut SterbTime)>, time: Res<Time>) {
    for (mut text, mut sterb) in &mut timer_text_query {
        sterb.0.tick(time.delta());
        let seconds_left = SECONDS_IN_TIMER - sterb.0.elapsed_secs();
        let min_left = (seconds_left / 60.0).floor();
        let sec_remainder = (seconds_left - (min_left * 60.0)).floor();
        text.0 = format!("00:{:0>2}:{:0>2}", min_left, sec_remainder);
    }
}
