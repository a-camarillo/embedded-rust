// TODO
// Figure out why sensor won't take measurement after being moved/touched

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    delay::Delay, 
    gpio::{self, IO}, 
    peripherals::Peripherals, 
    prelude::*, systimer::SystemTimer, 
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let  io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut trig: gpio::GpioPin<gpio::Output<gpio::PushPull>, 8> = io.pins.gpio8.into_push_pull_output();
    let echo = io.pins.gpio10.into_pull_up_input();

    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    loop {
        if echo.is_low() {
            println!("echo is low");
        }

        trig.set_low();
        delay.delay(5.micros());

        trig.set_high();
        println!("Trigger is high");
        delay.delay(10.micros());
        trig.set_low();
        println!("Trigger is low");

        while !echo.is_high() {}
        println!("echo is high");

       let start = SystemTimer::now();
        println!("start, is {:?}", start);

        while !echo.is_low() {}
        println!("echo is low");

        let end = SystemTimer::now();

        let dur = end - start;

        let distance = dur / 40 / 58;

        println!("Distance in cm: {:?}", distance);

        delay.delay(50.millis());
    }
}