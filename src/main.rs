#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::spi;
use smart_leds::RGB8;
use smart_leds::SmartLedsWrite;

const STRIP_LENGTH: usize = 48;
const TICK: u16 = 50;

const TURNING_ON: i8 = 2;
const TURNED_ON: i8 = 3;
const TURNING_OFF: i8 = 4;
const TURNING_OFF_DELAY: i8 = 5;
const TURNED_OFF: i8 = 6;

const RGB_ON: i8 = 1;
const RGB_OFF: i8 = 2;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let motion_sensor = pins.d3.into_pull_up_input();
    let light_sensor = pins.d4.into_pull_up_input();
    let touch_button = pins.d5.into_pull_up_input();
    
    let mut relay = pins.d6.into_output();
    relay.set_low();

    let mut state = TURNED_OFF;

    let mut turning_on_count = 0;
    let mut turning_off_count = 0;
    let mut turning_off_delay_count = 0;
    
    let mut rgb_state = RGB_OFF;
    let mut rgb_count = 0;
    let mut rgb_second_count = 0;

    let (spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    let mut strip = ws2812_spi::Ws2812::new(spi);

    let white = [65, 65, 65].into();
    
    let mut colors = self::colors();
 
    loop {

        if state == TURNED_OFF && touch_button.is_low() {
            state = TURNING_ON;
        }

        if state == TURNING_ON {
            
            turning_on_count += 1;
            
            if turning_on_count >= 5 {

                relay.set_high();

                state = TURNED_ON;

                turning_on_count = 0;
            }
        }

        if state == TURNED_ON && touch_button.is_low() {
            state = TURNING_OFF; 
        }

        if state == TURNING_OFF {

            turning_off_count += 1;

            if turning_off_count >= 5 {
                
                state = TURNING_OFF_DELAY;

                turning_off_count = 0;
            }
        }

        if state == TURNING_OFF_DELAY {
            turning_off_delay_count += 1;

            if turning_off_delay_count >= 600 { // 30 seconds

                relay.set_low();

                state = TURNED_OFF;

                turning_off_delay_count = 0;
            }
        }

        // RGB STRIP

        if state == TURNING_OFF {
            rgb_state = RGB_ON;
            colors = self::colors();
        }

        if rgb_state == RGB_OFF && motion_sensor.is_high() && light_sensor.is_high() {
            rgb_state = RGB_ON;
            colors = self::colors();
        }
        
        if rgb_state == RGB_ON {
            rgb_count += 1;

            if rgb_count % 20 == 0 { // 1 second
                
                rgb_second_count += 1;

                if rgb_second_count == 57 || rgb_second_count == 58 || rgb_second_count == 59 {
                    colors[STRIP_LENGTH - 3] = white;
                }
                
                strip.write(colors.iter().cloned()).ok();
                colors.rotate_right(1);
            }

            if rgb_second_count >= 60 { // 60 seconds
                rgb_state = RGB_OFF;
                rgb_count = 0;
                rgb_second_count = 0;
                strip.write(self::colors_off().iter().cloned()).ok();
            }
        }

        arduino_hal::delay_ms(TICK);


        // switch button state
//        if touch_button.is_low() {
//            if on {
//                on = false;
//                delayed_turning_off = true;
//            } else {
//                on = true;
//            }
//
//            arduino_hal::delay_ms(250);
//        }
//
//        if delayed_turning_off {
//            arduino_hal::delay_ms(1500);
//            relay.set_low();
//            delayed_turning_off = false;
//        }
//
//        if on {
//            relay.set_high();
//        } else {
//            relay.set_low();
//        }

//        if !on && motion_sensor.is_high() && light_sensor.is_high() {
//
//            let mut colors = self::colors();
//            
//            let seconds = 3;
//
//            for i in 0 .. seconds {
//
//                if i == seconds - 3 || i == seconds - 2 || i == seconds - 1 {
//                    colors[STRIP_LENGTH - 3] = white;
//                }
//
//                strip.write(colors.iter().cloned()).ok();
//
//                colors.rotate_right(1);
//
//                arduino_hal::delay_ms(1000);
//            }
//
//            strip.write(self::colors_off().iter().cloned()).ok();
//        }
//
//        arduino_hal::delay_ms(50);
    }
}

fn colors() -> [RGB8; STRIP_LENGTH] {

    let red  = [100, 1, 2].into();
    let pink = [100, 4, 8].into();

    let mut colors = [RGB8::default(); STRIP_LENGTH];

    for i in 0 .. STRIP_LENGTH / 3 {
        colors[i] = pink;
    }
   
    for i in STRIP_LENGTH / 3 .. STRIP_LENGTH {
        colors[i] = red;
    }

    return colors;
}

fn colors_off() -> [RGB8; STRIP_LENGTH] {

    return [RGB8::default(); STRIP_LENGTH];
}
