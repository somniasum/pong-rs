use macroquad::prelude::*;

use crate::ball::Ball;
const COMP_SIZE: Vec2 = vec2(40., 150.);

pub struct Computer {
    pub rect: Rect,
    pub max_speed: f32,
    pub reaction: f32,
    x_off: f32,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() - COMP_SIZE.x - 22.,
                100.0,
                COMP_SIZE.x,
                COMP_SIZE.y,
            ),
            max_speed: 250.,
            reaction: 200.,
            x_off: 20.,

        }
    }
    pub fn computer_movement(&mut self,ball: &Ball, dt: f32) {

        // x-axis calc
        let ball_center = ball.rect.y + ball.rect.h * 0.5 ;
        let comp_x = self.rect.x ;
        let dx = comp_x - (ball.rect.x + ball.rect.w / 2.);

        // if ball is away calc the y
        let target_y = if ball.vel.x.signum() * dx > 0. {
            ball_center
        }else {

            // t = distance / horizontal speed
            let t = dx.abs() / ball.vel.x.abs().max(1.);
            let mut future_y = ball_center + ball.vel.y * t;


            // bounce prediction
            let screen_h = screen_height() - ball.rect.h ;
            //invert direction
            if future_y < 0. || future_y > screen_h {
                let overflow = if future_y < 0. {
                    -future_y
                }else {
                    future_y - screen_h
                };
                future_y = if future_y < 0.{
                    overflow
                }else {
                    screen_h - overflow
                };
            }
            future_y
        };

            // calc frame distance
            let comp_center = self.rect.y + self.rect.h / 2.;
            let delta = (target_y - comp_center) * self.reaction;

            //clamp max speed
            let move_amount = delta.clamp(-self.max_speed, self.max_speed) * dt;
            self.rect.y += move_amount;

            // bound to screen
            self.rect.y = self.rect.y.clamp(0., screen_height() - self.rect.h);
    }

    pub fn update_pos(&mut self) {
        self.rect.x = screen_width() - COMP_SIZE.x - self.x_off;
    }

    pub fn sprite(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            LIGHTGRAY,
        );
    }
}
