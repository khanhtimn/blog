use serde::{Serialize, Deserialize};

pub const BACKDROP_COLOR: &str = "#30c0df";
pub const CANVAS_WIDTH: u32 = 552;
pub const CANVAS_HEIGHT: u32 = 480;
pub const GRAVITY: f64 = 0.125;
pub const THRUST: f64 = 3.6;
pub const PIPE_GAP: f64 = 90.0;
pub const PIPE_TOP_HEIGHT: f64 = 400.0;
pub const PIPE_Y_MIN_MAX: (f64, f64) = (-350.0, -150.0);
pub const PIPE_WIDTH: f64 = 52.0;
pub const BIRD_SIZE: f64 = 34.0;
pub const GROUND_HEIGHT: f64 = 112.0;
pub const BACKGROUND_HEIGHT: f64 = 228.0;
pub const BIRD_X_POSITION: f64 = 80.0;
pub const BIRD_Y_POSITION: f64 = 80.0;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GameAssets {
    pub bird_frames: Vec<String>,
    pub pipe_top: String,
    pub pipe_bottom: String,
    pub background: String,
    pub ground: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Loading,
    GetReady,
    Playing,
    GameOver,
}

#[derive(Clone)]
pub struct Bird {
    pub y: f64,
    pub speed: f64,
    pub rotation: f64,
    pub frame: usize,
}

#[derive(Clone)]
pub struct Pipe {
    pub x: f64,
    pub y: f64,
}
