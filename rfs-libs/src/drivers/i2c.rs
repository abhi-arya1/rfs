#![no_std]
#![no_main]

extern crate embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};

const DRIVER_ADDRESS_NAME : u8 = 0x00;

struct DriverData {
    x: i16,
    y: i16,
    z: i16,
}


pub struct Driver<I2C> {
    i2c: I2C,
    name: String,
}

impl<I2C, E> Driver<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Driver {
            i2c,
            name: String::from("Driver"),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data(&mut self) -> Result<DriverData, E> {
        let mut data = DriverData { x: 0, y: 0, z: 0 };
        self.i2c.write(DRIVER_ADDRESS_NAME, &[0x00])?;
        self.i2c.write_read(DRIVER_ADDRESS_NAME, &[0x00], &mut [0; 6])?;
        Ok(data)
    }
}