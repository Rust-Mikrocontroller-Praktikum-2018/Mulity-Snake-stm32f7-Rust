#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
extern crate r0;
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

use alloc::Vec;
use graphics;
use stm32f7::{board, embedded, ethernet, lcd, sdram, system_clock, touch, i2c};

use super::WIDTH;
use super::HEIGHT;

const GRID_BLOCK_SIZE: usize = 10;

/**
 * Contains all necessary state information of a game.
 */
pub struct Game {
    graphics: graphics::Graphics,
    grid: Vec<Vec<Tile>>,
    i2c_3: stm32f7::i2c::I2C,
    touch: (),
    snake_head_position: (usize, usize),
    snake_body_position: (usize, usize),
    snake_tail_position: (usize, usize),
    former_snake_tail: (usize, usize),
}

/**
 * Possible tiles inside of the game grid.
 */
#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    SnakeHead(Direction),
    SnakeBody(Direction),
    SnakeTail(Direction),
    Apple,
}
#[derive(PartialEq, Clone)]
enum Direction {
    up,
    down,
    left,
    right,
}

impl Game {
    /**
     * Create a new game.
     */
    pub fn new(graphics: graphics::Graphics, i2c_3: stm32f7::i2c::I2C, touch: ()) -> Game {
        let game_width = WIDTH / GRID_BLOCK_SIZE;
        let game_height = HEIGHT / GRID_BLOCK_SIZE;
        let mut return_game = Game {
            graphics: graphics,
            grid: vec![vec![Tile::Empty; game_height]; game_width],
            i2c_3: i2c_3,
            touch: touch,
            snake_head_position: (25, 10),
            snake_body_position: (24, 10),
            snake_tail_position: (23, 10),
            former_snake_tail: (22, 10),
        };
        return_game.grid[25][10] = Tile::SnakeHead(Direction::right);
        return_game
    }

    /**
     * Draws current game state to screen.
     */
    pub fn draw_game(&mut self) {
        // draw head (bmp of head)
        self.graphics.print_square_size_color_at(
            self.snake_head_position.0 * GRID_BLOCK_SIZE,
            self.snake_head_position.1 * GRID_BLOCK_SIZE,
            GRID_BLOCK_SIZE,
            lcd::Color {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            },
        );
        // draw body (bmp of body)
        self.graphics.print_square_size_color_at(
            self.snake_body_position.0 * GRID_BLOCK_SIZE,
            self.snake_body_position.1 * GRID_BLOCK_SIZE,
            GRID_BLOCK_SIZE,
            lcd::Color {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            },
        );

        // draw tail (bmp of tail)
        self.graphics.print_square_size_color_at(
            self.snake_tail_position.0 * GRID_BLOCK_SIZE,
            self.snake_tail_position.1 * GRID_BLOCK_SIZE,
            GRID_BLOCK_SIZE,
            lcd::Color {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            },
        );

        // draw tail (bmp of tail)
        self.graphics.print_square_size_color_at(
            self.former_snake_tail.0 * GRID_BLOCK_SIZE,
            self.former_snake_tail.1 * GRID_BLOCK_SIZE,
            GRID_BLOCK_SIZE,
            lcd::Color {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 255,
            },
        );
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_up(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y]
                    == self.grid[self.snake_head_position.0][self.snake_head_position.1]
                {
                    self.grid[x][y - 1] = Tile::SnakeHead(Direction::up);
                    self.grid[x][y] = Tile::Empty;
                    self.former_snake_tail = self.snake_tail_position;
                    self.snake_tail_position = self.snake_body_position;
                    self.snake_body_position = self.snake_head_position;
                    self.snake_head_position.1 = y - 1;

                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_down(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y]
                    == self.grid[self.snake_head_position.0][self.snake_head_position.1]
                {
                    self.grid[x][y + 1] = Tile::SnakeHead(Direction::down);
                    self.grid[x][y] = Tile::Empty;
                    self.former_snake_tail = self.snake_tail_position;
                    self.snake_tail_position = self.snake_body_position;
                    self.snake_body_position = self.snake_head_position;
                    self.snake_head_position.1 = y + 1;

                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_right(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y]
                    == self.grid[self.snake_head_position.0][self.snake_head_position.1]
                {
                    self.grid[x + 1][y] = Tile::SnakeHead(Direction::right);
                    self.grid[x][y] = Tile::Empty;
                    self.former_snake_tail = self.snake_tail_position;
                    self.snake_tail_position = self.snake_body_position;
                    self.snake_body_position = self.snake_head_position;
                    self.snake_head_position.0 = x + 1;

                    return;
                }
            }
        }
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    pub fn move_left(&mut self) {
        for x in 0..self.grid.len() - 1 {
            for y in 0..self.grid[0].len() - 1 {
                if self.grid[x][y]
                    == self.grid[self.snake_head_position.0][self.snake_head_position.1]
                {
                    self.grid[x - 1][y] = Tile::SnakeHead(Direction::left);
                    self.grid[x][y] = Tile::Empty;
                    self.former_snake_tail = self.snake_tail_position;
                    self.snake_tail_position = self.snake_body_position;
                    self.snake_body_position = self.snake_head_position;
                    self.snake_head_position.0 = x - 1;

                    return;
                }
            }
        }
    }

    /**
     * Calls the correct function to turn to the right direction
     */

    pub fn turn_position(&mut self) {
        if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::up)
        {
            self.move_up();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::down)
        {
            self.move_down();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::left)
        {
            self.move_left();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::left)
        {
            self.move_right();
        }
    }
}
// /**
//  * Sets the direction chosen by the user
//  */

// pub fn choose_direction(&mut self) {

//         for touch in &self.touch::touches(&mut self.i2c_3).unwrap() {
//             let mut x = touch.x;
//             let mut y = touch.y;

//         if x > 10 && x < 70 {
//         self.move_down();
//         }
//         else if x > 410 && x < 470 {
//         self.move_up();
//         }

//         }
// }
