extern crate arrayvec;
extern crate r0;
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

use alloc::Vec;
use graphics;
use random;
use stm32f7::{lcd, system_clock, touch};

use super::HEIGHT;
use super::WIDTH;

const GRID_BLOCK_SIZE: usize = 10;

/**
 * Contains all necessary state information of a game.
 */
pub struct Game {
    pub graphics: graphics::Graphics,
    pub random_gen: random::Random,
    grid: Vec<Vec<Tile>>,
    i2c_3: stm32f7::i2c::I2C,
    pub snake_head_position: (usize, usize),
    pub snake_body_position: Vec<(usize, usize)>,
    snake_tail_position: (usize, usize),
    former_snake_tail: (usize, usize),
    apple_position: (usize, usize),
    pub apple_counter: usize,
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
    pub fn new(
        graphics: graphics::Graphics,
        i2c_3: stm32f7::i2c::I2C,
        random_gen: random::Random,
    ) -> Game {
        let game_width = WIDTH / GRID_BLOCK_SIZE;
        let game_height = HEIGHT / GRID_BLOCK_SIZE;
        let mut return_game = Game {
            graphics: graphics,
            random_gen: random_gen,
            grid: vec![vec![Tile::Empty; game_height]; game_width],
            i2c_3: i2c_3,
            snake_head_position: (24, 10),
            snake_body_position: vec![(23, 10), (22, 10)],
            snake_tail_position: (21, 10),
            former_snake_tail: (20, 10),
            apple_position: (1, 10),
            apple_counter: 0,
        };
        return_game.grid[25][10] = Tile::SnakeHead(Direction::right);
        return_game
    }

    /**
     * Draws a frame around the game around the field.
     */

