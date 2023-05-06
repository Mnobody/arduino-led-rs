#![no_std]
#![no_main]

use panic_halt as _;

const TICK: u16 = 25;
const COUNTDOWN: i16 = 12000; // 5 min. - 300 sec. // 40 tick per second

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let motion_sensor = pins.d3.into_pull_up_input();
    let mut relay = pins.d6.into_output();
    relay.set_low();

    let mut countdown = COUNTDOWN;

    loop {

        if motion_sensor.is_high() {
            countdown = COUNTDOWN;
            relay.set_high();
        }

        if countdown <= 0 {
            relay.set_low();
        }
        
        countdown = countdown - 1;

        arduino_hal::delay_ms(TICK);
    }
}
