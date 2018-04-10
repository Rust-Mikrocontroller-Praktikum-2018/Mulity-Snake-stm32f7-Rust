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
extern crate r0;
extern crate smoltcp;

use alloc::Vec;
use smoltcp::socket::{Socket, SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpEndpoint};
use stm32f7::ethernet::IP_ADDR;
use stm32f7::{board, embedded, ethernet, lcd, sdram, system_clock};

mod game;
mod graphics;
mod network;

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

    /* ETHERNET START */
    let network;
    let mut ethernet_device = ethernet::EthernetDevice::new(
        Default::default(),
        Default::default(),
        rcc,
        syscfg,
        &mut gpio,
        ethernet_mac,
        ethernet_dma,
    );
    if let Err(e) = ethernet_device {
        println!("ethernet init failed: {:?}", e);
        panic!("ethernet init failed: {:?}", e);
    };
    if let Ok(ether) = ethernet_device {
        network = network::Network::new(ether, network::NetworkMode::client);
    }

    /* ETHERNET END */

    gameloop(graphics);
}

fn gameloop(mut graphics: graphics::Graphics) -> ! {
    // Initialize Game
    let mut game = game::Game::new(graphics);
    loop {
        // let ticks = system_clock::ticks();
        game.draw_game();
        system_clock::wait(10);
    }
}
