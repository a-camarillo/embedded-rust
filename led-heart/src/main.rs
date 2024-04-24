#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = match Board::take() {
        Some(i) => i,
        None => Board::take().unwrap(),
    };
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let clear_display = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let led_heart = [
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];
    
    loop {
        display.show(&mut timer, led_heart, 500);
        display.clear();
        display.show(&mut timer, clear_display, 500);
    }
}
