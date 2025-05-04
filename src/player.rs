use bevy::prelude::*;

const PLAYER_SPEED: f32 = 5.0;


#[derive(Component)]
pub struct Player {
    pub grid_pos: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
}

impl Player {
    pub fn new(grid_pos: Vec2) -> Self {
        Player { grid_pos, velocity: Vec2::ZERO, speed: PLAYER_SPEED }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            grid_pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            speed: PLAYER_SPEED,
        }
    }
}
