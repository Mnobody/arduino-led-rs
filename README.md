### Simple Arduino LED strip with Motion Sensor.

#### Parts
- Arduino NANO 3.0 v3 ATMEGA328P CH340 16MHz
- WS2812B
- PIR HC-SR501
- R1CH5VLL Relay
- PSL3P Light Sensor
- TTP223 Touch Button

#### Behavior

- 30 LEDs total length.
- 10 LEDs light up pink color, the rest 20 red.
- Pink LEDs move to the right every second.
- The last 3 LEDs will gradually turn white before the led strip turns off.
- LED strip lights up for 60 seconds afrter motion is detected.

---

This project is generated with 'avr-hal-template':

```bash
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

---

##### License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

##### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
