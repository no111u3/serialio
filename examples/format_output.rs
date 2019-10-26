//! Test the formated output
//!
//! This example hasn't a special requires
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use serialio::{sprintln, SerialIO};
use stm32f30x_hal::{prelude::*, serial::Serial};

use f3::hal::stm32f30x;

#[entry]
fn main() -> ! {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    let (tx, rx) = serial.split();

    let mut in_out = SerialIO::new(tx, rx);

    sprintln!(in_out, "The answer is {}", 40 + 2);

    asm::bkpt();

    loop {}
}
