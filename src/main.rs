use macroquad::audio::{Sound, load_sound};
use macroquad::prelude::*;
mod ball;
mod collision;
mod computer;
mod player;
mod ui;

use ball::*;
use collision::*;
use computer::*;
use player::*;
use ui::*;

// Main Game loop
#[macroquad::main("PONG")]
async fn main() {
    //audio
    let hit_sound: Sound = load_sound("hit.wav").await.unwrap();
    let score_sound: Sound = load_sound("score.wav").await.unwrap();

    // objects
    let position = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut ball = Ball::new(position);
    let mut player = Player::new();
    let mut computer = Computer::new();
    let mut collision = Collision::new(hit_sound, score_sound);
    let mut ui = Info::new();

    loop {
        clear_background(BLACK);

        // collision calls
        collision.physics(&mut ball, &player.rect);
        collision.physics(&mut ball, &computer.rect);
        collision.score_update(&mut ball);

        // UI calls
        ui.draw_score(&mut collision);

        //ball calls
        ball.ball_movement(get_frame_time(), &mut collision.hit_sound);
        ball.sprite();

        // compter calls
        computer.computer_movement(&mut ball, get_frame_time());
        computer.sprite();
        computer.update_pos();

        //player calls
        player.update_pos();
        player.sprite();
        player.player_movement(get_frame_time());

        next_frame().await
    }
}
