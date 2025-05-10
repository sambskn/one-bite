use bevy::prelude::*;

const PLAYER_SPEED: f32 = 7.0;
const JUMP_TIME_S: f64 = 1.5;
const JUMP_MAX_HEIGHT: f64 = 2.0;
pub const DEFAULT_GORB_COUNT: i32 = 5;

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub grid_pos: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub jump_start: Option<f64>,
    pub gorb_count: i32,
    pub blood_on_your_hands: i32,
}

impl Player {
    pub fn new(grid_pos: Vec2) -> Self {
        Player {
            grid_pos,
            velocity: Vec2::ZERO,
            speed: PLAYER_SPEED,
            jump_start: None,
            gorb_count: DEFAULT_GORB_COUNT,
            blood_on_your_hands: 0,
        }
    }

    pub fn get_height(self, time_elapsed: f64) -> f32 {
        match self.jump_start {
            Some(start_time) => {
                if start_time + JUMP_TIME_S > time_elapsed {
                    let t = time_elapsed - start_time;
                    let j_height = (JUMP_MAX_HEIGHT - (t + JUMP_MAX_HEIGHT.sqrt())) as f32;
                    if j_height > 0.0 { j_height } else { 0.0 }
                } else {
                    0.0
                }
            }
            None => 0.0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            grid_pos: Vec2::ZERO,
            velocity: Vec2::ZERO,
            speed: PLAYER_SPEED,
            jump_start: None,
            gorb_count: DEFAULT_GORB_COUNT,
            blood_on_your_hands: 0,
        }
    }
}
