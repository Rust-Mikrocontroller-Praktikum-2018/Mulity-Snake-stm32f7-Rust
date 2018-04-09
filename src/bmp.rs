#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]
#![feature(alloc)]

extern crate compiler_builtins;
#[macro_use]
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

// initialization routines for .data and .bss
extern crate r0;

extern crate alloc;

use stm32f7::{board, embedded, lcd, sdram, system_clock, touch, i2c};
use alloc::Vec;
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }
    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section (copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);
    asm!("DSB; ISB;"::::"volatile"); // pipeline flush

    main(board::hw());
}

struct Point {
    x: usize,
    y: usize,
}

fn print_pic_at(pic: BmpPic, point: Point, mut layer: lcd::Layer<lcd::FramebufferArgb8888>) {
    let width = pic.width;
    let height = pic.height;
    let draw_x = point.x as u32;
    let draw_y = point.y as u32;

    for x in draw_x..=width {
        for y in draw_y..=height {
            let current_pixel_number = (x - draw_x) * width + y;
            let current_pixel = &pic.pixels[(current_pixel_number - 1) as usize];
            layer.print_point_color_at(
                x as usize,
                y as usize,
                lcd::Color::rgb(current_pixel.red, current_pixel.green, current_pixel.blue),
            )
        }
    }
}

struct BmpPixel<'a> {
    red: &'a [u8],
    green: &'a [u8],
    blue: &'a [u8],
}

struct BmpPic<'a> {
    width: &'a [u32],
    height: &'a [u32],
    pixels: Vec<BmpPixel>,
}


enum RotDirection {
    twelve_o_clock,
    three_o_clock,
    six_o_clock,
    nine_o_clock,
}

fn bmp_to_bmp_pic(bmp: &[u8],pixel_start: u8) -> BmpPic {
    // let lenght_of_bmp = bmp.len();
    // println!("{}", lenght_of_bmp);
    
    // let width = bmp[18] as u32;
    // let height = bmp[22] as u32;
    
    // let mut pixel_vec = Vec::new();

    // let mut i = pixel_start as usize;

    // while i <= (lenght_of_bmp - 2) as usize {
    //     let bmp_pixel = BmpPixel {
    //         red: bmp[i as usize] as u8,
    //         green: bmp[(i + 1) as usize] as u8,
    //         blue: bmp[(i + 2) as usize] as u8,
    //     };
    //     pixel_vec.push(bmp_pixel);
    //     i = i + 3;
    let lenght_of_bmp = bmp.len();
    println!("{}", lenght_of_bmp);
    
    let width = &bmp[18];
    let height =&bmp[22];
    
    let mut pixel_vec = Vec::new();

    let mut i = pixel_start as usize;

    while i <= (lenght_of_bmp - 2) as usize {
        let bmp_pixel = BmpPixel {
            red: &bmp[i as usize] as u8,
            green: &bmp[(i + 1) as usize] as u8,
            blue: &bmp[(i + 2) as usize] as u8,
        };
        pixel_vec.push(bmp_pixel);
        i = i + 3;
    }

    let bmp_pic = BmpPic {
        width: width,
        height: height,
        pixels: pixel_vec,
    };
    bmp_pic
}

