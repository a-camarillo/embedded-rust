#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, 
    delay::Delay, 
    peripherals::Peripherals, 
    prelude::*,
    mcpwm::{PeripheralClockConfig, MCPWM, operator::PwmPinConfig, timer::PwmWorkingMode},
    gpio::IO,
};
use embedded_hal::pwm::SetDutyCycle;


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // initialize pin
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let servo = io.pins.gpio0;

    // initialize peripheral
    let clock_cfg = PeripheralClockConfig::with_frequency(&clocks, 32.MHz()).unwrap();
    let mut motor_control = MCPWM::new(peripherals.MCPWM0, clock_cfg);
    
    // connect operator0 to timer0
    motor_control.operator0.set_timer(&motor_control.timer0);

    // connect operator0 to servo pin
    let mut pwm_pin = motor_control.operator0.with_pin_a(servo, PwmPinConfig::UP_ACTIVE_HIGH);

    // Configure timer clock with 20 ms period and 50Hz frequency
    let timer_clock_cfg = clock_cfg
        .timer_clock_with_frequency(19999, PwmWorkingMode::Increase, 50.Hz())
        .unwrap();
    motor_control.timer0.start(timer_clock_cfg);

    let delay = Delay::new(&clocks);

    loop {
        // alternate duty cycle between 5% and 10% which correspond to 0 and 180 degrees,
        // respectively
        pwm_pin.set_duty_cycle_percent(10_u8).unwrap();
        delay.delay_millis(1000_u32);
        pwm_pin.set_duty_cycle_percent(5_u8).unwrap();
        delay.delay_millis(1000_u32);
    }
}
