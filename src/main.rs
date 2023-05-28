#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::spi;
use smart_leds::RGB8;
use smart_leds::SmartLedsWrite;

const STRIP_LENGTH: usize = 30;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let motion_sensor = pins.d3.into_pull_up_input();
    
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
 
    loop {

        if motion_sensor.is_high() {

            let mut colors = self::colors();
            
            let seconds = 90;

            for i in 0 .. seconds {

                if i == seconds - 3 || i == seconds - 2 || i == seconds - 1 {
                    colors[STRIP_LENGTH - 3] = white;
                }

                strip.write(colors.iter().cloned()).ok();

                colors.rotate_right(1);

                arduino_hal::delay_ms(1000);
            }

            strip.write(self::colors_off().iter().cloned()).ok();
        }
        
        arduino_hal::delay_ms(50);
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
