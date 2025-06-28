//Collision logic
use crate::ball::Ball;
use macroquad::audio::{Sound, play_sound_once};
use macroquad::prelude::*;
pub struct Collision {
    pub player_score: u32,
    pub comp_score: u32,
    pub sound: Sound,
}

impl Collision {
    pub fn new(sound: Sound) -> Self {
        Self { player_score: 0, comp_score: 0, sound }
    }

    pub fn score_update(&mut self, ball: &mut Ball) {
        if ball.rect.x < 0. {
            self.comp_score += 1;
            ball.reset();
        }else if ball.rect.x > screen_width() - ball.rect.w {
            self.player_score += 1;
            ball.reset();
        }
    }

    pub fn physics(&self, ball: &mut Ball, obj: &Rect) -> bool {
        if ball.rect.overlaps(obj) {
            ball.vel.x *= -1.;

            let obj_center = obj.y + obj.h / 2.;
            let ball_center = ball.rect.y + ball.rect.h / 2.;
            let offset = (ball_center - obj_center) / (obj.h / 2.);
            ball.vel.y = offset;

            ball.vel = ball.vel.normalize();

            true
        } else {
            false
        }
    }

    pub fn audio(&self, ball: &mut Ball, obj: &Rect) {
        if Collision::physics(&self, ball, obj) {
            if true {
                play_sound_once(&self.sound);
                Collision::physics(&self, ball, obj);
            }
        }
    }
}
