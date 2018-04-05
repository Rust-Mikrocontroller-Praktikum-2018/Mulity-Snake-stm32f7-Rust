#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
#[macro_use]
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

// initialization routines for .data and .bss
extern crate r0;

use stm32f7::{board, embedded, lcd, sdram, system_clock};

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

    lcd.set_background_color(lcd::Color::from_hex(0x000000));

    lcd::init_stdout(layer_2);


    gameloop(lcd);
}

fn gameloop(lcd: lcd::Lcd) -> ! {
    println!("Hello Snaker! :)");
    println!("It's Mulity Snaker Time!");
    println!("https://github.com/iBaff/Multi-Snake-stm32f7-Rust ❽");
    println!("http://arewegameyet.com/ ൠ");
    println!("Maybe we make a text game?ᴥ");
    loop {
        let ticks = system_clock::ticks();
    }
}