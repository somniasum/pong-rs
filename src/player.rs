use macroquad::prelude::*;

pub const PLAYER_SIZE: Vec2 = vec2(40., 150.);

// Player Info
pub struct Player {
    pub rect: Rect,
    pub x_off: f32,
    pub speed: f32,
    pub vel_y: f32,
    prev_y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() - PLAYER_SIZE.x * 20.,
                100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            speed: 500.,
            x_off: 20.,
            vel_y: 0.,
            prev_y: 100.0,
        }
    }

    pub fn player_movement(&mut self, dt: f32) {
        self.prev_y = self.rect.y;

        // Player movement
        let move_y = match (
            is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
            is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        ) {
            (false, true) => -1.,
            (true, false) => 1.,
            _ => 0.,
        };
        self.rect.y += move_y * dt * self.speed;

        if self.rect.y < 0. {
            self.rect.y = 0.;
        }

        if self.rect.y > screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h;
        }

        // Calculate velocity
        if dt > 0. {
            self.vel_y = (self.rect.y - self.prev_y) / dt;
        }
    }

    pub fn update_pos(&mut self) {
        self.rect.x = self.x_off;
    }

    pub fn sprite(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
    }
}
