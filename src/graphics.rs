use stm32f7::lcd;

pub struct Graphics {
    lcd: lcd::Lcd,
    layer_1: lcd::Layer<lcd::FramebufferArgb8888>,
    layer_2: lcd::Layer<lcd::FramebufferAl88>,
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
            lcd     : lcd,
            layer_1 : layer_1,
            layer_2 : layer_2,
        };
        // clear and set black background
        graphics.layer_1.clear();
        graphics.layer_2.clear();
        graphics.lcd.set_background_color(lcd::Color::from_hex(0x000000));
        graphics
    }

    /**
     * prints a square in defined size+color at position x,y
     */
    pub fn print_square_size_color_at(&mut self, x: usize, y: usize, size: usize, color: lcd::Color) {
        for x in x..(x+size) {
            for y in y..(y+size) {
                self.layer_1.print_point_color_at(x, y, color);
            }
        }
    }
}
