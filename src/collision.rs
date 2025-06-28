//Collision logic
use crate::ball::Ball;
use macroquad::audio::{Sound, play_sound_once};
use macroquad::prelude::*;
pub struct Collision {
    pub player_score: u32,
    pub comp_score: u32,
    pub hit_sound: Sound,
    pub score_sound: Sound,
}

impl Collision {
    pub fn new(hit_sound: Sound, score_sound: Sound) -> Self {
        Self { player_score: 0, comp_score: 0, hit_sound, score_sound }
    }

    pub fn score_update(&mut self, ball: &mut Ball) {


        if ball.rect.x + ball.rect.w < 0. {
            ball.reset();
            ball.speed += 20.;
            self.comp_score += 1;
            play_sound_once(&self.score_sound);
        }else if ball.rect.x > screen_width()  {
            ball.reset();
            ball.speed += 20.;
            self.player_score += 1;
            play_sound_once(&self.score_sound);
        }
    }

    pub fn physics(&self, ball: &mut Ball, obj: &Rect) -> bool {
        let towards = if obj.x > screen_width() / 2. { 1. } else { -1. };
        if ball.rect.overlaps(obj) && ball.vel.x.signum() == towards {
            if ball.vel.x > 0. {
                ball.rect.x = obj.x - ball.rect.w;

            }else {
                ball.rect.x = obj.x + obj.w;

            }
            ball.vel.x = -ball.vel.x;

            let obj_center = obj.y + obj.h / 2.;
            let ball_center = ball.rect.y + ball.rect.h / 2.;
            let offset = (ball_center - obj_center) / (obj.h / 2.);
            ball.vel.y = offset;

            ball.vel = ball.vel.normalize();

            play_sound_once(&self.hit_sound);
            true

        }else {
            false
        }

    }


}
