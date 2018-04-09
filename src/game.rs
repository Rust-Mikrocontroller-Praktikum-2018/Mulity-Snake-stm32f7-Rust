use alloc::Vec;
use graphics;
use lcd;

use super::WIDTH;
use super::HEIGHT;

const GRID_BLOCK_SIZE: usize = 10;

/**
 * Contains all necessary state information of a game.
 */
pub struct Game {
    graphics: graphics::Graphics,
    grid: Vec<Vec<Tile>>
}

/**
 * Possible tiles inside of the game grid.
 */
#[derive(PartialEq)]
#[derive(Clone)]
enum Tile {
    empty,
    snake,
    apple,
}

impl Game {
    /**
     * Create a new game.
     */
    pub fn new(graphics: graphics::Graphics) -> Game {
        let game_width = WIDTH / GRID_BLOCK_SIZE;
        let game_height = HEIGHT / GRID_BLOCK_SIZE;
        let mut return_game = Game {
            graphics: graphics,
            grid: vec![vec![Tile::empty; game_height]; game_width]
        };
        return_game.grid[20][20] = Tile::snake;
        return_game
    }

    /**
     * Draws current game state to screen.
     */
    pub fn draw_game(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                if self.grid[x][y] == Tile::snake {
                    self.graphics.print_square_size_color_at(x*GRID_BLOCK_SIZE, y*GRID_BLOCK_SIZE, GRID_BLOCK_SIZE, lcd::Color {red:255, green:0, blue:0, alpha: 255});
                }
            }
        }
    }
}
