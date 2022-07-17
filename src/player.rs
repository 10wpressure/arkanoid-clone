use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;

pub(crate) struct Player {
    pub rect: Rect,
    pub lives: i32,
    pub score: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
            lives: 3,
            score: 0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, SKYBLUE);
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (
            is_key_down(KeyCode::Left),
            is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn reset(&mut self) {
        self.rect.x = screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32;
        self.rect.y = screen_height() - 100f32;
        self.lives = 3;
        self.score = 0;
    }
}