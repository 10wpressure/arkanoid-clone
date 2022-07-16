use macroquad::prelude::*;

pub(crate) const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 40f32]);

pub(crate) struct Block {
    rect: Rect,
    size: Vec2,
    lives: i32,
}

impl Block {
    pub fn size() -> Vec2 {
        BLOCK_SIZE
    }

    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            size: BLOCK_SIZE,
            lives: 1,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}
