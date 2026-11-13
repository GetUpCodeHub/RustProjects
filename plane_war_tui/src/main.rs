mod game;
mod player;
mod enemy;
mod bullet;
mod render;
mod terminal;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::Game;
use std::time::{Duration, Instant};
use std::thread;

fn main() -> std::io::Result<()> {
    let _terminal = terminal::TerminalGuard::new()?;

    let mut game = Game::new();
    let mut last_time = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        while event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(());
                    }

                    KeyCode::Char('a') | KeyCode::Left => {
                        if !game.game_over {
                            game.move_left();
                        }
                    }

                    KeyCode::Char('d') | KeyCode::Right => {
                        if !game.game_over {
                            game.move_right();
                        }
                    }

                    KeyCode::Char(' ') => game.shoot(),

                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        if game.game_over {
                            game = Game::new();
                        }
                    }

                    _ => {}
                }
            }
        }

        game.update(dt);
        game.draw();

        thread::sleep(Duration::from_millis(80));
    }
}