fn main(hw: board::Hardware) -> ! {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        sai_2,
        syscfg,
        ethernet_mac,
        ethernet_dma,
        nvic,
        exti,
        sdmmc,
        ..
    } = hw;

    use embedded::interfaces::gpio::{self, Gpio};
    let mut gpio = Gpio::new(
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
    );

    system_clock::init(rcc, pwr, flash);
    // enable all gpio ports
    rcc.ahb1enr.update(|r| {
        r.set_gpioaen(true);
        r.set_gpioben(true);
        r.set_gpiocen(true);
        r.set_gpioden(true);
        r.set_gpioeen(true);
        r.set_gpiofen(true);
        r.set_gpiogen(true);
        r.set_gpiohen(true);
        r.set_gpioien(true);
        r.set_gpiojen(true);
        r.set_gpioken(true);
    });

    // init sdram (needed for display buffer)
    sdram::init(rcc, fmc, &mut gpio);
    // lcd controller
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);

    // let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_1 = match lcd.layer_1() {
        Some(layer1) => layer1,
        None => panic!("No lcd.layer_1!"),
    };

    // let mut layer_2 = lcd.layer_2().unwrap();
    let mut layer_2 = match lcd.layer_2() {
        Some(layer2) => layer2,
        None => panic!("No lcd.layer_2!"),
    };

    layer_1.clear();
    layer_2.clear();

    lcd.set_background_color(lcd::Color::rgb(255, 0, 0));

    lcd::init_stdout(layer_2);

    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    let pic: &[u8] = include_bytes!("../assets/Test7.bmp");
    let pixel_start = pic[10];


    let point = Point { x: 100, y: 271 };

    // print_pic_at(pic, point, layer_1);

    //layer_1.print_point_color_at(point.x, point.y, lcd::Color::rgb(0, 0, 0));
    // let current_pixel = &pic.pixels[(1) as usize];

    // layer_1.print_point_color_at(point.x, point.y, lcd::Color::rgb(current_pixel.red, current_pixel.green, current_pixel.blue));

    let width = pic.width;
    let height = pic.height;
    let draw_x = point.x as u32;
    let draw_y = point.y as u32;

    for y in 0..height {
        for x in 1..=width {
            let current_pixel_number = y * width + x - 1;
            let current_pixel = &pic.pixels[(current_pixel_number) as usize];
            layer_1.print_point_color_at(
                (x + draw_x) as usize,
                (draw_y - y) as usize,
                lcd::Color::rgb(current_pixel.red, current_pixel.green, current_pixel.blue),
            )
        }
    }

    loop {}
}
//     // Background Color
//     let back_ground_color = 0xDAEBE0;
//     lcd.set_background_color(lcd::Color::from_hex(back_ground_color));

//     // Drawing Color

//     let drawing_color = 0x100000;
//     // lcd.set_background_color(lcd::Color::from_hex(drawing_color));

//     //Draw Button

//     for x in 10..60 {
//         for y in 100..150 {
//             layer_1.print_point_color_at(x, y, lcd::Color::from_hex(0x000f))
//         }
//     }

//     // Erase Button

//     for x in (10 + 400)..(60 + 400) {
//         for y in 100..150 {
//             layer_1.print_point_color_at(x, y, lcd::Color::from_hex(0x000f))
//         }
//     }

//     let mut c = back_ground_color;

//     loop {
//         for touch in &touch::touches(&mut i2c_3).unwrap() {
//             let x = touch.x;
//             let y = touch.y;

//             if (x > 10 && x < 60) && (y > 100 && y < 150) {
//                 c = drawing_color;
//             } else if (x > 410 && x < 460) && (y > 100 && y < 150) {
//                 c = back_ground_color;
//             }
//         }

//         for touch in &touch::touches(&mut i2c_3).unwrap() {
//             let x = touch.x;
//             let y = touch.y;

//             if (x > 70 && x < 400) && (y > 6 && y < 267) {
//                 for i in (x - 5)..(x + 5) {
//                     for j in (y - 5)..(y + 5) {
//                         layer_1.print_point_color_at(
//                             i as usize,
//                             j as usize,
//                             lcd::Color::from_hex(c)
//                         );
//                     }
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let bild: &[u8] = include_bytes!("../assets/Test.bmp");
//     let pic = bmp_to_bmp_pic(bild);
//     let point = {x: 100, y:100};

//     print_pic_at(pic, point);

// }

// fn rotate_bmp(pic: BmpPic, rot_direction: RotDirection) -> BmpPic {
//     let width = pic.width;
//     let height = pic.height;

//     // let mut pixel_vec = Vec::new();
//     // for i in 0..pic.pixels.len(){
//     //     pixel_vec.push(&pic.pixels[i]);
//     // }

//     // let pixel_vec = &pic.pixels;
//     // let pixels = pic.pixels.len();
//     let pixel_vec = pic.pixels.clone();

//     let mut rot_pixel_vec = Vec::new();
//     let rot_width;
//     let rot_height;

//     match rot_direction {
//         RotDirection::twelve_o_clock => return pic,
//         RotDirection::three_o_clock => {
//             rot_width = height;
//             rot_height = width;

//             for j in 0..rot_height {
//                 for i in 0..rot_width {
//                     rot_pixel_vec.push(pixel_vec[(width - j + width * i as u8) as usize]);
//                 }
//             }
//             let rot_pic = BmpPic{width: rot_width, height: rot_height, pixels: rot_pixel_vec };
//             return rot_pic
//         }
//         RotDirection::six_o_clock => return pic,
//         RotDirection::nine_o_clock => return pic,
//     }
// }
