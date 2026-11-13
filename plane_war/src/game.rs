use macroquad::prelude::*;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;

pub struct Game {
    pub player: Player,
    pub bullets: Vec<Bullet>,
    pub enemies: Vec<Enemy>,
    pub score: u32,
    pub game_over: bool,

    shoot_timer: f32,
    spawn_timer: f32,
}

impl Game {
    // Initialize a new game
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            shoot_timer: 0.0,
            spawn_timer: 0.0,
        }
    }

    // Update the game state
    pub fn update(&mut self, dt: f32) {
        // Handle game restart
        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                *self = Game::new();
            }
            return;
        }

        // Update player movement
        self.player.update(dt);

        // Handle automatic shooting
        self.shoot_timer -= dt;

        if is_key_down(KeyCode::Space) && self.shoot_timer <= 0.0 {
            self.bullets.push(Bullet::new(
                vec2(
                    self.player.pos.x,
                    self.player.pos.y - 25.0,
                ),
            ));

            self.shoot_timer = 0.18;
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }

        // Spawn new enemies
        self.spawn_timer -= dt;

        if self.spawn_timer <= 0.0 {
            self.enemies.push(Enemy::new());
            self.spawn_timer = 0.7;
        }

        // Update enemy positions
        for enemy in &mut self.enemies {
            enemy.update(dt);
        }

        // Check bullet-enemy collisions
        for bullet in &mut self.bullets {
            if !bullet.active {
                continue;
            }

            for enemy in &mut self.enemies {
                if !enemy.active {
                    continue;
                }

                if bullet.pos.distance(enemy.pos) < 23.0 {
                    bullet.active = false;
                    enemy.active = false;
                    self.score += 10;
                    break;
                }
            }
        }

        // Check player-enemy collisions
        for enemy in &self.enemies {
            if enemy.active {
                let distance = self.player.pos.distance(enemy.pos);

                if distance < self.player.size + 14.0 {
                    self.game_over = true;
                    break;
                }
            }
        }

        // Remove inactive bullets and enemies
        self.bullets.retain(|b| b.active);
        self.enemies.retain(|e| e.active);
    }
}