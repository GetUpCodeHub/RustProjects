use macroquad::prelude::*;
use crate::config::ENEMY_SPEED;

pub struct Enemy {
    pub pos: Vec2,
    pub speed: f32,
    pub active: bool,
}

impl Enemy {
    // Generate a new enemy
    pub fn new() -> Self {
        Self {
            pos: vec2(
                rand::gen_range(25.0, screen_width() - 25.0),
                -20.0,
            ),
            speed: ENEMY_SPEED + rand::gen_range(0.0, 100.0),
            active: true,
        }
    }

    // Update enemy position
    pub fn update(&mut self, dt: f32) {
        self.pos.y += self.speed * dt;

        // Deactivate enemies that leave the screen
        if self.pos.y > screen_height() + 30.0 {
            self.active = false;
        }
    }

    // Draw the enemy aircraft
    pub fn draw(&self) {
        let p = self.pos;

        draw_triangle(
            vec2(p.x, p.y + 20.0),
            vec2(p.x - 19.0, p.y - 15.0),
            vec2(p.x + 19.0, p.y - 15.0),
            RED,
        );

        draw_circle(p.x, p.y, 8.0, MAROON);
    }
}