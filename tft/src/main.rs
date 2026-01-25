#![no_std]
#![no_main]

use core::cfg;
use core::assert;
use core::assert_eq;
use core::marker::Sized;
use core::clone::Clone;
use core::marker::Copy;
use core::default::Default;
use core::cmp::PartialEq;
use core::cmp::Ord;
use core::result::Result::Ok;

use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Speed},
    spi::{Spi, Config as SpiConfig},
    time::Hertz,
};
use embassy_time::Delay;
// use embassy_stm32::Delay;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};

use display_interface_spi::SPIInterface;
use st7735_lcd::{ST7735, Orientation};
use linux_embedded_hal::Delay;
use linux_embedded_hal::Pin;
use linux_embedded_hal::Spidev;
use embedded_hal::delay::DelayNs;

use panic_halt as _;
use cortex_m_rt::entry;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // ---------- GPIO ----------
    let dc  = Output::new(p.PB7, Level::Low, Speed::VeryHigh);
    let rst = Output::new(p.PB8, Level::Low, Speed::VeryHigh);
    let cs  = Output::new(p.PB6, Level::Low, Speed::VeryHigh);

    // ---------- SPI ----------
    let mut spi_cfg = SpiConfig::default();
    spi_cfg.frequency = Hertz(8_000_000);

    let spi = Spi::new_blocking(
        p.SPI1,
        p.PA5, // SCK
        p.PA7, // MOSI
        p.PA6, // MISO (unused)
        spi_cfg,
    );

    let di = SPIInterface::new(spi, dc);

    let mut display: ST7735<_, _, _> = ST7735::new(
        spi.into(),
        dc,
        rst,
        true,   // RGB
        false,  // inverted
        128,
        128,
    );

    let mut delay = Delay;
    display.init(&mut delay).ok();
    display.set_orientation(&Orientation::Portrait).ok();
    display.clear(Rgb565::BLACK).ok();

    draw_h(&mut display);

    loop {}
}

fn draw_h<D>(display: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = PrimitiveStyle::with_fill(Rgb565::GREEN);

    let _ = Rectangle::new(Point::new(30, 30), Size::new(10, 60))
        .into_styled(style)
        .draw(display);

    let _ = Rectangle::new(Point::new(60, 30), Size::new(10, 60))
        .into_styled(style)
        .draw(display);

    let _ = Rectangle::new(Point::new(30, 55), Size::new(40, 10))
        .into_styled(style)
        .draw(display);
}