    pub fn draw_frame(&mut self) {
        for i in GRID_BLOCK_SIZE - 2..WIDTH - GRID_BLOCK_SIZE {
            self.graphics.print_square_size_color_at(
                i,
                GRID_BLOCK_SIZE - 2,
                1,
                lcd::Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            );
        }
        for i in GRID_BLOCK_SIZE - 2..WIDTH - GRID_BLOCK_SIZE {
            self.graphics.print_square_size_color_at(
                i,
                HEIGHT - GRID_BLOCK_SIZE - 1,
                1,
                lcd::Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            );
        }
        for i in GRID_BLOCK_SIZE - 2..HEIGHT - GRID_BLOCK_SIZE {
            self.graphics.print_square_size_color_at(
                WIDTH - GRID_BLOCK_SIZE,
                i,
                1,
                lcd::Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            );
        }
        for i in GRID_BLOCK_SIZE - 2..HEIGHT - GRID_BLOCK_SIZE {
            self.graphics.print_square_size_color_at(
                GRID_BLOCK_SIZE - 2,
                i,
                1,
                lcd::Color {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 255,
                },
            );
        }
    }
    /**
     * Draws current game state to screen.
     */
    pub fn draw_game(&mut self) {
        // draw head (bmp of head)
        // Bmp
        let direction = &self.grid[self.snake_head_position.0][self.snake_head_position.1];
        let mut apple_offset = self.apple_position;
        let mut rot = self::graphics::RotDirection::R0;
        match direction {
            &Tile::SnakeHead(Direction::left) => {
                rot = self::graphics::RotDirection::R0;
                apple_offset = (self.apple_position.0 + 1, self.apple_position.1)
            }
            &Tile::SnakeHead(Direction::up) => {
                rot = self::graphics::RotDirection::R90;
                apple_offset = (self.apple_position.0, self.apple_position.1 + 1)
            }
            &Tile::SnakeHead(Direction::right) => {
                rot = self::graphics::RotDirection::R180;
                apple_offset = (self.apple_position.0 - 1, self.apple_position.1)
            }
            &Tile::SnakeHead(Direction::down) => {
                rot = self::graphics::RotDirection::R270;
                apple_offset = (self.apple_position.0, self.apple_position.1 - 1)
            }
            _ => {}
        }
        // println!("{}, {}, {}", self.snake_head_position.1, self.apple_position.1, apple_offset.1);

        if self.snake_head_position == apple_offset {
            self.graphics.print_bmp_at_with_rotaion(
                graphics::SNAKE_MOUTH_OPEN,
                (self.snake_head_position.0 * GRID_BLOCK_SIZE) as u32,
                (self.snake_head_position.1 * GRID_BLOCK_SIZE) as u32,
                rot,
            );
        } else {
            self.graphics.print_bmp_at_with_rotaion(
                graphics::SNAKE_MOUTH_CLOSED,
                (self.snake_head_position.0 * GRID_BLOCK_SIZE) as u32,
                (self.snake_head_position.1 * GRID_BLOCK_SIZE) as u32,
                rot,
            );
        }

        // self.graphics.print_square_size_color_at(
        //     self.snake_head_position.0 * GRID_BLOCK_SIZE,
        //     self.snake_head_position.1 * GRID_BLOCK_SIZE,
        //     GRID_BLOCK_SIZE - 1,
        //     lcd::Color {
        //         red: 0,
        //         green: 0,
        //         blue: 0,
        //         alpha: 255,
        //     },
        // );

        // draw body (bmp of body)
        for i in 0..self.snake_body_position.len() {
            self.graphics.print_square_size_color_at(
                self.snake_body_position[i].0 * GRID_BLOCK_SIZE,
                self.snake_body_position[i].1 * GRID_BLOCK_SIZE,
                GRID_BLOCK_SIZE - 1,
                lcd::Color {
                    red: 100,
                    green: 100,
                    blue: 100,
                    alpha: 255,
                },
            );
        }

        // draw tail (bmp of tail)
        self.graphics.print_square_size_color_at(
            self.snake_tail_position.0 * GRID_BLOCK_SIZE,
            self.snake_tail_position.1 * GRID_BLOCK_SIZE,
            GRID_BLOCK_SIZE - 1,
            lcd::Color {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            },
        );

        // erase former tail (bmp of tail)
        if self.former_snake_tail != (0, 0) {
            self.graphics.print_square_size_color_at(
                self.former_snake_tail.0 * GRID_BLOCK_SIZE,
                self.former_snake_tail.1 * GRID_BLOCK_SIZE,
                GRID_BLOCK_SIZE,
                lcd::Color {
                    red: 255,
                    green: 255,
                    blue: 255,
                    alpha: 0,
                },
            );
        }

        // draw apple (bmp of apple)
        // Bmp
        self.graphics.print_bmp_at_with_rotaion(
            graphics::APPLE_BMP,
            (self.apple_position.0 * GRID_BLOCK_SIZE) as u32,
            (self.apple_position.1 * GRID_BLOCK_SIZE) as u32,
            self::graphics::RotDirection::R0,
        )

        // Quadrat
        // self.graphics.print_square_size_color_at(
        //     self.apple_position.0 * GRID_BLOCK_SIZE,
        //     self.apple_position.1 * GRID_BLOCK_SIZE,
        //     GRID_BLOCK_SIZE - 1,
        //     lcd::Color {
        //         red: 255,
        //         green: 255,
        //         blue: 0,
        //         alpha: 255,
        //     },
        // );
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    fn move_up(&mut self) {
        let x = self.snake_head_position.0;
        let y = self.snake_head_position.1;

        self.grid[x][y - 1] = Tile::SnakeHead(Direction::up);
        self.grid[x][y] = Tile::Empty;
        self.former_snake_tail = self.snake_tail_position;
        self.snake_tail_position = self.snake_body_position[self.snake_body_position.len() - 1];
        for z in (0..self.snake_body_position.len() - 1).rev() {
            self.snake_body_position[z + 1] = self.snake_body_position[z];
        }
        self.snake_body_position[0] = self.snake_head_position;
        self.snake_head_position.1 = y - 1;

        return;
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    fn move_down(&mut self) {
        let x = self.snake_head_position.0;
        let y = self.snake_head_position.1;

        self.grid[x][y + 1] = Tile::SnakeHead(Direction::down);
        self.grid[x][y] = Tile::Empty;
        self.former_snake_tail = self.snake_tail_position;
        self.snake_tail_position = self.snake_body_position[self.snake_body_position.len() - 1];
        for z in (0..self.snake_body_position.len() - 1).rev() {
            self.snake_body_position[z + 1] = self.snake_body_position[z];
        }
        self.snake_body_position[0] = self.snake_head_position;
        self.snake_head_position.1 = y + 1;

        return;
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    fn move_right(&mut self) {
        let x = self.snake_head_position.0;
        let y = self.snake_head_position.1;

        self.grid[x + 1][y] = Tile::SnakeHead(Direction::right);
        self.grid[x][y] = Tile::Empty;
        self.former_snake_tail = self.snake_tail_position;
        self.snake_tail_position = self.snake_body_position[self.snake_body_position.len() - 1];
        for z in (0..self.snake_body_position.len() - 1).rev() {
            self.snake_body_position[z + 1] = self.snake_body_position[z];
        }
        self.snake_body_position[0] = self.snake_head_position;
        self.snake_head_position.0 = x + 1;

        return;
    }

    /**
     * Moves position of the snake in chosen direction.
     */
    fn move_left(&mut self) {
        let x = self.snake_head_position.0;
        let y = self.snake_head_position.1;

        self.grid[x - 1][y] = Tile::SnakeHead(Direction::left);
        self.grid[x][y] = Tile::Empty;
        self.former_snake_tail = self.snake_tail_position;
        self.snake_tail_position = self.snake_body_position[self.snake_body_position.len() - 1];
        for z in (0..self.snake_body_position.len() - 1).rev() {
            self.snake_body_position[z + 1] = self.snake_body_position[z];
        }
        self.snake_body_position[0] = self.snake_head_position;
        self.snake_head_position.0 = x - 1;

        return;
    }

    /**
     * Calls the correct function to turn to move the snake straight forward
     */

    pub fn move_straight(&mut self) {
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
            == Tile::SnakeHead(Direction::right)
        {
            self.move_right();
        }
    }

    /**
     * Calls the correct function to turn the snake to the right
     */

    pub fn turn_right(&mut self) {
        if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::up)
        {
            self.move_right();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::down)
        {
            self.move_left();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::left)
        {
            self.move_up();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::right)
        {
            self.move_down();
        }
    }

    /**
     * Calls the correct function to turn the snake to the left
     */

    pub fn turn_left(&mut self) {
        if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::up)
        {
            self.move_left();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::down)
        {
            self.move_right();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::left)
        {
            self.move_down();
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            == Tile::SnakeHead(Direction::right)
        {
            self.move_up();
        }
    }

    /**
     * Sets the direction chosen by the user
     */
    pub fn move_snake(&mut self) {
        let touches = self.get_touches();
        if touches.len() == 1 {
            let mut touch = touches[0];
            let mut x = touch.0;
            let mut y = touch.1;

            if x < 100 {
                self.turn_left();
            } else if x > 380 {
                self.turn_right();
            } else if x > 100 && x < 380 {
                self.pause_game();
            }
        } else {
            self.move_straight();
        }
    }

    /**
     * checks if snake bites
     */
    pub fn snake_bite(&mut self) {
        if self.snake_head_position == self.apple_position {
            self.snake_body_position.push(self.snake_tail_position);
            self.snake_tail_position = self.former_snake_tail;
            self.former_snake_tail = (0, 0); // has to be improved
            let x = self.random_gen
                .random_range(1, WIDTH as u32 / GRID_BLOCK_SIZE as u32 - 1);
            let y = self.random_gen
                .random_range(1, HEIGHT as u32 / GRID_BLOCK_SIZE as u32 - 1);
            self.apple_position = (x as usize, y as usize);
            self.apple_counter = self.apple_counter + 1;
        }
    }

    /**
     * returns touches array
     */
    pub fn get_touches(&mut self) -> Vec<(u16, u16)> {
        // &touch::touches(&mut self.i2c_3).unwrap()
        let mut touches = Vec::new();
        for touch in &touch::touches(&mut self.i2c_3).unwrap() {
            // .print_point_at(touch.x as usize, touch.y as usize);
            touches.push((touch.x, touch.y));
        }
        touches
    }

    /**
     * returns touches array
     */
    pub fn check_grid_edge(&mut self) {
        if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            != Tile::SnakeHead(Direction::left)
            && self.snake_head_position.0 == WIDTH / GRID_BLOCK_SIZE - 1
        {
            self.grid[self.snake_head_position.0][self.snake_head_position.1] = Tile::Empty;
            self.snake_head_position.0 = 1;
            self.grid[self.snake_head_position.0][self.snake_head_position.1] =
                Tile::SnakeHead(Direction::right);
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            != Tile::SnakeHead(Direction::right)
            && self.snake_head_position.0 == 0
        {
            self.grid[self.snake_head_position.0][self.snake_head_position.1] = Tile::Empty;
            self.snake_head_position.0 = WIDTH / GRID_BLOCK_SIZE - 2;
            self.grid[self.snake_head_position.0][self.snake_head_position.1] =
                Tile::SnakeHead(Direction::left);
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            != Tile::SnakeHead(Direction::down) && self.snake_head_position.1 == 0
        {
            self.grid[self.snake_head_position.0][self.snake_head_position.1] = Tile::Empty;
            self.snake_head_position.1 = HEIGHT / GRID_BLOCK_SIZE - 2;

            self.grid[self.snake_head_position.0][self.snake_head_position.1] =
                Tile::SnakeHead(Direction::up);
        } else if self.grid[self.snake_head_position.0][self.snake_head_position.1]
            != Tile::SnakeHead(Direction::up)
            && self.snake_head_position.1 == HEIGHT / GRID_BLOCK_SIZE - 1
        {
            self.grid[self.snake_head_position.0][self.snake_head_position.1] = Tile::Empty;
            self.snake_head_position.1 = 1;
            self.grid[self.snake_head_position.0][self.snake_head_position.1] =
                Tile::SnakeHead(Direction::down);
        }
    }
    /**
     * Set backround color
     */
    pub fn set_backround_color(&mut self) {
        for x in 5..WIDTH - 5 {
            for y in 5..HEIGHT - 5 {
                self.graphics.print_square_size_color_at(
                    x,
                    y,
                    1,
                    lcd::Color {
                        red: 209,
                        green: 115,
                        blue: 28,
                        alpha: 255,
                    },
                );
            }
        }
    }

    fn pause_game(&mut self) {
        self.graphics.print_pause_screen();
        println!("     score: {}", self.apple_counter);
        let mut pause = true;
        let mut new_game = false;
        loop {
            let touches = self.get_touches();

            if touches.len() == 1 {
                for touch in touches {
                    let mut x = touch.0;
                    let mut y = touch.1;

                    if (x > 100 + 8 + 90 && x < 100 + 8 + 90 + 100)
                        && (y > 6 + 139 && y < 6 + 139 + 30)
                    {
                        pause = false;
                        break;
                    }
                    if (x > 100 + 8 + 78 && x < 100 + 8 + 90 + 120)
                        && (y > 6 + 192 && y < 6 + 192 + 30)
                    {
                        new_game = true;
                        break;
                    }
                }
            }
            if !pause {
                self.graphics.layer_2.clear();
                break;
            }
            if new_game {
                self.graphics.layer_2.clear();
                self.graphics.layer_1.clear();
                break;
            }
        }
        if new_game {
            self.reset();
        }
    }

    fn restart_game(&mut self) {
        self.graphics.background_blink();

        self.graphics.print_restart_screen();
        println!("   score: {}", self.apple_counter);
        self.apple_counter = 0;
        let mut pause = true;
        let mut new_game = false;
        loop {
            let touches = self.get_touches();

            if touches.len() == 1 {
                for touch in touches {
                    let mut x = touch.0;
                    let mut y = touch.1;

                    if (x > 100 + 8 + 90 && x < 100 + 8 + 90 + 100)
                        && (y > 6 + 139 && y < 6 + 139 + 30)
                    {
                        pause = false;
                        break;
                    }
                    if (x > 100 + 8 + 78 && x < 100 + 8 + 90 + 120)
                        && (y > 6 + 192 && y < 6 + 192 + 30)
                    {
                        new_game = true;
                        break;
                    }
                }
            }
            if !pause {
                self.graphics.layer_2.clear();
                break;
            }
            if new_game {
                self.graphics.layer_2.clear();
                self.graphics.layer_1.clear();
                break;
            }
        }
        if new_game {
            self.reset();
        }
    }
    pub fn game_start_up(&mut self) {
        self.graphics.print_bmp_at_with_rotaion(
            self::graphics::WELCOME_SCREEN_BASE,
            0,
            0,
            graphics::RotDirection::R0,
        );

        let welcome = "Welcome to Mulity-Snake! Touch screen to start the game";

        for c in welcome.chars() {
            if c == ' ' || c == '-' || c == '!' {
                println!("{}", c);
            // system_clock::wait(10);
            } else {
                self.graphics.print_bmp_at_downwards(
                    self::graphics::WELCOME_SCREEN_OPEN_MOUTH,
                    188,
                    85,
                );

                print!("{}", c);
                system_clock::wait(10);
                self.graphics.print_bmp_at_downwards(
                    self::graphics::WELCOME_SCREEN_CLOSED_MOUTH,
                    188,
                    85,
                );
            }
        }
        loop {
            let touches = self.get_touches();
            let mut pause = true;
            if touches.len() == 1 {
                self.graphics.layer_1.clear();
                self.graphics.layer_2.clear();
                break;
            }
        }
    }
    pub fn reset(&mut self) {
        let game_width = WIDTH / GRID_BLOCK_SIZE;
        let game_height = HEIGHT / GRID_BLOCK_SIZE;
        self.grid = vec![vec![Tile::Empty; game_height]; game_width];
        self.grid[25][10] = Tile::SnakeHead(Direction::right);
        self.snake_head_position = (25, 10);
        self.snake_body_position = vec![(24, 10), (23, 10), (22, 10)];
        self.snake_tail_position = (21, 10);
        self.former_snake_tail = (20, 10);
        self.apple_position = (1, 10);
        self.draw_frame();
    }

    pub fn check_selfbite(&mut self) {
        for i in 0..self.snake_body_position.len() {
            if self.snake_head_position == self.snake_body_position[i] {
                self.restart_game();
                return;
            }
        }
        if self.snake_head_position == self.snake_tail_position {
            self.restart_game();
        }
    }

    pub fn return_wait_tick(&mut self) -> usize {

        let mut tick:usize = 100;
        
        if let Some(new) = tick.checked_sub(self.apple_counter * 5) {
        
           return new
        }
        tick
    }
}

