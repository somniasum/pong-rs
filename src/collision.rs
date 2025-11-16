use crate::ball::Ball;
use crate::computer::Computer;
use crate::player::Player;

use macroquad::audio::{Sound, play_sound_once};
use macroquad::prelude::*;

pub struct Collision {
    pub player_score: u32,
    pub comp_score: u32,
    pub hit_sound: Sound,
    pub score_sound: Sound,
    pub paddle_influence: f32,
}

impl Collision {
    pub fn new(hit_sound: Sound, score_sound: Sound) -> Self {
        Self {
            player_score: 0,
            comp_score: 0,
            hit_sound,
            score_sound,
            paddle_influence: 0.2,
        }
    }

    pub fn score_update(&mut self, ball: &mut Ball, player: &mut Player, comp: &mut Computer) {
        if ball.rect.x + ball.rect.w < 0. {
            ball.reset();
            self.speed_update(ball, player, comp);
            self.comp_score += 1;
            play_sound_once(&self.score_sound);
        } else if ball.rect.x > screen_width() {
            ball.reset();
            self.speed_update(ball, player, comp);
            self.player_score += 1;
            play_sound_once(&self.score_sound);
        }
    }

    pub fn speed_update(&self, ball: &mut Ball, player: &mut Player, comp: &mut Computer) {
        ball.speed += 50.;
        player.speed += 20.;
        comp.max_speed += 20.;
        comp.reaction += 20.;
    }

    // Trigonometric collision
    pub fn physics(&self, ball: &mut Ball, obj: &Rect, paddle_vel_y: f32) -> bool {
        let towards = if obj.x > screen_width() / 2. { 1. } else { -1. };

        if ball.rect.overlaps(obj) && ball.vel.x.signum() == towards {
            // Position correction
            if ball.vel.x > 0. {
                ball.rect.x = obj.x - ball.rect.w;
            } else {
                ball.rect.x = obj.x + obj.w;
            }

            let obj_center = obj.y + obj.h / 2.;
            let ball_center = ball.rect.y + ball.rect.h / 2.;
            let hit_offset = ((ball_center - obj_center) / (obj.h / 2.)).clamp(-1., 1.);

            //  Reflection angle

            let reflected_x = -ball.vel.x;
            let reflected_y = ball.vel.y;

            let base_angle = reflected_y.atan2(reflected_x.abs());

            let max_deflection_angle = std::f32::consts::PI / 5.;
            let position_influence = hit_offset * max_deflection_angle;

            // Paddle velocity influence
            let velocity_influence = if paddle_vel_y.abs() > 0.1 {
                // 15 degrees
                let max_vel_angle = std::f32::consts::PI / 12.;
                let normalized_vel = (paddle_vel_y / 500.).clamp(-1., 1.);
                normalized_vel * max_vel_angle * self.paddle_influence
            } else {
                0.
            };

            let final_angle = base_angle + position_influence + velocity_influence;

            // 68 degrees
            let max_angle = std::f32::consts::PI * 0.38;
            let clamped_angle = final_angle.clamp(-max_angle, max_angle);

            // Velocity vector
            let speed_x = clamped_angle.cos();
            let speed_y = clamped_angle.sin();

            if towards > 0. {
                // Computer
                ball.vel.x = -speed_x.abs();
            } else {
                // Player
                ball.vel.x = speed_x.abs();
            }
            ball.vel.y = speed_y;

            // Normalization
            ball.vel = ball.vel.normalize();

            play_sound_once(&self.hit_sound);
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self, ball: &mut Ball, player: &mut Player, comp: &mut Computer) {
        self.player_score = 0;
        self.comp_score = 0;
        ball.speed = 600.;
        player.speed = 500.;
        comp.max_speed = 250.;
        comp.reaction = 200.;
    }
}
