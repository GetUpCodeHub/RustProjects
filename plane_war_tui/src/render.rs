use crate::game::{Game, WIDTH, HEIGHT};
use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn draw(game: &Game) {
    let mut screen = vec![vec![' '; WIDTH]; HEIGHT];

    let title = format!(" PLANE WAR | SCORE: {} ", game.score);

    for (i, c) in title.chars().enumerate() {
        if i < WIDTH {
            screen[0][i] = c;
        }
    }

    for x in 0..WIDTH {
        screen[HEIGHT - 1][x] = '-';
    }

    for row in screen.iter_mut().skip(1).take(HEIGHT - 2) {
        row[0] = '|';
        row[WIDTH - 1] = '|';
    }

    let px = game.player.x;
    let py = game.player.y;

    screen[py][px] = 'A';

    if px > 1 {
        screen[py][px - 1] = '/';
    }

    if px + 1 < WIDTH - 1 {
        screen[py][px + 1] = '\\';
    }

    for bullet in &game.bullets {
        if bullet.y > 0 && bullet.y < HEIGHT - 1 {
            screen[bullet.y][bullet.x] = '|';
        }
    }

    for enemy in &game.enemies {
        if enemy.y > 0 && enemy.y < HEIGHT - 1 {
            screen[enemy.y][enemy.x] = 'V';
        }
    }

    if game.game_over {
        let message = " GAME OVER! PRESS R TO RESTART ";
        let start = (WIDTH - message.len()) / 2;
        let y = HEIGHT / 2;

        for (i, c) in message.chars().enumerate() {
            if start + i < WIDTH {
                screen[y][start + i] = c;
            }
        }
    }

    let mut out = stdout();

    execute!(
        out,
        cursor::MoveTo(
            u16::try_from(0usize).expect("Invalid X"),
            u16::try_from(0usize).expect("Invalid Y")
        ),
        Clear(ClearType::All)
    ).unwrap();

    for row in &screen {
        println!("{}", row.iter().collect::<String>());
    }

    println!(" A/D: MOVE | SPACE: SHOOT | Q: QUIT");
}