use macroquad::{
    audio::{Sound, load_sound_from_bytes, play_sound, play_sound_once},
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
use quad_snd::PlaySoundParams;
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
    GameWin,
    GameOver,
}

// Main Game loop
#[macroquad::main("PONG")]
async fn main() {
    //audio
    let hit_sound_bytes = include_bytes!("hit.wav");
    let score_sound_bytes = include_bytes!("score.wav");
    let background_sound_bytes = include_bytes!("background_music.wav");

    let hit_sound: Sound = load_sound_from_bytes(hit_sound_bytes).await.unwrap();
    let score_sound: Sound = load_sound_from_bytes(score_sound_bytes).await.unwrap();
    let background_sound: Sound = load_sound_from_bytes(background_sound_bytes).await.unwrap();

    // objects
    let position = vec2(screen_width() * 0.5, screen_height() * 0.5);
    let mut ball = Ball::new(position);
    let mut player = Player::new();
    let mut computer = Computer::new();
    let mut collision = Collision::new(hit_sound, score_sound);
    let mut ui = Info::new();
    //gamestate
    let mut game_state = GameState::Menu;
    //music
    let mut music = false;

    loop {
        match game_state {
            GameState::Menu => {
                // Menu logic
                if !music {
                    play_sound(
                        &background_sound,
                        PlaySoundParams {
                            volume: 0.5,
                            looped: false,
                        },
                    );
                    music = true;
                }
                draw_text("Press [ Space ] to start");
                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Playing;
                    music = false;
                }
            }
            GameState::Playing => {
                // Playing logic
                clear_background(BLACK);
                music = false;
                // collision calls
                collision.physics(&mut ball, &player.rect);
                collision.physics(&mut ball, &computer.rect);
                collision.score_update(&mut ball, &mut player, &mut computer);

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
                if collision.player_score == 5 {
                    collision.reset(&mut ball, &mut player, &mut computer);
                    game_state = GameState::GameWin;
                } else if collision.comp_score == 5 {
                    collision.reset(&mut ball, &mut player, &mut computer);
                    game_state = GameState::GameOver;
                }
            }
            GameState::GameWin => {
                // Game Win logic
                draw_text("You Win \n Press [ Space ] to restart.");

                if is_key_down(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
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
