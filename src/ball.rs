use macroquad::{
    audio::{Sound, play_sound_once},
    prelude::*,
};
const BALL_SIZE: Vec2 = vec2(40., 40.);

pub struct Ball {
    pub rect: Rect,
    pub vel: Vec2,
    pub speed: f32,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE.x, BALL_SIZE.y),

            speed: 600.,
            vel: vec2(rand::gen_range(-100., 100.), rand::gen_range(-20., -13.)).normalize(),
        }
    }

    pub fn ball_movement(&mut self, dt: f32, sound: &mut Sound) {
        self.rect.y += self.vel.y * dt * self.speed;
        self.rect.x += self.vel.x * dt * self.speed;

        //top and bottom collision
        let ball_top = self.rect.y;
        let ball_bottom = self.rect.y + self.rect.h;

        if ball_top <= 0. && self.vel.y < 0. {
            self.vel.y *= -1.;
            self.rect.y = 0.; // snap inside
            play_sound_once(sound);
        }
        if ball_bottom >= screen_height() && self.vel.y > 0. {
            self.vel.y *= -1.;
            self.rect.y = screen_height() - self.rect.h;
            play_sound_once(sound);
        }
    }

    pub fn reset(&mut self) {
        self.rect.x = screen_width() * 0.5;
        self.rect.y = screen_height() * 0.5;
        self.vel = vec2(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize();
    }
    pub fn sprite(&self) {
        draw_circle(self.rect.x, self.rect.y, BALL_SIZE.y / 2., RED);
    }
}
