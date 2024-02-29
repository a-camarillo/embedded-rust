#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use embedded_hal::blocking::serial;
use nb::block;
use heapless::Vec;
use core::{
    fmt::Write,
    str::from_utf8,
};

use microbit::hal::prelude::*;

// i2c compatible modules
#[cfg(feature = "v2")]
use microbit::{
    hal::twim,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    pac::twim0::frequency::FREQUENCY_A,
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

use lsm303agr::{
    AccelOutputDataRate,
    Lsm303agr,
    MagOutputDataRate,
};

const ACCELEROMTER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v2")]
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // reading a single register
    // let mut acc = [0];
    // let mut mag = [0];

    // // First write the address + register onto the bus, then read the chip's responses
    // i2c.write_read(ACCELEROMTER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    // i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();

    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);

    // using a driver
    // let mut sensor = Lsm303agr::new_with_i2c(i2c);
    // sensor.init().unwrap();
    // sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    // loop{
    //     if sensor.accel_status().unwrap().xyz_new_data {
    //         let data = sensor.accel_data().unwrap();

    //         rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
    //     }
    // }

    // challenge: communicate with serial interface to respond to "magnetometer"
    // and "accelerometer"
    let carriage = b"\r";
    let newline = b"\n";
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();
    
    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if byte == carriage[0] || byte == newline[0] {
                break;
            }

            if buffer.push(byte).is_err() {
                write!(serial, "error: Buffer is full\r\n").unwrap();
                break;
            }
        }

        if from_utf8(&buffer).unwrap().trim() == "accelerometer" {
            while !sensor.accel_status().unwrap().xyz_new_data {
            }

            let data = sensor.accel_data().unwrap();
            write!(serial, "Accelerometer: x {} y {} z {}", data.x, data.y, data.z).unwrap();
        } else if from_utf8(&buffer).unwrap().trim() == "magnetometer" {
            while !sensor.accel_status().unwrap().xyz_new_data {
            }
            let data = sensor.mag_data().unwrap();
            write!(serial, "Magnetometer: x {} y {} z {}", data.x, data.y, data.z).unwrap(); 
        } else {
            write!(serial, "error: command not detected\r\n").unwrap();
        }
    } 
}