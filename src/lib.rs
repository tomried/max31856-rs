//! Rust driver for MAX31856
//!
//! Uses [`embedded-hal`] traits and patterns from Eldruin's [`driver-examples`]
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//! [`driver-examples`]: https://github.com/eldruin/driver-examples
//!
//! Features:
//! - Modify default configuration. see [`config()`]
//! - Read/write configuration. See: [`send_config()`]
//! - Read Linearized thermocouple temperature in Celcius. See: [`temperature()`]
//!
//! [`config()`]: struct.Max31856.html#method.config
//! [`send_config()`]: struct.Max31856.html#method.send_config
//! [`temperature()`]: struct.Max31856.html#method.temperature
//!
//! Features in the next few versions:
//! - Interrupts with FAULT pin
//! - External temperature sensor for cold junction conversion
//! - Read/write fault mask registers.
//! - Read/write cold junction fault mask registers.
//! - Read/write Linearized temperature fault registers.
//! - Read/write cold junction temperature offset registers. 
//! - Read cold junction temperature. 
//! - Read Fault status. 
//! 
//! ## Usage example
//! ```
//! extern crate max31856
//! extern crate linux_embedded_hal
//! 
//! let spi = Spidev::open("/dev/spidev0.0").unwrap();
//! let cs = Pin::new(25);
//! let fault = Pin::new(23); //Fault pin is unused
//! let mut sensor = Max31856::new(spi, cs, fault);
//! // A default configuration is set on creation. It can be edited as follows
//! sensor.config().average_samples(max31856::AveragingMode::FourSamples);
//! sensor.send_config();
//! println!(sensor.temperature().unwrap());
//! sensor.config().conversion_mode(max31856::CMode::AutomaticConversion);
//! sensor.send_config();
//! println!(sensor.temperature().unwrap());
//! ```
//! 

#![deny(unsafe_code, warnings, missing_docs)]
#![no_std]
#![allow(dead_code)]

extern crate embedded_hal as hal;
use hal::spi::{Mode, MODE_3};
use configuration::{FaultBits};

mod configuration;
pub use configuration::{CMode, OneShot, OCFaultModes, FaultModes, DeviceErrors, 
    NoiseRejectionMode, AveragingMode, ThermocoupleType, Max31856Options};
mod registers;
use registers::Registers;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// SPI communication error
    Spi(CommE),
    /// Pin setting error
    Pin(PinE),
    /// Invalid argument provided
    InvalidArgument,
    /// Errors from the device. 
    /// Can be more than one. If there is undervoltage or overvoltage, 
    /// other errors are not detected. Fix that first. Use DeviceError
    Device(DeviceErrors),
}

/// SPI mode (CPOL = 1, CPHA = 1)
pub const MODE: Mode = MODE_3; // See Table 5. Serial Interface Function

/// SPI interface
#[doc(hidden)]
#[derive(Debug, Default)]
pub struct SpiInterface<SPI, CS> {
    pub spi: SPI,
    pub cs: CS,
}

/// write interface trait
#[doc(hidden)]
pub trait SpiTransfer {
    type Error;
    fn write(&mut self, address: u8, word: u8) -> Result<(), Self::Error>;
    fn read(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

impl<SPI, CS, CommE, PinE> SpiTransfer for SpiInterface<SPI, CS>
where 
    SPI: hal::blocking::spi::Transfer<u8, Error=CommE>
        +  hal::blocking::spi::Write<u8, Error=CommE>,
    CS: hal::digital::v2::OutputPin<Error = PinE>,
{
    type Error = Error<CommE, PinE>;
    /// Write one byte to SPI. 
    fn write(&mut self, address:u8, word: u8) -> Result<(), Self::Error>{
        self.cs.set_low().map_err(Error::Pin)?;
        let result = self
            .spi
            .write(&[address, word])
            .map_err(Error::Spi);
        self.cs.set_high().map_err(Error::Pin)?;
        result
    }

    /// Read from the first address specified in the first byte of buffer.
    /// The buffer size should be equal to reply + 1 bytes
    fn read(&mut self, buffer:&mut [u8]) -> Result<(), Self::Error>{
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi
            .transfer(buffer)
            .map_err(Error::Spi)?;
        self.cs.set_high().map_err(Error::Pin)?;
        Ok(())
    }
}


/// Max31856 Precision Thermocouple to Digital Converter with Linearization
#[derive(Debug, Default)]
pub struct Max31856<I, FP> {
    iface: I,
    fault: FP,
    config: Max31856Options
}

impl<SPI, CS, FP> Max31856<SpiInterface<SPI, CS>, FP> {
    /// Create a new instance of Max31856
    pub fn new(spi: SPI, chip_select: CS, fault_pin: FP) -> Self {
        Max31856 {
            iface: SpiInterface {
                spi,
                cs: chip_select,
            },
            fault: fault_pin,
            config: Max31856Options::default(),
        }
    }
}

impl<CommE, PinE, DI, FP> Max31856<DI, FP>
where
    DI: SpiTransfer<Error = Error<CommE, PinE>>,
    FP: hal::digital::v2::InputPin,
{

    /// Parse options and write to C0 and C1 registers. 
    pub fn send_config(&mut self) -> Result<(), DI::Error> {
        self.send_c0()?;
        self.send_c1()
    }

    fn send_c0(&mut self) -> Result<(), DI::Error> {
        self.iface.write(Registers::CR0.write_address, self.config.extract_c0())
    }

    fn send_c1(&mut self) -> Result<(), DI::Error> {
        self.iface.write(Registers::CR1.write_address, self.config.extract_c1())
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
    pub fn cold_junction_temperature(&mut self) -> Result<f32, DI::Error> {
        todo!()
    }

    /// Get the linearized and cold-junction-compensated thermocouple
    /// temperature value.
    pub fn temperature(&mut self) -> Result<f32, DI::Error>{
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

        let mut buffer = [0u8; 4]; // Three bytes of temperature data
        buffer[0] = Registers::LTCBH.read_address;
        self.iface.read(&mut buffer)?;
        // TODO Check if any of the faults are triggered especially 
        // Check for over/under voltage or open circuit fault

        // The three bits are rearranged to derive the temperature
        let sign = if buffer[1] & 0x80 == 0x80 {-1.0} else {1.0};
        let mut value = ((buffer[1] & 0x7F) as i32) << 24;
        value += (buffer[2] as i32) << 16;
        value += (buffer[3] as i32) << 8;
        Ok(sign * (value as f32)/1048576.0)
    }

    /// Check if any of the faults are triggered
    pub fn fault_status(&mut self) -> Result<(), DI::Error>{
        let mut buffer = [0u8; 2]; // One byte value from Fault status register
        buffer[0] = Registers::SR.read_address;
        self.iface.read(&mut buffer)?;
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
