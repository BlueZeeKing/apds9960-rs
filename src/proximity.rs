use crate::{
    register::{Config2, Enable, Status},
    Apds9960, BitFlags, Register, DEV_ADDR,
};
use embedded_hal::i2c;

/// Proximity.
impl<I2C, E> Apds9960<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Enable proximity detection
    pub fn enable_proximity(&mut self) -> Result<(), E> {
        self.set_flag_enable(Enable::PEN, true)
    }

    /// Disable proximity detection
    pub fn disable_proximity(&mut self) -> Result<(), E> {
        self.set_flag_enable(Enable::PEN, false)
    }

    /// Enable proximity interrupt generation
    pub fn enable_proximity_interrupts(&mut self) -> Result<(), E> {
        self.set_flag_enable(Enable::PIEN, true)
    }

    /// Disable proximity interrupt generation
    pub fn disable_proximity_interrupts(&mut self) -> Result<(), E> {
        self.set_flag_enable(Enable::PIEN, false)
    }

    /// Enable proximity saturation interrupt generation
    pub fn enable_proximity_saturation_interrupts(&mut self) -> Result<(), E> {
        self.set_flag_config2(Config2::PSIEN, true)
    }

    /// Disable proximity saturation interrupt generation
    pub fn disable_proximity_saturation_interrupts(&mut self) -> Result<(), E> {
        self.set_flag_config2(Config2::PSIEN, false)
    }

    /// Set the proximity interrupt low threshold.
    pub fn set_proximity_low_threshold(&mut self, threshold: u8) -> Result<(), E> {
        self.write_register(Register::PILT, threshold)
    }

    /// Set the proximity interrupt high threshold.
    pub fn set_proximity_high_threshold(&mut self, threshold: u8) -> Result<(), E> {
        self.write_register(Register::PIHT, threshold)
    }

    /// Set the number of times the value falls out of range for an interupt to trigger
    pub fn set_proximity_persistence_filter(&mut self, triggers: u8) -> Result<(), E> {
        let mut initial = self.read_register(0x8C)?;
        initial &= 0b00001111;
        initial |= (triggers << 4) & 0b11110000;
        self.write_register(0x8C, initial)
    }

    /// Checks if a proximity interupt has occured
    pub fn get_proximity_interupt(&mut self) -> Result<bool, E> {
        Ok(self.read_register(0x93)? & 0b0010_0000 > 0)
    }

    /// Set the proximity up/right photodiode offset.
    pub fn set_proximity_up_right_offset(&mut self, offset: i8) -> Result<(), E> {
        self.write_register(Register::POFFSET_UR, offset as u8)
    }

    /// Set the proximity down/left photodiode offset.
    pub fn set_proximity_down_left_offset(&mut self, offset: i8) -> Result<(), E> {
        self.write_register(Register::POFFSET_DL, offset as u8)
    }

    /// Set the proximity up/right and down/left photodiode offset.
    pub fn set_proximity_offsets(
        &mut self,
        offset_up_right: i8,
        offset_down_left: i8,
    ) -> Result<(), E> {
        self.i2c.write(
            DEV_ADDR,
            &[
                Register::POFFSET_UR,
                offset_up_right as u8,
                offset_down_left as u8,
            ],
        )
    }

    /// Clear proximity interrupt.
    pub fn clear_proximity_interrupt(&mut self) -> Result<(), E> {
        self.touch_register(Register::PICLEAR)
    }

    /// Read the proximity sensor data.
    ///
    /// Returns `nb::Error::WouldBlock` as long as the data is not ready.
    pub fn read_proximity(&mut self) -> nb::Result<u8, E> {
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
    pub fn is_proximity_data_valid(&mut self) -> Result<bool, E> {
        let status = self.read_register(Register::STATUS)?;
        Ok(Status::create(status).is(Status::PVALID, true))
    }
}
