//! Rust driver for MAX31856
//!
//! Uses [`embedded-hal`] 1.0.0 traits (SpiDevice) and patterns from Eldruin's [`driver-examples`]
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//! [`driver-examples`]: https://github.com/eldruin/driver-examples
//! 
//! Communication with MAX31856 only works with Spi Mode 1 or 3.
//!
//! Features:
//! - Modify default configuration. See: [`config()`]
//! - Read/write configuration. See: [`send_config()`]
//! - Read Linearized thermocouple temperature in Celcius. See: [`temperature()`]
//! - Read cold junction temperature. See: [`cold_junction_temperature()`]
//! - Read Fault status. See: [`fault_status()`]
//!
//! [`config()`]: struct.Max31856.html#method.config
//! [`send_config()`]: struct.Max31856.html#method.send_config
//! [`temperature()`]: struct.Max31856.html#method.temperature
//! [`cold_junction_temperature()`]: struct.Max31856.html#method.cold_junction_temperature
//! [`fault_status()`]: struct.Max31856.html#method.fault_status
//!
//! Features in the next few versions:
//! - Interrupts with FAULT pin
//! - External temperature sensor for cold junction conversion
//! - Read/write fault mask registers.
//! - Read/write cold junction fault mask registers.
//! - Read/write Linearized temperature fault registers.
//! - Read/write cold junction temperature offset registers. 
//! 
//! ## Usage example
//! ```
//! use max31856;
//! 
//! fn example<S, FP>(spi_dev: S, fault_pin: FP) -> Result<(), max31856::Error>
//! where
//!     S: embedded_hal::spi::SpiDevice,
//!     FP: embedded_hal::digital::InputPin,
//! {
//!     let mut sensor = max31856::Max31856::new(spi_dev, fault_pin);
//!     // A default configuration is set on creation. It can be edited as follows
//!     sensor.config().average_samples(max31856::AveragingMode::FourSamples);
//!     let _ = sensor.send_config();
//!     println!("Temperature: {}", sensor.temperature().unwrap());
//!     sensor.config().conversion_mode(max31856::CMode::AutomaticConversion);
//!     let _ = sensor.send_config();
//!     println!("Temperature: {}", sensor.temperature().unwrap());
//!     // Faults can be assessed via 
//!     println!("Status: {:?}", sensor.fault_status()); 
//!     Ok(())
//! }
//! ```
//! 

#![deny(unsafe_code, warnings, missing_docs)]
#![no_std]
#![allow(dead_code)]

extern crate embedded_hal as hal;
use hal::spi::{Mode, MODE_3};
use configuration::FaultBits;

mod configuration;
pub use configuration::{CMode, OneShot, OCFaultModes, FaultModes, DeviceErrors, 
    NoiseRejectionMode, AveragingMode, ThermocoupleType, Max31856Options};
mod registers;
use registers::Registers;

/// Errors in this crate
#[derive(Debug)]
pub enum Error {
    /// SPI communication error
    Spi,
    /// Pin setting error
    Pin,
    /// Invalid argument provided
    InvalidArgument,
    /// Errors from the device. 
    /// Can be more than one. If there is undervoltage or overvoltage, 
    /// other errors are not detected. Fix that first. Use DeviceError
    Device(DeviceErrors),
}

/// SPI mode (CPOL = 1, CPHA = 1)
pub const MODE: Mode = MODE_3; // See Table 5. Serial Interface Function


/// Max31856 Precision Thermocouple to Digital Converter with Linearization
#[derive(Debug, Default)]
pub struct Max31856<SPI, FP> {
    spi: SPI,
    fault: FP,
    config: Max31856Options
}

