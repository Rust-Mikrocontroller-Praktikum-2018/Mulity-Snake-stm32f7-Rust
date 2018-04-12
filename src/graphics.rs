use stm32f7::{lcd, system_clock};

pub struct Graphics {
    lcd: lcd::Lcd,
    pub layer_1: lcd::Layer<lcd::FramebufferArgb8888>,
    pub layer_2: lcd::Layer<lcd::FramebufferAl88>,
}

pub enum RotDirection {
    r_0,
    r_90,
    r_180,
    r_270,
}

pub const pause_screen_right: &[u8] = include_bytes!("../assets/Pause_screen_snake_left.bmp");
pub const pause_screen_left: &[u8] = include_bytes!("../assets/Pause_screen_snake_right.bmp");
pub const pause_screen_game_over: &[u8] = include_bytes!("../assets/Pause_screen_game_over.bmp");
pub const pause_screen_pause: &[u8] = include_bytes!("../assets/Pause_screen_pause.bmp");
pub const pause_screen_resume: &[u8] = include_bytes!("../assets/Pause_screen_resume.bmp");
pub const pause_screen_new_game: &[u8] = include_bytes!("../assets/Pause_screen_New_game.bmp");
pub const welcome_screen_base: &[u8] = include_bytes!("../assets/Welcom_screen/Snake_base2.bmp");
pub const welcome_screen_open_mouth: &[u8] =
    include_bytes!("../assets/Welcom_screen/Snake_mouth_open.bmp");
pub const welcome_screen_closed_mouth: &[u8] =
    include_bytes!("../assets/Welcom_screen/Snake_mouth_shut.bmp");
pub const apple_bmp: &[u8] = include_bytes!("../assets/apple.bmp");
pub const snake_mouth_closed: &[u8] = include_bytes!("../assets/snake_head_closed.bmp");
pub const snake_mouth_open: &[u8] = include_bytes!("../assets/snake_head_opened.bmp");

impl Graphics {
    /**
     * Needs to be called first. Initialize Graphics.\n
     * example for lcd: `lcd::init(ltdc, rcc, &mut gpio)`
     */
    pub fn new(mut lcd: lcd::Lcd) -> Graphics {
        // get layer 1+2
        let layer_1 = lcd.layer_1().unwrap();
        let layer_2 = lcd.layer_2().unwrap();
        let layer_2_copy = lcd.layer_2().unwrap();

        // set stdout to layer_2
        lcd::init_stdout(layer_2_copy);

        // assignments to our struct
        let mut graphics = Graphics {
            lcd: lcd,
            layer_1: layer_1,
            layer_2: layer_2,
        };
        // clear and set black background
        graphics.layer_1.clear();
        graphics.layer_2.clear();
        graphics
            .lcd
            .set_background_color(lcd::Color::from_hex(0xb07708)); // snake color
        graphics
    }
    pub fn background_blink(&mut self) {
        for i in 0..6 {
            self.lcd.set_background_color(lcd::Color::rgb(255, 0, 0));
            system_clock::wait(50);
            self.lcd
                .set_background_color(lcd::Color::from_hex(0xb07708));
                system_clock::wait(50);
        }
    }
    /**
     * prints a square in defined size+color at position x,y
     */
    pub fn print_square_size_color_at(
        &mut self,
        x: usize,
        y: usize,
        size: usize,
        color: lcd::Color,
    ) {
        for x in x..(x + size) {
            for y in y..(y + size) {
                self.layer_1.print_point_color_at(x, y, color);
            }
        }
    }

    // pub fn print_string_at(&mut self,x:usize,y:usize,string: &str){
    //     let text = TextWriter{font_renderer: TTF ,x_pos: x, y_pos: y};
    //     text.layer(&self).write_str(string);

    // }
    /**
     * prints a pixels from a slice of a bitmap at position x,y on layer1 with rotation of 0,90,180,270 degree
     * Bitmaps need to be in 24bit color depth and uncompressed
     */
    pub fn print_bmp_at_with_rotaion(&mut self, pic: &[u8], x: u32, y: u32, rot: RotDirection) {
        let pixels_start = pic[10] as u32;
        let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
        let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);
        let pixel_rest = width % 4; // If width isn't dividable by 4 the lines are filled with bytes of zeros

        let rot_width = height;
        let rot_height = width;

        let at_x = x;
        let mut at_y = y;
        let mut bytenr: u32 = pixels_start;
        let pic_length: u32 = pic.len() as u32;

        match rot {
            RotDirection::r_0 => {
                at_y = y + height - 1;
            }
            RotDirection::r_90 => {
                at_y = y + rot_height - 1;
            }
            RotDirection::r_180 => {
                at_y = y + height - 1;
                bytenr = pic_length - 1;
            }
            RotDirection::r_270 => {
                at_y = y + rot_height - 1;
            }
        }

