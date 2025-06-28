use macroquad::audio::{Sound, load_sound };
use macroquad::prelude::*;
mod ball;
mod collision;
mod computer;
mod player;
mod ui;

use ball::Ball;
use collision::*;
use computer::Computer;
use player::Player;
use ui::Info;

// Main Game loop
#[macroquad::main("PONG")]
async fn main() {
    //audio
    let hit_sound: Sound = load_sound("hit.wav").await.unwrap();

    // objects
    let position = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut ball = Ball::new(position);
    let mut player = Player::new();
    let mut computer = Computer::new();

    let mut collision = Collision::new(hit_sound);
    let mut info = Info::new();
    //let sound = load_sound("underclocked.wav").await.unwrap();

    loop {
        clear_background(BLACK);
        /*    play_sound(
            &sound,
            PlaySoundParams {
                looped: true,
                volume: 0.5,
            },
        );*/
        //normal function calls
        collision.physics(&mut ball, &player.rect);
        collision.physics(&mut ball, &computer.rect);

        //object function calls
        collision.score_update(&mut ball);
        collision.audio(&mut ball, &player.rect);
        collision.audio(&mut ball, &computer.rect);
        info.draw_score(&mut collision);
        ball.ball_movement(get_frame_time());
        ball.sprite();
        computer.computer_movement(&mut ball, get_frame_time());
        computer.sprite();
        computer.update_pos();
        player.update_pos();
        player.sprite();

        player.player_movement(get_frame_time());
        next_frame().await
    }
}
