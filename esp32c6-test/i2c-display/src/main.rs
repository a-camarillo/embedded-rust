#![no_std]
#![no_main]

// I'm not sure why but this example only compiles with `--features embedded-hal-02`
// embedded-hal-02 is a dependency but not imported so my guess is that the import pulls in
// a trait impl that I2CDisplayInterface requires 

use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, 
        ascii::{FONT_6X10, FONT_9X18_BOLD}}, 
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use esp_backtrace as _;
use esp_hal::{clock::ClockControl,
            delay::Delay,
            peripherals::Peripherals,
            prelude::*,
            gpio::IO,
            i2c::I2C,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    //create a new peripheral object with described wiring
    //and standard i2c clock speed 
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio6,
        io.pins.gpio7,
        100.kHz(),
        &clocks,
        None,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    
    let text_style_big = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    loop {
        Text::with_alignment(
                "esp-hal",
                display.bounding_box().center() + Point::new(0, 0),
                text_style_big,
                Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();
        display.clear(BinaryColor::Off).unwrap();

        delay.delay(5.secs());
    }
}
