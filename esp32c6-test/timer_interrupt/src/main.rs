#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, 
    peripherals::{Interrupt, Peripherals, TIMG0}, 
    prelude::*,
    interrupt::{self, Priority},
    timer::{Timer, Timer0, TimerGroup, TimerInterrupts},
};

// create a new mutex to be used safely across threads
static TIMER0: Mutex<RefCell<Option<Timer<Timer0<TIMG0>, esp_hal::Blocking>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // defining the time group with associated timer interrupt handlers
    let timg = TimerGroup::new(
        peripherals.TIMG0, 
        &clocks, 
        Some(TimerInterrupts { timer0_t0: Some(tg0_t0_level), ..Default::default()}));
    
    let mut timer0 = timg.timer0;

    interrupt::enable(Interrupt::TG0_T0_LEVEL, Priority::Priority1).unwrap();

    timer0.start(500u64.millis());
    timer0.listen();
    
    let mut new_time: u64 = 0;
    
    critical_section::with(|cs| {
        TIMER0.borrow_ref_mut(cs).replace(timer0);
        new_time = TIMER0.borrow_ref_mut(cs).as_mut().unwrap().now();
    });

    loop {
        esp_println::println!("new time {:?} ms", new_time);
    }
}

#[handler]
fn tg0_t0_level() {
   critical_section::with(|cs| { 
        let mut timer0 = TIMER0.borrow_ref_mut(cs);
        let timer0 = timer0.as_mut().unwrap();
        esp_println::println!(
            "Interrupt at {} ms",
            timer0.now() 
            );

        timer0.clear_interrupt();
        timer0.start(5000u64.millis());
   }) 
}
