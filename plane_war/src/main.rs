
use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 400.0;
const BULLET_SPEED: f32 = 600.0;
const ENEMY_SPEED: f32 = 150.0;

struct Player {
    pos: Vec2,
    size: f32,
}

struct Bullet {
    pos: Vec2,
    active: bool,
}

struct Enemy {
    pos: Vec2,
    speed: f32,
    active: bool,
}

struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    game_over: bool,
    shoot_timer: f32,
    spawn_timer: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player {
                pos: vec2(screen_width() / 2.0, screen_height() - 70.0),
                size: 20.0,
            },
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            shoot_timer: 0.0,
            spawn_timer: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                *self = Game::new();
            }
            return;
        }

        // 玩家移动
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.player.pos.x -= PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.player.pos.x += PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.player.pos.y -= PLAYER_SPEED * dt;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.player.pos.y += PLAYER_SPEED * dt;
        }

        // 限制飞机不能飞出屏幕
        self.player.pos.x = self.player.pos.x.clamp(
            20.0,
            screen_width() - 20.0,
        );

        self.player.pos.y = self.player.pos.y.clamp(
            30.0,
            screen_height() - 30.0,
        );

        // 自动发射子弹
        self.shoot_timer -= dt;

        if is_key_down(KeyCode::Space) && self.shoot_timer <= 0.0 {
            self.bullets.push(Bullet {
                pos: vec2(
                    self.player.pos.x,
                    self.player.pos.y - 25.0,
                ),
                active: true,
            });

            self.shoot_timer = 0.18;
        }

        // 更新子弹
        for bullet in &mut self.bullets {
            bullet.pos.y -= BULLET_SPEED * dt;

            if bullet.pos.y < 0.0 {
                bullet.active = false;
            }
        }

        // 生成敌机
        self.spawn_timer -= dt;

        if self.spawn_timer <= 0.0 {
            let x = rand::gen_range(
                25.0,
                screen_width() - 25.0,
            );

            self.enemies.push(Enemy {
                pos: vec2(x, -20.0),
                speed: ENEMY_SPEED + rand::gen_range(0.0, 100.0),
                active: true,
            });

            self.spawn_timer = 0.7;
        }

        // 更新敌机位置
        for enemy in &mut self.enemies {
            enemy.pos.y += enemy.speed * dt;

            if enemy.pos.y > screen_height() + 30.0 {
                enemy.active = false;
            }
        }

        // 子弹与敌机碰撞检测
        for bullet in &mut self.bullets {
            if !bullet.active {
                continue;
            }

            for enemy in &mut self.enemies {
                if !enemy.active {
                    continue;
                }

                let distance = bullet.pos.distance(enemy.pos);

                if distance < 23.0 {
                    bullet.active = false;
                    enemy.active = false;
                    self.score += 10;
                    break;
                }
            }
        }

        // 玩家与敌机碰撞检测
        for enemy in &self.enemies {
            if enemy.active {
                let distance = self.player.pos.distance(enemy.pos);

                if distance < self.player.size + 14.0 {
                    self.game_over = true;
                    break;
                }
            }
        }

        // 清理已经消失的子弹和敌机
        self.bullets.retain(|b| b.active);
        self.enemies.retain(|e| e.active);
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(8, 12, 35, 255));

        // 星空背景
        for i in 0..70 {
            let x = ((i * 137 + 31) % 997) as f32
                / 997.0 * screen_width();

            let y = ((i * 251 + 17) % 991) as f32
                / 991.0 * screen_height();

            draw_circle(x, y, 1.5, WHITE);
        }

        // 绘制玩家飞机
        let p = self.player.pos;

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

        // 绘制子弹
        for bullet in &self.bullets {
            draw_rectangle(
                bullet.pos.x - 3.0,
                bullet.pos.y - 10.0,
                6.0,
                18.0,
                YELLOW,
            );

            draw_circle(
                bullet.pos.x,
                bullet.pos.y - 8.0,
                4.0,
                ORANGE,
            );
        }

        // 绘制敌机
        for enemy in &self.enemies {
            let p = enemy.pos;

            draw_triangle(
                vec2(p.x, p.y + 20.0),
                vec2(p.x - 19.0, p.y - 15.0),
                vec2(p.x + 19.0, p.y - 15.0),
                RED,
            );

            draw_circle(p.x, p.y, 8.0, MAROON);
        }

        // 显示分数
        draw_text(
            &format!("SCORE: {}", self.score),
            20.0,
            35.0,
            30.0,
            WHITE,
        );

        draw_text(
            "MOVE: WASD / ARROWS",
            20.0,
            65.0,
            18.0,
            LIGHTGRAY,
        );

        draw_text(
            "SHOOT: SPACE",
            20.0,
            88.0,
            18.0,
            LIGHTGRAY,
        );

        // 游戏结束界面
        if self.game_over {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::from_rgba(0, 0, 0, 190),
            );

            let title = "GAME OVER";
            let title_size = 50.0;

            let title_width = measure_text(
                title,
                None,
                title_size as u16,
                1.0,
            ).width;

            draw_text(
                title,
                screen_width() / 2.0 - title_width / 2.0,
                screen_height() / 2.0 - 30.0,
                title_size,
                RED,
            );

            let score_text = format!("FINAL SCORE: {}", self.score);

            draw_text(
                &score_text,
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0 + 20.0,
                25.0,
                WHITE,
            );

            draw_text(
                "PRESS R TO RESTART",
                screen_width() / 2.0 - 125.0,
                screen_height() / 2.0 + 70.0,
                22.0,
                YELLOW,
            );
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Plane War".to_owned(),
        window_width: 480,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
