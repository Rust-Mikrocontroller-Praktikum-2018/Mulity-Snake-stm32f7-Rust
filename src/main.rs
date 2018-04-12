#![no_std]
#![no_main]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(alloc)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate compiler_builtins;
#[macro_use]
extern crate stm32f7_discovery as stm32f7; // initialization routines for .data and .bss

// initialization routines for .data and .bss
#[macro_use]
extern crate alloc;
extern crate arrayvec;
extern crate r0;
extern crate smoltcp;

#[macro_use]
use stm32f7::{board, embedded, ethernet, exceptions, lcd, sdram, system_clock, touch, i2c};

mod game;
mod graphics;
mod random;

pub const HEIGHT: usize = 272;
pub const WIDTH: usize = 480;

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static mut __DATA_END: u32;
        static mut __DATA_START: u32;

        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    // initializes the .data section (copy the data segment initializers from flash to RAM)
    r0::init_data(&mut __DATA_START, &mut __DATA_END, &__DATA_LOAD);
    // zeroes the .bss section
    r0::zero_bss(&mut __BSS_START, &__BSS_END);

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
        syscfg,
        ethernet_mac,
        ethernet_dma,
        i2c_3,
        rng,
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
    let lcd = lcd::init(ltdc, rcc, &mut gpio);
    let graphics = graphics::Graphics::new(lcd);

    //i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    i2c_3.test_1();
    i2c_3.test_2();

    system_clock::wait(200);

    touch::check_family_id(&mut i2c_3).unwrap();

    /* ETHERNET START */

    /* ETHERNET END */
    // l0et layer2 = lcd::Layer<lcd::FramebufferAl88>;

    let random_gen = random::Random::new(rng, rcc);
    // Initialize Game
    let mut game = game::Game::new(graphics, i2c_3, random_gen);
    gameloop(game);
}

fn gameloop(mut game: game::Game) -> ! {
    //game.game_start_up();
    game.draw_frame();

    loop {
        // let ticks = system_clock::ticks();
        game.move_snake();
        game.snake_bite();
        game.check_grid_edge();
        game.check_selfbite();
        game.draw_game();
        system_clock::wait(100);

    }
}
