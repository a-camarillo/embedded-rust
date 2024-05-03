#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    delay::Delay, 
    peripherals::Peripherals, 
    prelude::*,
    gpio::IO,
    uart::{config::Config, TxRxPins, Uart},
};
use esp_println::println;
use nb::block;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = TxRxPins::new_tx_rx(
        io.pins.gpio4.into_push_pull_output(),
        io.pins.gpio5.into_floating_input(),
    );
    let delay = Delay::new(&clocks);
    
    let mut serial1 = Uart::new_with_config(
        peripherals.UART1,
        Config::default(),
        Some(pins),
        &clocks,
        None,
    );

    println!("Start");
    loop {
        serial1.write_byte(0x42).ok();
        let read = block!(serial1.read_byte());

        match read {
            Ok(read) => println!("Read 0x{:02x}", read),
            Err(err) => println!("Error {:?}", err),
        }

        delay.delay_millis(250);
    }
}
