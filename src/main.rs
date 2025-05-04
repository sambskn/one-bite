use bevy::prelude::*;
use player::Player;
use world::{TILE_SIZE, generate_world};
mod player;
mod world;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgba(1.0, 1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Startup, generate_world)
        .add_systems(Update, camera_transform_update)
        .add_systems(Update, handle_gamepad_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2d::default();
    commands.spawn(camera);

    commands.spawn((
        Sprite::from_image(asset_server.load("dude.png")),
        Player::new(Vec2::ZERO),
    ));
}

const PLAYER_SIZE: f32 = 32.0;
const PLAYER_SLOWDOWN: f32 = 0.25;
const MIN_SPEED: f32 = 0.00000001;
const MAX_VEL: f32 = 40.0;

fn handle_gamepad_input(
    mut query: Query<(&mut Player, &mut Transform)>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
) {
    for gamepad in gamepads.iter() {
        let gamepad_left_stick = gamepad.left_stick();
        for (mut player, mut player_transform) in &mut query {
            let delta_f32 = time.delta_secs_f64() as f32;
            let dir_times_speed_and_time = gamepad_left_stick * player.speed / delta_f32;
            if dir_times_speed_and_time.length() > 0.0 {
                player.velocity += dir_times_speed_and_time;
            }
            if player.velocity.length() > MIN_SPEED {
                player.velocity *= PLAYER_SLOWDOWN;
                let new_vel = player.velocity.length();
                if new_vel > MAX_VEL {
                    player.velocity *= 1.0 / (new_vel / MAX_VEL);
                }
                player_transform.translation += Vec3::new(
                    player.velocity.x * delta_f32,
                    player.velocity.y * delta_f32,
                    0.0,
                );
                player.grid_pos.x =
                    ((player_transform.translation.x - (PLAYER_SIZE / 2.0)) / TILE_SIZE).round();
                player.grid_pos.y =
                    ((player_transform.translation.y - (PLAYER_SIZE / 2.0)) / TILE_SIZE).round();
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

const CAM_SPEED: f32 = 50.0;

fn camera_transform_update(
    mut cam_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Player>,
    time: Res<Time>,
) {
    for player in player_query.iter() {
        for mut transform in cam_query.iter_mut() {
            let mut diff = Vec3::new(
                player.grid_pos.x as f32 * TILE_SIZE,
                player.grid_pos.y as f32 * TILE_SIZE,
                transform.translation.z,
            ) - transform.translation;
            diff = diff * time.delta_secs_f64() as f32 * CAM_SPEED;
            transform.translation += diff;
        }
    }
}
