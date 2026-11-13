use macroquad::prelude::*;
use crate::config::PLAYER_SPEED;

pub struct Player {
    pub pos: Vec2,
    pub size: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: vec2(
                screen_width() / 2.0,
                screen_height() - 70.0,
            ),
            size: 20.0,
        }
    }

    // Update player movement
    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.pos.x -= PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.pos.x += PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.pos.y -= PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.pos.y += PLAYER_SPEED * dt;
        }

        // Keep the player inside the screen boundaries
        self.pos.x = self.pos.x.clamp(20.0, screen_width() - 20.0);
        self.pos.y = self.pos.y.clamp(30.0, screen_height() - 30.0);
    }

    // Draw the player aircraft
    pub fn draw(&self) {
        let p = self.pos;

        draw_triangle(
            vec2(p.x, p.y - 25.0),
            vec2(p.x - 20.0, p.y + 20.0),
            vec2(p.x + 20.0, p.y + 20.0),
            SKYBLUE,
        );

        draw_triangle(
            vec2(p.x, p.y - 12.0),
            vec2(p.x - 7.0, p.y + 12.0),
            vec2(p.x + 7.0, p.y + 12.0),
            WHITE,
        );
    }
}