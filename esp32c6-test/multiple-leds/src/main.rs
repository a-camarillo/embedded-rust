#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    gpio::{AnyPin, Input, IO, Output, PullDown, PushPull},
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let led1 = io.pins.gpio8.into_push_pull_output();
    let led2 = io.pins.gpio10.into_push_pull_output();

    let button = io.pins.gpio0.into_pull_down_input().into();

    let mut pins = [led1.into(), led2.into()];
    

    let delay = Delay::new(&clocks);

    loop {
        toggle_pins(&mut pins, &button);
        delay.delay_millis(500);
    }
}

fn toggle_pins(leds: &mut [AnyPin<Output<PushPull>>], button: &AnyPin<Input<PullDown>>) {

    if button.is_low() {
        println!("Button not pressed");
    } else {
        println!("Button pressed");
        for pin in leds.iter_mut() {
            pin.toggle();
        }
    }
}
