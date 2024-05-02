#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::IO, peripherals::Peripherals, prelude::*, rmt::Rmt};

use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use smart_leds::{
    brightness,
    gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio8;

    let rmt = Rmt::new(peripherals.RMT, 80.MHz(), &clocks, None).unwrap();

    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, led, rmt_buffer, &clocks);
   
    let delay = Delay::new(&clocks);

    let mut color = Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };

    let mut data;
    
    loop {
        for hue in 0..=255 {
            color.hue = hue;

            data = [hsv2rgb(color)];

            led.write(brightness(gamma(data.iter().cloned()), 10))
                .unwrap();
            delay.delay_millis(20);
        }
    }
}
