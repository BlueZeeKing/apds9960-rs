use hal::blocking::i2c;
use {
    register::{GStatus, Status},
    Apds9960, BitFlags, Error, Register, DEV_ADDR,
};

impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Read the proximity sensor data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_proximity(&mut self) -> nb::Result<u8, Error<E>> {
        if !self.is_proximity_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        self.read_register(Register::PDATA)
            .map_err(nb::Error::Other)
    }

    /// Read whether the proximity sensor data is valid.
    ///
    /// This is checked internally in `read_proximity()` as well.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_proximity_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        Ok(Status::new(status).is(Status::PVALID, true))
    }

    /// Read the amount of available data in the gesture FIFO registers.
    pub fn read_gesture_data_level(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::GFLVL)
    }

    /// Read whether there is valid gesture data available.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_gesture_data_valid(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::GSTATUS)?;
        Ok(GStatus::new(status).is(GStatus::GVALID, true))
    }

    /// Read gesture data.
    ///
    /// Will read the gesture data up to the minimum of: gesture data level, array size.
    /// Make sure to provide an array with at least the number of elements returned by the
    /// `read_gesture_data_level()` method multiplied by 4.
    ///
    /// The data contents will be organized as follows:
    /// `[up_dataset0, down_dataset0, left_dataset0, right_dataset0,
    ///   up_dataset1, down_dataset1, left_dataset1, right_dataset1, ...]`
    ///
    /// Returns `nb::Error::WouldBlock` as long as not enough data is available.
    pub fn read_gesture_data(&mut self, data: &mut [u8]) -> nb::Result<(), Error<E>> {
        if !self.is_gesture_data_valid().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let level = self.read_gesture_data_level().map_err(nb::Error::Other)?;
        let byte_count = core::cmp::min(data.len(), 4*level as usize);
        self.read_data(Register::GFIFO_U, &mut data[..byte_count]).map_err(nb::Error::Other)?;
        Ok(())
    }

    /// Read the device ID.
    ///
    /// This is per default `0xAB`.
    pub fn read_device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_register(Register::ID)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.read_data(register, &mut data)?;
        Ok(data[0])
    }

    fn read_data(&mut self, register: u8, data: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(DEV_ADDR, &[register], data)
            .map_err(Error::I2C)
    }
}
