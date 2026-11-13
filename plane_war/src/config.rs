use macroquad::prelude::*;

pub const PLAYER_SPEED: f32 = 400.0;
pub const BULLET_SPEED: f32 = 600.0;
pub const ENEMY_SPEED: f32 = 150.0;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Plane War".to_owned(),
        window_width: 480,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}