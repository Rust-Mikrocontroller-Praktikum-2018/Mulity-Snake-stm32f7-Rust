use stm32f7::lcd;

pub struct Graphics {
    lcd: lcd::Lcd,
    layer_1: lcd::Layer<lcd::FramebufferArgb8888>,
    layer_2: lcd::Layer<lcd::FramebufferAl88>,
}

pub enum RotDirection {
    r_0,
    r_90,
    r_180,
    r_270,
}

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
            .set_background_color(lcd::Color::from_hex(0x9CC136)); // snake color
        graphics
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
                at_y = y + height;
            }
            RotDirection::r_90 => {
                at_y = y + rot_height;
            }
            RotDirection::r_180 => {
                at_y = y + height;
                bytenr = pic_length - 1;
            }
            RotDirection::r_270 => {
                at_y = y + rot_height;
            }
        }

        match rot {
            RotDirection::r_0 => for i in 0..height {
                for j in 0..width {
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[bytenr as usize + 2],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                        ),
                    );
                    bytenr = bytenr + 3;
                    if j == (width - 1) {
                        bytenr = bytenr + pixel_rest;
                    }
                }
            },
            RotDirection::r_90 => for i in 0..rot_height {
                bytenr = pixels_start + width * 3 - 3 * (i + 1) - pixel_rest;
                for j in 0..rot_width {
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                        ),
                    );
                    bytenr = bytenr + width * 3 + pixel_rest;
                }
            },
            RotDirection::r_180 => for i in 0..height {
                for j in 0..width {
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[(bytenr) as usize],
                            pic[(bytenr - 1) as usize],
                            pic[(bytenr - 2) as usize],
                        ),
                    );
                    bytenr = bytenr - 3;
                    if j == (width - 1) {
                        bytenr = bytenr - pixel_rest;
                    }
                }
            },
            RotDirection::r_270 => for i in 0..rot_height {
                bytenr = pixels_start + ((width - 1) * 3 * height) + 3 * i;
                for j in 0..rot_width {
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                        ),
                    );
                    if bytenr > (width * 3) {
                        bytenr = bytenr - width * 3;
                    }
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
                // println!("{}",bytenr );
                self.layer_2.print_point_color_at(
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
}
