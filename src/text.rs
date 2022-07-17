use macroquad::prelude::*;

pub struct GameText {
    pub font: Font,
}

impl GameText {
    pub fn new(font: Font) -> GameText {
        Self { font }
    }

    pub fn draw_title_text(&self, text: &str, color: Color) {
        let font = self.font;
        let dims = measure_text(text, Some(font), 50u16, 1.0f32);
        draw_text_ex(
            text,
            screen_width() * 0.5f32 - dims.width * 0.5f32,
            screen_height() * 0.5f32 - dims.height * 0.5f32,
            TextParams { font, font_size: 50u16, color, ..Default::default() },
        )
    }

    pub fn draw_score_text(&self, score: i32) {
        let font = self.font;
        let score_text = format!("score: {}", score);
        let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);

        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            40.0,
            TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() });
    }

    pub fn draw_lives_text(&self, lives: i32) {
        let font = self.font;
        let lives_text = format!("lives: {}", lives);
        draw_text_ex(
            &lives_text,
            30.0,
            40.0,
            TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() });
    }
}

