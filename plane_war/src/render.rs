use macroquad::prelude::*;
use crate::game::Game;

pub fn draw_game(game: &Game) {
    // Draw the background
    clear_background(Color::from_rgba(8, 12, 35, 255));

    // Draw the starfield
    for i in 0..70 {
        let x = ((i * 137 + 31) % 997) as f32
            / 997.0 * screen_width();

        let y = ((i * 251 + 17) % 991) as f32
            / 991.0 * screen_height();

        draw_circle(x, y, 1.5, WHITE);
    }

    // Draw the player aircraft
    game.player.draw();

    // Draw bullets
    for bullet in &game.bullets {
        bullet.draw();
    }

    // Draw enemy aircraft
    for enemy in &game.enemies {
        enemy.draw();
    }

    // Display the score
    draw_text(
        &format!("SCORE: {}", game.score),
        20.0,
        35.0,
        30.0,
        WHITE,
    );

    // Display control instructions
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

    // Display the game over screen
    if game.game_over {
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

        // Display the final score
        let score_text = format!("FINAL SCORE: {}", game.score);

        let score_width = measure_text(
            &score_text,
            None,
            25,
            1.0,
        ).width;

        draw_text(
            &score_text,
            screen_width() / 2.0 - score_width / 2.0,
            screen_height() / 2.0 + 20.0,
            25.0,
            WHITE,
        );

        // Display restart instructions
        let restart = "PRESS R TO RESTART";

        let restart_width = measure_text(
            restart,
            None,
            22,
            1.0,
        ).width;

        draw_text(
            restart,
            screen_width() / 2.0 - restart_width / 2.0,
            screen_height() / 2.0 + 70.0,
            22.0,
            YELLOW,
        );
    }
}