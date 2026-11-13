use macroquad::prelude::*;
use crate::config::BULLET_SPEED;

pub struct Bullet {
    pub pos: Vec2,
    pub active: bool,
}

impl Bullet {
    // Create a new bullet
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            active: true,
        }
    }

    // Update bullet position
    pub fn update(&mut self, dt: f32) {
        self.pos.y -= BULLET_SPEED * dt;

        // Deactivate bullets that leave the screen
        if self.pos.y < 0.0 {
            self.active = false;
        }
    }

    // Draw the bullet
    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - 3.0,
            self.pos.y - 10.0,
            6.0,
            18.0,
            YELLOW,
        );

        draw_circle(
            self.pos.x,
            self.pos.y - 8.0,
            4.0,
            ORANGE,
        );
    }
}