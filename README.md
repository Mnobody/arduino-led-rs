### Simple Arduino LED strip with Motion Sensor.

#### Parts
- Arduino NANO 3.0 v3 ATMEGA328P CH340 16MHz
- PIR HC-SR501
- R1CH5VLL Relay

#### Behavior

- After movement detected relay is turned on for 5 min.
- If movement detected during 5 min. countdown, countdown starts from the beginning.

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
