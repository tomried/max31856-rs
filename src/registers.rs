/// Registers that can be written to and read from. Uses different addresses
pub struct ReadWriteRegister {
    pub read_address: u8,
    pub write_address: u8,
    pub factory_default: u8,
}

/// Registers that can only be read from
pub struct ReadOnlyRegister {
    pub read_address: u8,
}

pub struct Registers {}

impl Registers {
    pub const CR0: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x00,
        write_address: 0x80,
        factory_default: 0x00,
    }; //Configuration 0 Register
    pub const CR1: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x01,
        write_address: 0x81,
        factory_default: 0x03,
    }; //Configuration 1 Register
    pub const MASK: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x02,
        write_address: 0x82,
        factory_default: 0xFF,
    }; //Fault Mask Register
    pub const CJHF: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x03,
        write_address: 0x83,
        factory_default: 0x7F,
    }; //Cold-Junction High Fault Threshold
    pub const CJLF: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x04,
        write_address: 0x84,
        factory_default: 0xC0,
    }; //Cold-Junction Low Fault Threshold
    pub const LTHFTH: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x05,
        write_address: 0x85,
        factory_default: 0x7F,
    }; //Linearized Temperature High Fault Threshold MSB
    pub const LTHFTL: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x06,
        write_address: 0x86,
        factory_default: 0xFF,
    }; //Linearized Temperature High Fault Threshold LSB
    pub const LTLFTH: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x07,
        write_address: 0x87,
        factory_default: 0x80,
    }; //Linearized Temperature Low Fault Threshold MSB
    pub const LTLFTL: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x08,
        write_address: 0x88,
        factory_default: 0x00,
    }; //Linearized Temperature Low Fault Threshold LSB
    pub const CJTO: ReadWriteRegister = ReadWriteRegister {
        read_address: 0x09,
        write_address: 0x89,
        factory_default: 0x00,
    }; //Cold Junction Temperature offset
    pub const CJTH: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0A }; //Cold Junction Temperature High
    pub const CJTL: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0B }; //Cold Junction Temperature Low
    pub const LTCBH: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0C };
    pub const LTCBM: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0D };
    pub const LTCBL: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0E };
    pub const SR: ReadOnlyRegister = ReadOnlyRegister { read_address: 0x0F };
}
