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
            .set_background_color(lcd::Color::from_hex(0x000000));
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

    // /**
    //  * prints a pixels from a slice of a bitmap at position x,y on layer1
    //  */
    // pub fn print_bmp_at_0(&mut self, pic: &[u8], at_x: u32, at_y: u32) {
    //     let pixels_start = pic[10] as u32;
    //     let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
    //     let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);
    //     let pic_length: u32 = pic.len() as u32;
    //     let mut bytenr: u32 = pixels_start;
    //     println!(
    //         "width: {}, height: {}, pixel start: {}, pic length: {}",
    //         width, height, pixels_start, pic_length
    //     );

    //     for i in 0..height {
    //         if (at_y - i) < 0 {
    //             break;
    //         }
    //         for j in 0..width {
    //             if (j + at_x) > 479 {
    //                 bytenr = bytenr + (width - j) * 3;
    //                 break;
    //             }
    //             self.layer_1.print_point_color_at(
    //                 (j + at_x) as usize,
    //                 (at_y - i) as usize,
    //                 lcd::Color::rgb(
    //                     pic[bytenr as usize + 2],
    //                     pic[(bytenr + 1) as usize],
    //                     pic[(bytenr) as usize],
    //                 ),
    //             );
    //             bytenr = bytenr + 3;
    //         }
    //     }
    // }
    // /*
    //  ** 180 grad gedreht
    //  */
    // pub fn print_bmp_at_180(&mut self, pic: &[u8], at_x: u32, at_y: u32) {
    //     let pixels_start = pic[10] as u32;
    //     let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
    //     let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);
    //     let pic_length: u32 = pic.len() as u32;
    //     let mut bytenr: u32 = pic_length - 1;
    //     println!(
    //         "width: {}, height: {}, pixel start: {}, pic length: {}",
    //         width, height, pixels_start, pic_length
    //     );

    //     for i in 0..height {
    //         if (at_y - i) < 0 {
    //             break;
    //         }
    //         for j in 0..width {
    //             if (j + at_x) > 479 {
    //                 bytenr = bytenr + (width - j) * 3;
    //                 break;
    //             }
    //             self.layer_1.print_point_color_at(
    //                 (j + at_x) as usize,
    //                 (at_y - i) as usize,
    //                 lcd::Color::rgb(
    //                     pic[bytenr as usize - 2],
    //                     pic[(bytenr - 1) as usize],
    //                     pic[(bytenr) as usize],
    //                 ),
    //             );
    //             bytenr = bytenr - 3;
    //         }
    //     }
    // }
    // /*
    //  ** 90 grad gedreht
    //  */
    // pub fn print_bmp_at_90(&mut self, pic: &[u8], at_x: u32, at_y: u32) {
    //     let pixels_start = pic[10] as u32;
    //     let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
    //     let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);

    //     let rot_width = height;
    //     let rot_height = width;

    //     let pic_length: u32 = pic.len() as u32;
    //     let mut bytenr: u32;
    //     println!(
    //         "width: {}, height: {}, pixel start: {}, pic length: {}",
    //         width, height, pixels_start, pic_length
    //     );

    //     for i in 0..rot_height {
    //         bytenr = pixels_start + width * 3 - 3 * (i + 1);

    //         if (at_y - i) < 0 {
    //             break;
    //         }
    //         for j in 0..rot_width {
    //             if (j + at_x) > 479 {
    //                 bytenr = bytenr + (width - j) * 3;
    //                 break;
    //             }
    //             self.layer_1.print_point_color_at(
    //                 (j + at_x) as usize,
    //                 (at_y - i) as usize,
    //                 lcd::Color::rgb(
    //                     pic[(bytenr + 2) as usize],
    //                     pic[(bytenr + 1) as usize],
    //                     pic[(bytenr) as usize],
    //                 ),
    //             );
    //             bytenr = bytenr + width * 3;
    //         }
    //     }
    // }
    // pub fn print_bmp_at_270(&mut self, pic: &[u8], at_x: u32, at_y: u32) {
    //     let pixels_start = pic[10] as u32;
    //     let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
    //     let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);

    //     let rot_width = height;
    //     let rot_height = width;

    //     let pic_length: u32 = pic.len() as u32;
    //     let mut bytenr: u32;
    //     println!(
    //         "width: {}, height: {}, pixel start: {}, pic length: {}",
    //         width, height, pixels_start, pic_length
    //     );

    //     for i in 0..rot_height {
    //         bytenr = pixels_start + ((width - 1) * 3 * height) + 3 * i;

    //         if (at_y - i) < 0 {
    //             break;
    //         }
    //         for j in 0..rot_width {
    //             // if (j + at_x) > 479 {
    //             //     bytenr = bytenr *(width - j) * 3;
    //             //     break;
    //             // }
    //             self.layer_1.print_point_color_at(
    //                 (j + at_x) as usize,
    //                 (at_y - i) as usize,
    //                 lcd::Color::rgb(
    //                     pic[(bytenr + 2) as usize],
    //                     pic[(bytenr + 1) as usize],
    //                     pic[(bytenr) as usize],
    //                 ),
    //             );
    //             if bytenr > (width * 3) {
    //                 bytenr = bytenr - width * 3;
    //             }
    //             //println!("{}",bytenr )
    //         }
    //     }
    // }
    /**
     * prints a pixels from a slice of a bitmap at position x,y on layer1 with rotation of 0,90,180,270 degree
     */
    pub fn print_bmp_at_with_rotaion(
        &mut self,
        pic: &[u8],
        at_x: u32,
        at_y: u32,
        rot: RotDirection,
    ) {
        let pixels_start = pic[10] as u32;
        let width = (pic[18] as u32) + ((pic[19] as u32) * 256_u32);
        let height = (pic[22] as u32) + ((pic[23] as u32) * 256_u32);

        let rot_width = height;
        let rot_height = width;

        let pic_length: u32 = pic.len() as u32;
        let mut bytenr: u32 = pixels_start;

        match rot {
            RotDirection::r_0 => for i in 0..height {
                if (at_y - i) < 0 {
                    break;
                }
                for j in 0..width {
                    if (j + at_x) > 479 {
                        bytenr = bytenr + (width - j) * 3;
                        break;
                    }
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
                }
            },
            RotDirection::r_90 => for i in 0..rot_height {
                bytenr = pixels_start + width * 3 - 3 * (i + 1);

                if (at_y - i) < 0 {
                    break;
                }
                for j in 0..rot_width {
                    if (j + at_x) > 479 {
                        bytenr = bytenr + (width - j) * 3;
                        break;
                    }
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[(bytenr + 2) as usize],
                            pic[(bytenr + 1) as usize],
                            pic[(bytenr) as usize],
                        ),
                    );
                    bytenr = bytenr + width * 3;
                }
            },
            RotDirection::r_180 => for i in 0..height {
                if (at_y - i) < 0 {
                    break;
                }
                for j in 0..width {
                    if (j + at_x) > 479 {
                        bytenr = bytenr + (width - j) * 3;
                        break;
                    }
                    self.layer_1.print_point_color_at(
                        (j + at_x) as usize,
                        (at_y - i) as usize,
                        lcd::Color::rgb(
                            pic[bytenr as usize - 2],
                            pic[(bytenr - 1) as usize],
                            pic[(bytenr) as usize],
                        ),
                    );
                    bytenr = bytenr - 3;
                }
            },
            RotDirection::r_270 => for i in 0..rot_height {
                bytenr = pixels_start + ((width - 1) * 3 * height) + 3 * i;

                if (at_y - i) < 0 {
                    break;
                }
                for j in 0..rot_width {
                    if (j + at_x) > 479 {
                        bytenr = bytenr * (width - j) * 3;
                        break;
                    }
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
                    //println!("{}",bytenr )
                }
            },
        }
    }
}
