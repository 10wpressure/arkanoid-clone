use macroquad::prelude::*;

pub(crate) const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 40f32]);

#[derive(PartialEq, Eq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeletion,
    // IncreaseBallSize,
    // DecreaseBallSpeed,
}

pub(crate) struct Block {
    pub rect: Rect,
    pub _size: Vec2,
    pub lives: i32,
    pub block_type: BlockType,
}

impl Block {
    pub fn size() -> Vec2 {
        BLOCK_SIZE
    }

    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            _size: BLOCK_SIZE,
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => {
                match self.lives {
                    2 => RED,
                    _ => ORANGE,
                }
            }
            BlockType::SpawnBallOnDeletion => GREEN,
            // BlockType::DecreaseBallSpeed => PURPLE,
            // BlockType::IncreaseBallSize => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
