#![no_std]
#![no_main]

mod sg90;

use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, gpio::Io, mcpwm::{operator::PwmPinConfig, timer::PwmWorkingMode, McPwm, PeripheralClockConfig}, peripherals::Peripherals, prelude::*, system::SystemControl
};
use sg90::Servo;


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // initialize pin
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let pin = io.pins.gpio0;

    // initialize peripheral
    let clock_cfg = if let Ok(i) = PeripheralClockConfig::with_frequency(&clocks, 32.MHz()) {
        i
    } else {
        panic!("Frequency Error: Target frequency resulted in a prescaler value not in the range 0..255");
    };
    
    let mut motor_control = McPwm::new(peripherals.MCPWM0, clock_cfg);
    
    // connect operator0 to timer0
    motor_control.operator0.set_timer(&motor_control.timer0);

    // connect operator0 to servo pin
    let pwm_pin = motor_control.operator0.with_pin_a(pin, PwmPinConfig::UP_ACTIVE_HIGH);

    // Configure timer clock with 20 ms period and 50Hz frequency
    let timer_clock_cfg = if let Ok(i) = clock_cfg.timer_clock_with_frequency(19999, PwmWorkingMode::Increase, 50.Hz()) {
        i
    } else {
        panic!("Frequency Error: Target frequency or Period resulted in a prescaler value not in the range 0..255");
    };
    
    motor_control.timer0.start(timer_clock_cfg);

    let mut servo = Servo::new(pwm_pin);

    let delay = Delay::new(&clocks);

    loop {
        // alternate duty cycle between 2% and 12% which correspond to 0 and 180 degrees,
        // respectively
        servo.set_position(0);
        delay.delay_millis(1000_u32);
        servo.set_position(90);
        delay.delay_millis(1000_u32);
        servo.set_position(180);
        delay.delay_millis(1000_u32);
    }
}