impl<SPI, FP> Max31856<SPI, FP>
where
    SPI: embedded_hal::spi::SpiDevice,
    FP: hal::digital::InputPin,
{
    /// Create a new instance of Max31856
    pub fn new(spi: SPI, fault_pin: FP) -> Self {
        Max31856 {
            spi,
            fault: fault_pin,
            config: Max31856Options::default(),
        }
    }

    /// Parse options and write to C0 and C1 registers. 
    pub fn send_config(&mut self) -> Result<(), Error> {
        self.send_c0()?;
        self.send_c1()
    }

    fn send_c0(&mut self) -> Result<(), Error> {
        self.spi.write(&[Registers::CR0.write_address, self.config.extract_c0()])
        .map_err(|_| Error::Spi)
    }

    fn send_c1(&mut self) -> Result<(), Error> {
        self.spi.write(&[Registers::CR1.write_address, self.config.extract_c1()])
        .map_err(|_| Error::Spi)
    }

    /// Get a reference of stored configuration. This can be then used to modify certain
    /// values. send_config() can then be used to write it to the sensor. 
    pub fn config(&mut self) -> &mut Max31856Options{
        &mut self.config
    }
    //TODO: method for writing and reading fault mask register

    //TODO: method for setting cold junction high and low fault threshold

    //TODO: method for setting linearized temperature high and low threshold

    //TODO: method for cold junction temperature offset

    /// Get the measured value of cold-junction temperature 
    /// plus the value in the Cold-Junction Offset register
    pub fn cold_junction_temperature(&mut self) -> Result<f32, Error> {
        let mut buffer = [0u8; 3]; // Two bytes of temperature data
        buffer[0] = Registers::CJTH.read_address;
        self.spi.transfer_in_place(&mut buffer).map_err(|_| Error::Spi)?;
        let mut value: i16 = (buffer[1] as i16) << 8;
        value += buffer[2] as i16;
        Ok(value as f32 / 256.0)
    }

    /// Get the linearized and cold-junction-compensated thermocouple
    /// temperature value.
    pub fn temperature(&mut self) -> Result<f32, Error>{
        //If conversion mode is normally off, a one-time conversion should be done.
        //The one shot conversion takes about 150ms and then the bit is reset.
        //On automatic conversion mode, the temperature can requested without 1-shot trigger

        let cmode = self.config.conversion_mode;
        match cmode {
            CMode::NormallyOff => {
                self.config.one_shot_conversion = OneShot::OneShotConversion;
                self.send_c0()?; //One shot only changes c0. This part is executed often
            }
            _ => {}
        } 

        let mut buffer = [0u8; 4]; // One byte address, three bytes of temperature data
        buffer[0] = Registers::LTCBH.read_address;
        self.spi.transfer_in_place(&mut buffer).map_err(|_| Error::Spi)?;
        // TODO Check if any of the faults are triggered especially 
        // Check for over/under voltage or open circuit fault

        // move bytes into int
        let mut value: i32 = (buffer[1] as i32) << 16;
        value += (buffer[2] as i32) << 8;
        value += buffer[3] as i32;
        // fill extra bits with msb for twos-complement representation with larger int
        if buffer[1] & 0x80 == 0x80 {
            value += 0xFF_i32 << 24;
        }
        // shift out least significant 5 bits because they are not used
        value >>= 5;
        // multiply by measurement accuracy
        Ok(value as f32 * 0.0078125)
    }

    /// Check if any of the faults are triggered
    pub fn fault_status(&mut self) -> Result<(), Error>{
        let mut buffer = [0u8; 2]; // One byte value from Fault status register
        buffer[0] = Registers::SR.read_address;
        self.spi.transfer_in_place(&mut buffer).map_err(|_| Error::Spi)?;
        let error_id = buffer[1];
        let mut has_error = false;
        let mut errors =  DeviceErrors::default();

        //If overvoltage or undervoltage, all other errors might not be set
        if(error_id & FaultBits::OVUV) !=0 {
            errors.overvoltage_undervoltage = true;
            return Err(Error::Device(errors))
        }
        if(error_id & FaultBits::CJ_HIGH) !=0 {
            errors.cold_junction_high = true;
            has_error = true;
        }
        if(error_id & FaultBits::CJ_LOW) !=0 {
            errors.cold_junction_low = true;
            has_error = true;
        }
        if(error_id & FaultBits::CJ_RANGE) !=0 {
            errors.cold_junction_out_of_range = true;
            has_error = true;
        }
        if(error_id & FaultBits::OPEN) !=0 {
            errors.open_circuit = true;
            has_error = true;
        }
        if(error_id & FaultBits::TC_HIGH) !=0 {
            errors.thermocouple_high = true;
            has_error = true;
        }
        if(error_id & FaultBits::TC_LOW) !=0 {
            errors.thermocouple_low = true;
            has_error = true;
        }
        if(error_id & FaultBits::TC_RANGE) !=0 {
            errors.thermocouple_out_of_range = true;
            has_error = true;
        }
        if has_error {            
            Err(Error::Device(errors))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod internal {
    use super::*;
    #[test]
    fn can_extract_max31856_c0_c1() {
        let mut options = Max31856Options::new();
        options.average_samples(AveragingMode::SixteenSamples)
            .fault_mode(FaultModes::Interrupt)
            .noise_rejection_frequency(NoiseRejectionMode::Reject50Hz)
            .conversion_mode(CMode::AutomaticConversion);
        let c0_c1 = (options.extract_c0(), options.extract_c1());
        assert_eq!(c0_c1, (0b1000_0101, 0b0100_0011));
    }
}
