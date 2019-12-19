#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::hal::spi::{Spi, MODE_0};
use hifive1::pin;
use hifive1::hal::delay::Sleep;
use max7219::*;

#[entry]
fn main() -> ! {
    let br = DeviceResources::take().unwrap();
    let p = br.peripherals;
    let gpio = br.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

    // Configure SPI pins
    let mosi = pin!(gpio, spi0_mosi).into_iof0();
    let sck = pin!(gpio, spi0_sck).into_iof0();
    let cs = pin!(gpio, spi0_ss0).into_iof0();
    // let cs = pin!(gpio, spi0_ss0).into_output();

    // Configure SPI
    let pins = (mosi, (), sck, cs);
    // let pins = (mosi, (), sck);
    let spi = Spi::new(p.QSPI1, pins, MODE_0, 1_000_000.hz(), clocks);

    let mut display = MAX7219::from_spi(2, spi).unwrap();
    // let mut display = MAX7219::from_spi_cs(2, spi, cs).unwrap();

    // make sure to wake the display up
    display.power_on().unwrap();
    // write given octet of ASCII characters with dots specified by 3rd param bits
    display.write_str(0, b"12345678", 0b00100000).unwrap();
    display.write_str(1, b"spi_help", 0b00100000).unwrap();
    // set display intensity lower
    display.set_intensity(0, 0x1).unwrap();
    display.set_intensity(1, 0x2).unwrap();

    let mut sleep = Sleep::new(br.core_peripherals.clint.mtimecmp, clocks);

    loop {
        sleep.delay_ms(100000 as u32);
    }
}
