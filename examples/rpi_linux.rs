use linux_embedded_hal::{Spidev, Pin};

fn main() {
    let _spi = Spidev::open("/dev/spidev0.0").unwrap();
    let _chip_select = Pin::new(25);
}