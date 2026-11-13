use crate::player::Player;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::render;

pub const WIDTH: usize = 40;
pub const HEIGHT: usize = 22;

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
    pub fn new() -> Self {
        Self {
            player: Player::new(WIDTH / 2, HEIGHT - 3),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            shoot_timer: 0.0,
            spawn_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_over {
            return;
        }

        self.shoot_timer -= dt;
        self.spawn_timer -= dt;

        for bullet in &mut self.bullets {
            bullet.update();
        }

        self.bullets.retain(|b| b.y > 0);

        if self.spawn_timer <= 0.0 {
            self.enemies.push(Enemy::new(WIDTH));
            self.spawn_timer = 0.8;
        }

        for enemy in &mut self.enemies {
            enemy.update();
        }

        let mut destroyed = Vec::new();
        let mut bullet_destroyed = Vec::new();

        for (bi, bullet) in self.bullets.iter().enumerate() {
            for (ei, enemy) in self.enemies.iter().enumerate() {
                if bullet.x == enemy.x && bullet.y == enemy.y {
                    destroyed.push(ei);
                    bullet_destroyed.push(bi);
                    break;
                }
            }
        }

        destroyed.sort_unstable();
        destroyed.dedup();

        bullet_destroyed.sort_unstable();
        bullet_destroyed.dedup();

        for &i in destroyed.iter().rev() {
            self.enemies.remove(i);
            self.score += 10;
        }

        for &i in bullet_destroyed.iter().rev() {
            self.bullets.remove(i);
        }

        for enemy in &self.enemies {
            if enemy.x == self.player.x && enemy.y == self.player.y
            {
                self.game_over = true;
            }
        }
    }

    pub fn move_left(&mut self) {
        self.player.move_left();
    }

    pub fn move_right(&mut self) {
        self.player.move_right(WIDTH);
    }

    pub fn shoot(&mut self) {
        if self.shoot_timer <= 0.0 && !self.game_over {
            self.bullets.push(Bullet::new(
                self.player.x,
                self.player.y - 1,
            ));

            self.shoot_timer = 0.2;
        }
    }

    pub fn draw(&self) {
        render::draw(self);
    }
}