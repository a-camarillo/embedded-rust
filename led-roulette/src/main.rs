#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
// use panic_halt as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer}
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    // single light example

    // board.display_pins.col1.set_low().unwrap();
    // board.display_pins.row1.set_high().unwrap();

    // delay example

    // let mut timer = Timer::new(board.TIMER0);


    // loop {
    //     timer.delay_ms(1000u16);
    //     rprintln!("1000 ms passed");
    // }

    // blinking example
    
    // let mut timer = Timer::new(board.TIMER0);
    
    // board.display_pins.col1.set_low().unwrap();
    // let mut row1 = board.display_pins.row1;

    // loop {
    //     row1.set_low().unwrap();
    //     rprintln!("Dark!");
    //     timer.delay_ms(1_000_u16);
    //     row1.set_high().unwrap();
    //     rprintln!("Light!");
    //     timer.delay_ms(1_000_u16);
    // }

    // rotating light challenge
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut led_matrix = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    
    // if i represents the rows of our matrix and j represents the columns
    // then when i = 0 and i = 4, we want to modify all values of j, else we want to only modify j[0] and j[4]
    
    let mut i = 0;
    let mut j = 0;
    let min = 0;
    let max = 4;

    loop {
        while i == min && j < max {        
            led_matrix[i][j] = 1;

            display.show(&mut timer, led_matrix, 50);
            display.clear();

            led_matrix[i][j] = 0;
            
            j += 1;
        }
        while j == max  && i < max {        
            led_matrix[i][j] = 1;

            display.show(&mut timer, led_matrix, 50);
            display.clear();

            led_matrix[i][j] = 0;
            
            i += 1;
        }
        while i == max && j > min {
            led_matrix[i][j] = 1;

            display.show(&mut timer, led_matrix, 50);
            display.clear();

            led_matrix[i][j] = 0;
            
            j -= 1;
        }
        while j == min && i > min {
            led_matrix[i][j] = 1;

            display.show(&mut timer, led_matrix, 50);
            display.clear();

            led_matrix[i][j] = 0;
            
            i -= 1;
        }
    } 
}
