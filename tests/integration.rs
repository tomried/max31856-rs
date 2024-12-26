use max31856;
use embedded_hal_mock as hal;
use self::hal::eh1::spi::{Mock as SpiMock, 
    Transaction as SpiTransaction};
use self::hal::eh1::digital::Mock as PinMock;
use self::max31856::{Max31856, Error};

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
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0]), //Write C0
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x81, 0x23]), //Write C1
        SpiTransaction::transaction_end(),
    ];

    let mut spi = SpiMock::new(&spi_expectations);
    let mut fault = PinMock::new(&[]);

    let mut sensor = Max31856::new(&mut spi, &mut fault);
    sensor.config().average_samples(max31856::AveragingMode::FourSamples);
    sensor.send_config().unwrap();
    spi.done();
    fault.done();
}

#[test]
fn can_read_temperature_normally_off() {
    // SPI transactions
    let spi_expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]), //Write oneshot c0
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x05, 0x72, 0xC0]), //Read temperature registers
        SpiTransaction::transaction_end(),
    ];

    let mut spi = SpiMock::new(&spi_expectations);
    let mut fault = PinMock::new(&[]);
    let mut sensor = Max31856::new(&mut spi, &mut fault);
    assert_eq!(sensor.temperature().unwrap(), 87.171875);
    spi.done();
    fault.done();
}

#[test]
fn can_get_fault_status(){
    // SPI transactions
    let spi_expectations = [
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0F, 0], vec![0x0F, 0x15]), //Read fault status register
        SpiTransaction::transaction_end(),
    ];

    let mut spi = SpiMock::new(&spi_expectations);
    let mut fault = PinMock::new(&[]);
    let mut sensor = Max31856::new(&mut spi, &mut fault);
    let result = sensor.fault_status();
    match result {
        Err(Error::Device(errors)) => {
            assert_eq!(false, errors.cold_junction_out_of_range);
            assert_eq!(false, errors.thermocouple_out_of_range);
            assert_eq!(false, errors.cold_junction_high);
            assert_eq!(true, errors.cold_junction_low);
            assert_eq!(false, errors.thermocouple_high);
            assert_eq!(true, errors.thermocouple_low);            
            assert_eq!(false, errors.overvoltage_undervoltage);
            assert_eq!(true, errors.open_circuit);
        }
        _ => panic!("Wrong result"),
    }
    spi.done();
    fault.done();
}

#[test]
fn can_read_temperatures_normally_off() {
    // SPI transactions
    let spi_expectations = [
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 1600.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x64, 0x00, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 1000.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x3E, 0x80, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 100.9375 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x06, 0x4F, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 25.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x01, 0x90, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 0.0625 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x00, 0x01, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of 0.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0x00, 0x00, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of -0.0625 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0xFF, 0xFF, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of -0.25 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0xFF, 0xFC, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of -1.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0xFF, 0xF0, 0x00]),
        SpiTransaction::transaction_end(),
        // Write oneshot c0
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80, 0x40]),
        SpiTransaction::transaction_end(),
        // Read temperature register with a value of -250.0 °C
        SpiTransaction::transaction_start(),
        SpiTransaction::transfer_in_place(vec![0x0C, 0,0,0], vec![0x0C, 0xF0, 0x60, 0x00]),
        SpiTransaction::transaction_end(),
    ];

    let mut spi = SpiMock::new(&spi_expectations);
    let mut fault = PinMock::new(&[]);
    let mut sensor = Max31856::new(&mut spi, &mut fault);
    assert_eq!(sensor.temperature().unwrap(), 1600.0);
    assert_eq!(sensor.temperature().unwrap(), 1000.0);
    assert_eq!(sensor.temperature().unwrap(),  100.9375);
    assert_eq!(sensor.temperature().unwrap(),   25.0);
    assert_eq!(sensor.temperature().unwrap(),    0.0625);
    assert_eq!(sensor.temperature().unwrap(),    0.0);
    assert_eq!(sensor.temperature().unwrap(),   -0.0625);
    assert_eq!(sensor.temperature().unwrap(),   -0.25);
    assert_eq!(sensor.temperature().unwrap(),   -1.0);
    assert_eq!(sensor.temperature().unwrap(), -250.0);
    spi.done();
    fault.done();
}
