# Rust driver for MAX31856

Uses [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits and patterns from Eldruin's [`driver-examples`](https://github.com/eldruin/driver-examples)

Features:
- Modify default configuration. see `config()`
- Read/write configuration. See: `send_config()`
- Read Linearized thermocouple temperature in Celcius. See: `temperature()`

Features in the next few versions:
- Interrupts with FAULT pin
- External temperature sensor for cold junction conversion
- Read/write fault mask registers.
- Read/write cold junction fault mask registers.
- Read/write Linearized temperature fault registers.
- Read/write cold junction temperature offset registers. 
- Read cold junction temperature. 
- Read Fault status. 

## Usage example
```
extern crate max31856
extern crate linux_embedded_hal

let spi = Spidev::open("/dev/spidev0.0").unwrap();
let cs = Pin::new(25);
let fault = Pin::new(23); //Fault pin is unused
let mut sensor = Max31856::new(spi, cs, fault);
// A default configuration is set on creation. It can be edited as follows
sensor.config().average_samples(max31856::AveragingMode::FourSamples);
sensor.send_config();
println!(sensor.temperature().unwrap());
sensor.config().conversion_mode(max31856::CMode::AutomaticConversion);
sensor.send_config();
println!(sensor.temperature().unwrap());
```
## Support

For questions, issues, feature requests like compatibility with similar devices
and other changes, please file an
[issue in the github project](https://github.com/idheepan/max31856-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal