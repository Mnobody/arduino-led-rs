#![no_std]
#![no_main]

use panic_halt as _;
//use arduino_hal::hal::wdt;

//const TICK: u16 = 25;
const TICK: u16 = 50;
const MINUTE: u16 = 60000; // 1 min.
const MINUTES: u8 = 5; // 5 min.
//const COUNTDOWN: i16 = 12000; // 5 min. - 300 sec. // 40 tick per second
//const MIN: i16 = 1;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let motion_sensor = pins.d3.into_pull_up_input();
    let mut relay = pins.d6.into_output();

    relay.set_low();

//    let mut countdown = COUNTDOWN;

//    let mut watchdog = wdt::Wdt::new(dp.WDT, &dp.CPU.mcusr);
//    watchdog.start(wdt::Timeout::Ms8000).unwrap();

    loop {

        if motion_sensor.is_high() {
//            countdown = COUNTDOWN;
            relay.set_high();

            for _ in 1 .. MINUTES {
                arduino_hal::delay_ms(MINUTE);
            }

            relay.set_low();
        }

//        arduino_hal::delay_ms(DELAY);

//        if countdown <= MIN {
//            relay.set_low();
//        } else {
//            countdown = countdown - 1;
//        }

        arduino_hal::delay_ms(TICK);

//        watchdog.feed();
    }
}
// cargo run --release
