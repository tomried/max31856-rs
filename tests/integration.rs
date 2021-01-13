extern crate max31856;
extern crate embedded_hal_mock as hal;
use self::hal::spi::{Mock as SpiMock, 
    Transaction as SpiTransaction};
use self::hal::pin::{Transaction as PinTransaction, 
    Mock as PinMock, 
    State as PinState};
use self::max31856::Max31856;

#[test]
fn can_create_max31856_options() {
    let mut options = max31856::Max31856Options::new();
    options.average_samples(max31856::AveragingMode::SixteenSamples)
        .conversion_mode(max31856::CMode::AutomaticConversion);
}

#[test]
fn can_send_configuration(){
    // SPI transactions
    let spi_expectations = [
        SpiTransaction::write(vec![0x80, 0]), //Write C0
        SpiTransaction::write(vec![0x81, 0x23]), //Write C1
    ];
    // Pin transactions
    let pin_expectations = [
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ];

    let spi = SpiMock::new(&spi_expectations);
    let cs = PinMock::new(&pin_expectations);
    let fault = PinMock::new(&pin_expectations);

    let mut sensor = Max31856::new(spi, cs, fault);
    sensor.config().average_samples(max31856::AveragingMode::FourSamples);
    sensor.send_config().unwrap();
}

#[test]
fn can_read_temperature_normally_off() {
        // SPI transactions
        let spi_expectations = [
            SpiTransaction::write(vec![0x80, 0x40]), //Write oneshot c0
            SpiTransaction::transfer(vec![0x0C, 0,0,0], vec![0x0C, 0x05, 0x72, 0xC0]), //Write C1
        ];
        // Pin transactions
        let pin_expectations = [
            //Setting C0 for one shot
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
            //Transfering data
            PinTransaction::set(PinState::Low),
            PinTransaction::set(PinState::High),
        ];
    
        let spi = SpiMock::new(&spi_expectations);
        let cs = PinMock::new(&pin_expectations);
        let fault = PinMock::new(&pin_expectations);
        let mut sensor = Max31856::new(spi, cs, fault);
        assert_eq!(sensor.temperature().unwrap(), 87.171875);
}

#[test]
fn can_read_negative_temperature_normally_off() {
    // SPI transactions
    let spi_expectations = [
        SpiTransaction::write(vec![0x80, 0x40]), //Write oneshot c0
        SpiTransaction::transfer(vec![0x0C, 0,0,0], vec![0x0C, 0x85, 0x72, 0xC0]), //Write C1
    ];
    // Pin transactions
    let pin_expectations = [
        //Setting C0 for one shot
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
        //Transfering data
        PinTransaction::set(PinState::Low),
        PinTransaction::set(PinState::High),
    ];

    let spi = SpiMock::new(&spi_expectations);
    let cs = PinMock::new(&pin_expectations);
    let fault = PinMock::new(&pin_expectations);
    let mut sensor = Max31856::new(spi, cs, fault);
    assert_eq!(sensor.temperature().unwrap(), -87.171875);
}