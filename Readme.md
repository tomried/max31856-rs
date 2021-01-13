Rust driver for MAX31856
//!
Uses [`embedded-hal`] traits and patterns from Eldruin's [`driver-examples`]
//!
[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
[`driver-examples`]: https://github.com/eldruin/driver-examples
//!
Features:
- Modify default configuration. see [`config()`]
- Read/write configuration. See: [`send_config()`]
- Read Linearized thermocouple temperature in Celcius. See: [`temperature()`]
//!
[`config()`]: struct.Max31856.html#method.config
[`send_config()`]: struct.Max31856.html#method.send_config
[`temperature()`]: struct.Max31856.html#method.temperature
//!
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