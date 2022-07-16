mod ball;
mod block;
mod player;

use crate::ball::Ball;
use crate::block::Block;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("arkanoid")]
async fn main() {
    // create game objects
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();

    // initialize level settings
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = Block::size() + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );

    // create blocks
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    // create balls
    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    )));

    loop {
        if is_key_down(KeyCode::Space) {
            balls.push(Ball::new(vec2(
                screen_width() * 0.5f32,
                screen_height() * 0.5f32,
            )));
        }

        player.update(get_frame_time());

        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter_mut() {
            ball.draw();
        }
        next_frame().await;
    }
}