        match rot {
            RotDirection::r_0 => for i in 0..height {
                for j in 0..width {
                    if pic[(bytenr + 2) as usize] > 245 && pic[(bytenr + 1) as usize] > 245
                        && pic[(bytenr) as usize] > 245
                    {

                    } else {
                        self.layer_1.print_point_color_at(
                            (j + at_x) as usize,
                            (at_y - i) as usize,
                            lcd::Color::rgb(
                                pic[bytenr as usize + 2],
                                pic[(bytenr + 1) as usize],
                                pic[(bytenr) as usize],
                            ),
                        );
                    }
                    bytenr = bytenr + 3;
                    if j == (width - 1) {
                        bytenr = bytenr + pixel_rest;
                    }
                }
            },
            RotDirection::r_90 => for i in 0..rot_height {
                bytenr = pixels_start + width * 3 - 3 * (i + 1);
                for j in 0..rot_width {
                    if pic[(bytenr + 2) as usize] > 245 && pic[(bytenr + 1) as usize] > 245
                        && pic[(bytenr) as usize] > 245
                    {

                    } else {
                        self.layer_1.print_point_color_at(
                            (j + at_x) as usize,
                            (at_y - i) as usize,
                            lcd::Color::rgb(
                                pic[(bytenr + 2) as usize],
                                pic[(bytenr + 1) as usize],
                                pic[(bytenr) as usize],
                            ),
                        );
                    }
                    bytenr = bytenr + width * 3 + pixel_rest;
                }
            },
            RotDirection::r_180 => for i in 0..height {
                bytenr = bytenr - pixel_rest;
                for j in 0..width {
                    if pic[(bytenr - 2) as usize] > 245 && pic[(bytenr - 1) as usize] > 245
                        && pic[(bytenr) as usize] > 245
                    {

                    } else {
                        self.layer_1.print_point_color_at(
                            (j + at_x) as usize,
                            (at_y - i) as usize,
                            lcd::Color::rgb(
                                pic[(bytenr) as usize],
                                pic[(bytenr - 1) as usize],
                                pic[(bytenr - 2) as usize],
                            ),
                        );
                    }
                    bytenr = bytenr - 3;
                }
            },
            RotDirection::r_270 => for i in 0..rot_height {
                bytenr = pixels_start + (height - 1) * (3 * width + pixel_rest) + i * 3;

                for j in 0..rot_width {
                    if pic[(bytenr + 2) as usize] > 245 && pic[(bytenr + 1) as usize] > 245
                        && pic[(bytenr) as usize] > 245
                    {

                    } else {
                        self.layer_1.print_point_color_at(
                            (j + at_x) as usize,
                            (at_y - i) as usize,
                            lcd::Color::rgb(
                                pic[(bytenr + 2) as usize],
                                pic[(bytenr + 1) as usize],
                                pic[(bytenr) as usize],
                            ),
                        );
                    }
                    if bytenr > ((3 * width) + pixel_rest) {
                        bytenr = bytenr - (3 * width) - pixel_rest;
                    };
                }
            },
        }
    }
    pub fn print_bmp_at_downwards(&mut self, pic: &[u8], x: u32, y: u32) {
        let pixels_start = pic[10] as u32;
        let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
        let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);
        let pixel_rest = width % 4;

        let at_x = x;
        let mut at_y = y;
        let mut bytenr: u32 = pixels_start;
        let pixel_end: u32 = pic.len() as u32 - 1;
        // println!("{},{}",pixel_rest,pixel_end );

        for i in 0..height {
            bytenr = pixel_end + 1 - (pixel_rest + width * 3) * (i + 1);
            for j in 0..width {
                // println!("{}",bytenr );
                self.layer_1.print_point_color_at(
                    (j + at_x) as usize,
                    (at_y + i) as usize,
                    lcd::Color::rgb(
                        pic[(bytenr + 2) as usize],
                        pic[(bytenr + 1) as usize],
                        pic[(bytenr) as usize],
                    ),
                );
                bytenr = bytenr + 3;
            }
        }
    }
    pub fn print_bmp_at_layer2(&mut self, pic: &[u8], x: u32, y: u32) {
        let pixels_start = pic[10] as u32;
        let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
        let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);
        let pixel_rest = width % 4;

        let at_x = x;
        let mut at_y = y;
        let mut bytenr: u32 = pixels_start;
        let pixel_end: u32 = pic.len() as u32 - 1;
        // println!("{},{}",pixel_rest,pixel_end );

        for i in 0..height {
            bytenr = pixel_end + 1 - (pixel_rest + width * 3) * (i + 1);
            for j in 0..width {
                if pic[(bytenr + 2) as usize] > 245 && pic[(bytenr + 1) as usize] > 245
                    && pic[(bytenr) as usize] > 245
                {

                } else {
                    self.layer_2.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y + i) as usize,
                        lcd::Color::rgba(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                            pic[(bytenr) as usize]-50,
                        ),
                    );
                }
                bytenr = bytenr + 3;
            }
        }
    }
    pub fn print_pause_screen(&mut self) {
        self.print_bmp_at_layer2(pause_screen_left, 90 + 8, 20);
        self.print_bmp_at_layer2(pause_screen_right, 90 + 100 + 102 + 8, 20);
        self.print_bmp_at_layer2(pause_screen_pause, 100 + 90 + 8, 10 + 45);
        self.print_bmp_at_layer2(pause_screen_resume, 100 + 8 + 90, 139 + 6);
        self.print_bmp_at_layer2(pause_screen_new_game, 100 + 8 + 78, 192 + 6);
    }
    pub fn print_restart_screen(&mut self) {
        self.print_bmp_at_layer2(pause_screen_left, 60 + 8, 20);
        self.print_bmp_at_layer2(pause_screen_right, 90 + 100 + 20 + 102 + 8, 20);
        self.print_bmp_at_layer2(pause_screen_game_over, 60 + 90 + 8, 10 + 45);
        self.print_bmp_at_layer2(pause_screen_new_game, 100 + 8 + 78, 192 + 6);
    }
}
