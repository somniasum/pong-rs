use macroquad::prelude::*;

use crate::collision::*;

pub struct Info {}

impl Info {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw_score(&mut self, collision: &mut Collision) {
        let score_text_comp = format!("COMP score: {}", collision.comp_score);
        let score_text_player =  format!("P1 score: {}", collision.player_score);
        draw_text_ex(
            &score_text_comp,
            screen_width() * 0.5 + 300. * 0.5,
            40.,
            TextParams {
                font_size: 30,
                color: WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
                    &score_text_player,
                    screen_width() * 0.5 - 600. * 0.5,
                    40.,
                    TextParams {
                        font_size: 30,
                        color: WHITE,
                        ..Default::default()
                    },
                );
    }
}
