use crate::collision::Collision;
use macroquad::prelude::*;

pub struct Info {
    blink_timer: f32,
}

impl Info {
    pub fn new() -> Self {
        Self { blink_timer: 0. }
    }

    // Main menu
    pub fn draw_main_menu(&mut self, dt: f32) {
        clear_background(BLACK);

        self.blink_timer += dt;

        let title = "PONG";
        let title_size = 120;
        let title_dims = measure_text(title, None, title_size, 1.0);

        draw_text_ex(
            title,
            screen_width() / 2. - title_dims.width / 2.,
            screen_height() * 0.25,
            TextParams {
                font_size: title_size,
                color: WHITE,
                ..Default::default()
            },
        );

        let paddding = screen_height() * 0.45;

        let blink = ((self.blink_timer * 3.).sin() * 0.5 + 0.5).clamp(0.4, 1.0);
        let prompt = "[ SPACE ]";
        let prompt_size = 32;
        let prompt_dims = measure_text(prompt, None, prompt_size, 1.0);
        draw_text_ex(
            prompt,
            screen_width() / 2. - prompt_dims.width / 2.,
            paddding + 80.,
            TextParams {
                font_size: prompt_size,
                color: Color::new(255., 255., 255., blink),
                ..Default::default()
            },
        );
    }

    /// PAUSE MENU
    pub fn draw_pause_menu(&mut self, dt: f32, collision: &Collision) {
        self.blink_timer += dt;

        draw_rectangle(
            0.,
            0.,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 180),
        );

        // Pause menu panel
        let panel_width = 500.;
        let panel_height = 400.;
        let panel_x = screen_width() / 2. - panel_width / 2.;
        let panel_y = screen_height() / 2. - panel_height / 2.;

        // Panel background
        draw_rectangle(panel_x, panel_y, panel_width, panel_height, BLACK);

        let title = "GAME PAUSED";
        let title_size = 48;
        let title_dims = measure_text(title, None, title_size, 1.0);
        draw_text_ex(
            title,
            screen_width() / 2. - title_dims.width / 2.,
            panel_y + 70.,
            TextParams {
                font_size: title_size,
                color: WHITE,
                ..Default::default()
            },
        );

        // Current score
        let score_text = format!("{}  -  {}", collision.player_score, collision.comp_score);
        let score_size = 28;
        let score_dims = measure_text(&score_text, None, score_size, 1.0);
        draw_text_ex(
            &score_text,
            screen_width() / 2. - score_dims.width / 2.,
            panel_y + 120.,
            TextParams {
                font_size: score_size,
                color: GRAY,
                ..Default::default()
            },
        );

        // Menu options
        let options = vec![
            ("[ SPACE ]", "RESUME GAME", WHITE),
            ("[ ESC ]", "TITLE SCREEN", WHITE),
        ];

        let option_size = 22;
        let start_y = panel_y + 190.;

        for (i, (key, text, color)) in options.iter().enumerate() {
            let y = start_y + (i as f32 * 50.);
            let is_primary = i == 0;

            // Key button
            let key_width = 100.;
            let key_x = screen_width() / 2. - 120.;

            let key_color = if is_primary { BLACK } else { BLACK };

            draw_rectangle(key_x, y - 20., key_width, 35., key_color);

            let key_dims = measure_text(key, None, option_size, 1.0);
            draw_text_ex(
                key,
                key_x + key_width / 2. - key_dims.width / 2.,
                y,
                TextParams {
                    font_size: option_size,
                    color: *color,
                    ..Default::default()
                },
            );

            draw_text_ex(
                text,
                key_x + key_width + 20.,
                y,
                TextParams {
                    font_size: option_size,
                    color: LIGHTGRAY,
                    ..Default::default()
                },
            );
        }
    }

    // Score
    pub fn draw_score(&self, collision: &mut Collision) {
        let score_text = format!("{}   {}", collision.player_score, collision.comp_score);
        let size = 60;
        let dims = measure_text(&score_text, None, size, 1.0);

        draw_text_ex(
            &score_text,
            screen_width() / 2. - dims.width / 2.,
            60.,
            TextParams {
                font_size: size,
                color: GRAY,
                ..Default::default()
            },
        );

        let dash_height = 15.;
        let dash_gap = 10.;
        let mut y = 0.;
        while y < screen_height() {
            draw_rectangle(
                screen_width() / 2. - 2.,
                y,
                4.,
                dash_height,
                Color::new(1., 1., 1., 0.2),
            );
            y += dash_height + dash_gap;
        }
    }

    // Game win/game over
    pub fn draw_game_over(&mut self, won: bool, dt: f32) {
        clear_background(BLACK);

        self.blink_timer += dt;

        let title = if won { "YOU WIN!" } else { "GAME OVER" };
        let title_color = if won { WHITE } else { GRAY };
        let title_size = 80;
        let title_dims = measure_text(title, None, title_size, 1.0);

        draw_text_ex(
            title,
            screen_width() / 2. - title_dims.width / 2. + 3.,
            screen_height() * 0.4 + 3.,
            TextParams {
                font_size: title_size,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            title,
            screen_width() / 2. - title_dims.width / 2.,
            screen_height() * 0.4,
            TextParams {
                font_size: title_size,
                color: title_color,
                ..Default::default()
            },
        );

        let blink = ((self.blink_timer * 3.).sin() * 0.5 + 0.5).clamp(0.3, 1.0);
        let prompt = "[ SPACE ]";
        let prompt_size = 30;
        let prompt_dims = measure_text(prompt, None, prompt_size, 1.0);
        draw_text_ex(
            prompt,
            screen_width() / 2. - prompt_dims.width / 2.,
            screen_height() * 0.6,
            TextParams {
                font_size: prompt_size,
                color: Color::new(255., 255., 255., blink),
                ..Default::default()
            },
        );

        // ESC to title screen
        let menu_prompt = "[ ESC ] TITLE SCREEN";
        let menu_size = 20;
        let menu_dims = measure_text(menu_prompt, None, menu_size, 1.0);
        draw_text_ex(
            menu_prompt,
            screen_width() / 2. - menu_dims.width / 2.,
            screen_height() * 0.7,
            TextParams {
                font_size: menu_size,
                color: GRAY,
                ..Default::default()
            },
        );
    }
}
