use macroquad::{
    audio::{Sound, load_sound_from_bytes, play_sound},
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

// Window
fn window_conf() -> Conf {
    Conf {
        window_title: "P0NG".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        ..Default::default()
    }
}

pub enum GameState {
    Title,
    Playing,
    Paused,
    GameWin,
    GameOver,
}

// Main Game loop
#[macroquad::main(window_conf)]
async fn main() {
    //audio
    let hit_sound_bytes = include_bytes!("Sounds/hit.wav");
    let score_sound_bytes = include_bytes!("Sounds/score.wav");
    let background_sound_bytes = include_bytes!("Sounds/background_music.wav");

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
    let mut game_state = GameState::Title;

    //music
    let mut music_playing = false;

    loop {
        let dt = get_frame_time();

        match game_state {
            GameState::Title => {
                if !music_playing {
                    play_sound(
                        &background_sound,
                        PlaySoundParams {
                            volume: 0.5,
                            looped: true,
                        },
                    );
                    music_playing = true;
                }

                // Main menu
                ui.draw_main_menu(dt);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                clear_background(BLACK);

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Update player
                player.player_movement(dt);
                player.update_pos();

                // Update computer
                computer.computer_movement(&mut ball, dt);
                computer.update_pos();

                // Ball
                ball.ball_movement(dt, &mut collision.hit_sound);

                // Collision
                collision.physics(&mut ball, &player.rect, player.vel_y);
                collision.physics(&mut ball, &computer.rect, computer.vel_y);

                // Score update
                collision.score_update(&mut ball, &mut player, &mut computer);

                // Draw UI
                ui.draw_score(&mut collision);

                // Draw sprites
                ball.sprite();
                computer.sprite();
                player.sprite();

                // Check win
                if collision.player_score == 5 {
                    game_state = GameState::GameWin;
                } else if collision.comp_score == 5 {
                    game_state = GameState::GameOver;
                }
            }

            GameState::Paused => {
                clear_background(BLACK);

                ui.draw_score(&mut collision);
                ball.sprite();
                computer.sprite();
                player.sprite();

                // PAUSE MENU
                ui.draw_pause_menu(dt, &collision);

                // Resume game
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                // Title screen
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Title;
                    collision.reset(&mut ball, &mut player, &mut computer);
                }
            }

            GameState::GameWin => {
                ui.draw_game_over(true, dt);

                // Restart game
                if is_key_pressed(KeyCode::Space) {
                    collision.reset(&mut ball, &mut player, &mut computer);
                    ball.reset();
                    game_state = GameState::Playing;
                }

                // Return to title screen
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Title;
                }
            }

            GameState::GameOver => {
                ui.draw_game_over(false, dt);

                // Restart game
                if is_key_pressed(KeyCode::Space) {
                    collision.reset(&mut ball, &mut player, &mut computer);
                    ball.reset();
                    game_state = GameState::Playing;
                }

                // Return to menu
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Title;
                }
            }
        }

        next_frame().await;
    }
}
