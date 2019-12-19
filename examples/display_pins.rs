#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::e310x::Peripherals;
use max7219::*;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Configure clocks
    hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure SPI pins
    let gpio = p.GPIO0.split();

    let data = gpio.pin3.into_output(); // 11
    let sck = gpio.pin5.into_output(); // 13
    let cs = gpio.pin2.into_output(); // 10

    let mut display = MAX7219::from_pins(2, data, cs, sck).unwrap();

    // make sure to wake the display up
    display.power_on().unwrap();
    // write given octet of ASCII characters with dots specified by 3rd param bits
    display.write_str(0, b"1234HELP", 0b00010000).unwrap();
    display.write_str(1, b"pulafoul", 0b00100000).unwrap();
    // set display intensity lower
    display.set_intensity(0, 0x1).unwrap();
    display.set_intensity(1, 0x2).unwrap();

    loop {}
}