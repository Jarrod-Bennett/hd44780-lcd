//! 8-bit data bus implementation.
//! todo! try pin arrays rather than 8 individual pins

use crate::data_bus::*;
use embedded_hal as hal;
// use embedded_hal::blocking::digital::PinState;
use hal::blocking::delay::{DelayMs, DelayUs};
// use hal::blocking::digital::OutputPin;
use crate::DataBusError;
use hal::digital::v2::OutputPin;
use pins::*;

pub struct ReadWriteBus8 {
    // has io pins
    pin: u8,
}

impl ReadWriteBus8 {
    fn from_pins() {}

    fn release() {}
}

impl ReadableDataBus for ReadWriteBus8 {
    type Error = DataBusError;

    fn read_busy_flag<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<bool, Self::Error> {
        todo!()
    }

    fn read_address_counter<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<u8, Self::Error> {
        todo!()
    }

    fn read_busy_flag_and_address_counter<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<(bool, u8), Self::Error> {
        todo!()
    }

    fn read_data<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<u8, Self::Error> {
        todo!()
    }
}

impl DataBus for ReadWriteBus8 {
    type Error = ();

    fn write_byte<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        data: u8,
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn write_bytes<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        bytes: &[u8],
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

//--------------------------------------------------------------------------------------------------

/// 8-bit data bus with write-only data pins. Note: RW pin on device should be tied low.
pub struct WriteOnlyBus8<
    EN: OutputPin,
    RS: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    en: EnablePin<EN>,
    rs: RegisterSelectPin<RS>,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<
        EN: OutputPin,
        RS: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > WriteOnlyBus8<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7>
{
    /// Create a new instance of a write-only 8-bit bus from a group of pins.
    pub fn from_pins(
        en: EN,
        rs: RS,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> WriteOnlyBus8<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7> {
        WriteOnlyBus8 {
            en: EnablePin::from_output_pin(en),
            rs: RegisterSelectPin::from_output_pin(rs),
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        }
    }

    /// Destroy the bus and return the pins.
    pub fn release(self) -> (EN, RS, D0, D1, D2, D3, D4, D5, D6, D7) {
        (
            self.en.release(),
            self.rs.release(),
            self.d0,
            self.d1,
            self.d2,
            self.d3,
            self.d4,
            self.d5,
            self.d6,
            self.d7,
        )
    }

    /// Set the data pins to the provided value
    pub fn set_pins(&mut self, data: u8) -> Result<(), DataBusError> {
        if data & (1 << 0) != 0 {
            self.d0.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d0.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 1) != 0 {
            self.d1.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d1.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 2) != 0 {
            self.d2.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d2.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 3) != 0 {
            self.d3.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d3.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 4) != 0 {
            self.d4.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d4.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 5) != 0 {
            self.d5.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d5.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 6) != 0 {
            self.d6.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d6.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        if data & (1 << 7) != 0 {
            self.d7.set_high().map_err(|_| DataBusError::PinSetError)?;
        } else {
            self.d7.set_low().map_err(|_| DataBusError::PinSetError)?;
        }
        Ok(())
        // todo! embedded-hal-alpha option
        // self.d0
        //     .set_state(PinState::from(data & (1 << 0) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d1
        //     .set_state(PinState::from(data & (1 << 1) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d2
        //     .set_state(PinState::from(data & (1 << 2) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d3
        //     .set_state(PinState::from(data & (1 << 3) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d4
        //     .set_state(PinState::from(data & (1 << 4) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d5
        //     .set_state(PinState::from(data & (1 << 5) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d6
        //     .set_state(PinState::from(data & (1 << 6) != 0))
        //     .map_err(|_| DataBusError::PinSetError)?;
        // self.d7
        //     .set_state(PinState::from(data & (1 << 7) != 0))
        //     .map_err(|_| DataBusError::PinSetError)
    }

    /// Enable pulse.
    fn enable<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<(), DataBusError> {
        self.en.start_transaction()?;
        delay.delay_ms(2); //.map_err(|_| DataBusError::DelayError)?;
        self.en.end_transaction()?;
        Ok(())
    }
}

impl<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7> DataBus
    for WriteOnlyBus8<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7>
where
    EN: OutputPin,
    RS: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
{
    type Error = DataBusError;

    fn write_byte<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        byte: u8,
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        match transaction {
            TransactionType::Instruction => self.rs.select_instruction_register()?,
            TransactionType::Data => self.rs.select_data_register()?,
        };
        self.set_pins(byte)?;
        self.enable(delay).map_err(|_| DataBusError::DelayError)?;
        Ok(())
    }

    fn write_bytes<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        bytes: &[u8],
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error> {
        match transaction {
            TransactionType::Instruction => self.rs.select_instruction_register()?,
            TransactionType::Data => self.rs.select_data_register()?,
        };
        for byte in bytes {
            self.set_pins(*byte)?;
            self.enable(delay).map_err(|_| DataBusError::DelayError)?;
        }

        Ok(())
    }
}
