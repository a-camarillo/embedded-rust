#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use microbit::{hal::{
        clocks::Clocks,
        gpio, pwm,
        rtc::{Rtc, RtcInterrupt},
        time::Hertz,
        prelude::OutputPin,
    },
    pac::{self, interrupt},
    Board,
};

static RTC: Mutex<RefCell<Option<Rtc<pac::RTC0>>>> = Mutex::new(RefCell::new(None));
static SPEAKER: Mutex<RefCell<Option<pwm::Pwm<pac::PWM0>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    if let Some(mut board) = Board::take() {
        cortex_m::interrupt::free(move |cs| {
            // non-blocking: The LF CLK pin is used by the speaker
            let _clocks = Clocks::new(board.CLOCK)
                .enable_ext_hfosc()
                .set_lfclk_src_synth()
                .start_lfclk();
            let mut rtc = Rtc::new(board.RTC0, 511).unwrap();
            rtc.enable_counter();
            rtc.enable_interrupt(RtcInterrupt::Tick, Some(&mut board.NVIC));
            rtc.enable_event(RtcInterrupt::Tick);

            *RTC.borrow(cs).borrow_mut() = Some(rtc);

            let mut speaker_pin = board.speaker_pin.into_push_pull_output(gpio::Level::High);
            let _ = speaker_pin.set_low();

            let speaker = pwm::Pwm::new(board.PWM0);
            speaker
                // output the waveform on the speaker pin
                .set_output_pin(pwm::Channel::C0, speaker_pin.degrade())
                // Use prescale by 16 to achieve darker sounds
                .set_prescaler(pwm::Prescaler::Div16)
                // Initial Frequency
                .set_period(Hertz(1u32))
                // Configure for up and down counter mode
                .set_counter_mode(pwm::CounterMode::UpAndDown)
                // Set maximum duty cycle
                .set_max_duty(32767)
                // enable PWM
                .enable();
            speaker
                .set_seq_refresh(pwm::Seq::Seq0, 0)
                .set_seq_end_delay(pwm::Seq::Seq0, 0);

            let max_duty = speaker.max_duty();
            speaker.set_duty_on_common(max_duty / 2);

            *SPEAKER.borrow(cs).borrow_mut() = Some(speaker);

            unsafe {
                pac::NVIC::unmask(pac::Interrupt::RTC0);
            }
            pac::NVIC::unpend(pac::Interrupt::RTC0);
        });
    }
    loop {
        continue;
    }
}

const STOP_FREQUENCY: u32 = 500;

#[interrupt]
fn RTC0() {
    static mut FREQUENCY: u32 = 1;

    cortex_m::interrupt::free(|cs| {
        if let (Some(speaker), Some(rtc)) = (
            SPEAKER.borrow(cs).borrow().as_ref(),
            RTC.borrow(cs).borrow().as_ref(),
        ) {
            if *FREQUENCY < STOP_FREQUENCY {
                speaker.set_period(Hertz(*FREQUENCY));
            } else {
                speaker.set_period(Hertz(STOP_FREQUENCY));
            }

            let max_duty = speaker.max_duty();
            speaker.set_duty_on_common(max_duty / 2);

            if *FREQUENCY >= STOP_FREQUENCY + 250 {
                defmt::info!("Fin");

                speaker.disable();
                rtc.disable_counter();
            };

            rtc.reset_event(RtcInterrupt::Tick);
        }
    });
    *FREQUENCY += 1;
}
