use macroquad::{
    audio::{Sound, load_sound_from_bytes},
    prelude::*,
};
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

pub fn draw_text(text: &str) {
    draw_text_ex(
        &text,
        screen_width() / 3.,
        screen_height() / 2.,
        TextParams {
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );
}
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

// Main Game loop
#[macroquad::main("PONG")]
async fn main() {
    //audio
    let hit_sound_bytes = include_bytes!("hit.wav");
    let score_sound_bytes = include_bytes!("score.wav");

    let hit_sound: Sound = load_sound_from_bytes(hit_sound_bytes).await.unwrap();
    let score_sound: Sound = load_sound_from_bytes(score_sound_bytes).await.unwrap();

    // objects
    let position = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut ball = Ball::new(position);
    let mut player = Player::new();
    let mut computer = Computer::new();
    let mut collision = Collision::new(hit_sound, score_sound);
    let mut ui = Info::new();
    //gamestate
    let mut game_state = GameState::Menu;

    loop {
        match game_state {
            GameState::Menu => {
                // Menu logic
                //
                draw_text("Press [ Space ] to start");
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Playing logic
                clear_background(BLACK);
                // collision calls
                collision.physics(&mut ball, &player.rect);
                collision.physics(&mut ball, &computer.rect);
                collision.score_update(&mut ball, &mut player, &mut computer);
                if collision.player_score == 5 {
                    draw_text("You win!");
                    game_state = GameState::Menu;
                    collision.comp_score = 0;
                    collision.player_score = 0;
                } else if collision.comp_score == 5 {
                    game_state = GameState::GameOver;
                    collision.comp_score = 0;
                    collision.player_score = 0;
                }

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
            }
            GameState::GameOver => {
                // Game Over logic
                draw_text("Game Over \n Press [ Space ] to restart.");
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}
