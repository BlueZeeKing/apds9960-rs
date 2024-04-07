use crate::{Apds9960, Register, DEV_ADDR};
use embedded_hal::i2c;

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Read the device ID.
    ///
    /// This is per default `0xAB`.
    pub fn read_device_id(&mut self) -> Result<u8, E> {
        self.read_register(Register::ID)
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u8, E> {
        let mut data = [0];
        self.read_data(register, &mut data)?;
        Ok(data[0])
    }

    pub(crate) fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), E> {
        self.i2c.write_read(DEV_ADDR, &[register], data)
    }
}
