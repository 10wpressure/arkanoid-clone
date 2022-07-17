mod ball;
mod block;
mod player;
mod game;
mod collision;
mod text;

use macroquad::prelude::*;
use crate::ball::{Ball, BALL_SIZE};
use crate::block::{Block, BlockType};
use crate::player::Player;
use crate::collision::resolve_collision;
use crate::game::{GameState};
use crate::text::GameText;

fn init_blocks(blocks: &mut Vec<Block>) {
    // initialize level settings
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = Block::size() + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);

    // create blocks
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), BlockType::Regular));
    }

    // choose spawn ball blocks
    for _ in 0..2 {
        let spawn_index = rand::gen_range(0, blocks.len());
        blocks[spawn_index].block_type = BlockType::SpawnBallOnDeletion;
    }

    // // choose decrease ball speed blocks
    // for _ in 0..2 {
    //     let spawn_index = rand::gen_range(0, blocks.len());
    //     blocks[spawn_index].block_type = BlockType::DecreaseBallSpeed;
    // }
    //
    // // choose increase ball size blocks
    // for _ in 0..2 {
    //     let spawn_index = rand::gen_range(0, blocks.len());
    //     blocks[spawn_index].block_type = BlockType::IncreaseBallSize;
    // }
}

fn reset_game(blocks: &mut Vec<Block>, balls: &mut Vec<Ball>, player: &mut Player) {
    *player = Player::new();
    balls.clear();
    blocks.clear();
    init_blocks(blocks);
    create_ball(balls);
}

fn create_ball(balls: &mut Vec<Ball>) {
    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    )));
}


#[macroquad::main("arkanoid")]
async fn main() {
    // create game objects
    let font = load_ttf_font("res/game-over.regular.ttf").await.unwrap();

    let mut game_state = GameState::Menu;
    let mut player = Player::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut balls: Vec<Ball> = Vec::new();
    let text = GameText::new(font);

    init_blocks(&mut blocks);

    loop {
        match game_state {
            GameState::Menu => {
                clear_background(BLACK);
                text.draw_title_text("Press SPACE to start", WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::GameIsRunning;
                    clear_background(WHITE);
                    reset_game(&mut blocks, &mut balls, &mut player)
                }
            }
            GameState::GameIsRunning => {
                clear_background(WHITE);

                // draw player
                player.draw();
                // draw blocks
                for block in blocks.iter() {
                    block.draw();
                }
                // draw balls
                for ball in balls.iter_mut() {
                    ball.draw();
                }
                // draw UI
                text.draw_score_text(player.score);
                text.draw_lives_text(player.lives);

                // // spawn another ball
                // if is_key_pressed(KeyCode::Space) {
                //     balls.push(Ball::new(vec2(
                //         screen_width() * 0.5f32,
                //         screen_height() * 0.5f32,
                //     )));
                // }

                // player animation
                player.update(get_frame_time());

                // balls animation
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                // collision detection
                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                player.score += 10;
                                if block.block_type == BlockType::SpawnBallOnDeletion {
                                    spawn_later.push(Ball::new(ball.rect.point()));
                                }
                            }
                        }
                    }
                }
                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }

                // block deletion
                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());

                // balls deletion, player lives check
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player.lives -= 1;
                    balls.push(Ball::new(player.rect.point() + vec2(
                        player.rect.w*0.5f32 + BALL_SIZE * 0.5f32, -50f32)))
                }

                // game over condition
                if player.lives <= 0 {
                    player.lives = 0;
                    game_state = GameState::GameOver;
                }

                // victory condition
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
                blocks.retain(|block| block.lives > 0);
            }
            GameState::LevelCompleted => {
                clear_background(WHITE);
                text.draw_title_text(&format!("You WON! {} score", player.score), BLACK);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    player.reset();
                }
            }
            GameState::GameOver => {
                clear_background(WHITE);
                text.draw_title_text(&format!("You LOST! {} score", player.score), BLACK);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    player.reset();
                }
            }
        }
        next_frame().await;
    }
}
