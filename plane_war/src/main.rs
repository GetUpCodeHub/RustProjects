mod config;
mod player;
mod bullet;
mod enemy;
mod game;
mod render;

use macroquad::prelude::*;
use game::Game;

use crate::config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();

        game.update(dt);

        render::draw_game(&game);

        next_frame().await;
    }
}