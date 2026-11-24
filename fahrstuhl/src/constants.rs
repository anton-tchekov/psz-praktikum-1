use macroquad::prelude::*;

pub const EPS: f32 = 0.003;

pub const SPAWN_RATE: f32 = 1.2; // Hz (Average)

pub const NUM_CABINS: usize = 3;
pub const NUM_FLOORS: usize = 4;

pub const FLOOR_BORDER: f32 = 0.5;
pub const FLOOR_W: f32 = CABIN_STEP * (NUM_CABINS - 1) as f32 + WIDTH + 2.0 * WALL;

pub const PLAYER_SPEED: f32 = 1.0;

pub const WALL: f32 = 0.05;
pub const HEIGHT: f32 = 2.0;
pub const WIDTH: f32 = 1.0;
pub const DEPTH: f32 = 1.2;
pub const GAP: f32 = 0.02;

pub const DOOR_SPEED: f32 = 2.5;
pub const CABIN_SPEED: f32 = 2.5;

pub const CABIN_STEP: f32 = 2.0;
pub const FLOOR_HEIGHT: f32 = 3.0;

pub const FLOOR_D: f32 = 3.0;
pub const DOOR_X: f32 = WIDTH / 6.0;
