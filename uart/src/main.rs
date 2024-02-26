#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::blocking::serial;
use nb::block;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::Vec;

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap(); 

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // sending a single byte 
    // nb::block!(serial.write(b'X')).unwrap();
    // nb::block!(serial.flush()).unwrap();

    // sending a string
    // let sample = "The quick brown fox jumps over the lazy dog";
    // for c in sample.as_bytes() {
    //     nb::block!(serial.write(*c)).unwrap();
    //     nb::block!(serial.flush()).unwrap();
    // }

    // using core::fmt::Write
    // write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    // nb::block!(serial.flush()).unwrap();

    // receive a single byte
    // loop {
    //     let byte = nb::block!(serial.read()).unwrap();
    //     rprintln!("{}", char::from(byte));
    // }
    
    // echo server
    // loop {
    //     let req = nb::block!(serial.read()).unwrap();
    //     nb::block!(serial.write(req)).unwrap();
    //     nb::block!(serial.flush()).unwrap();
    // }
    
    // reverse string
    let mut buffer: Vec<u8, 32> = Vec::new();
    let carriage = b"\r";
    let newline = b"\n";

    loop {
        buffer.clear();

        loop {
            let req = nb::block!(serial.read()).unwrap();
            
            if buffer.push(req).is_err() {
                write!(serial, "error: Buffer is full\r\n").unwrap();
                break;
            }

            if req == carriage[0] || req == newline[0] {
                for byte in buffer.iter().rev().chain(&[b'\n',b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }
        } 
        nb::block!(serial.flush()).unwrap();
    }
}
