//! Driver for a HD44780-compatible LCD.
//!
//! Built upon embedded-hal pins to provide generic support for any device.
//! Supports (will support) both embedded-hal v0.2.5 and embedded-hal-alpha v1.0.4alpha (select
//! feature = hal-alpha to activate).
//!
//! Supports both full feature device control and communication and simplified model with write only
//! capability (select feature = write-only). This API is (most likely anyway) less power expensive
//! as pin direction changes are reduced, instead blocking to provide sufficient time for operations
//! to occur. Write-only is more modular as well, requiring only Output pins to be provided, wheras
//! full-feature requires custom IoPin trait to be implemented, allowing the data pins to be both
//! read from and written to, required to poll the device busy flag and addresses. When/if
//! embedded-hal provides an official IoPin trait, this will be used instead. An implementation of
//! the IoPin trait is provided for the stm32f3xx series of devices (config feature = stm32f3xx-io).
//!
//! todo! examples etc.
//! todo! async!

// todo: check design patterns in embedded book
// todo: #[inline]
// todo!: no-read feature - only writes, never reads feature
// todo! implement analog v0 pin
// todo 4 pin data line option
// todo - right now any pin that implements output pin can call set_read and so on, I don't like that - I think its fixed but check
// todo going to need a delay method in the struct - embedded time? async?

pub use embedded_hal as hal;
pub use hal::digital::v2::{InputPin, OutputPin};
use crate::private::Sealed;


mod private {
    pub trait Sealed {}
}
impl<T: OutputPin> Sealed for T {}

/// A bi-directional pin that can be read from (in input mode) or written to (in output mode)
pub trait IoPin: InputPin + OutputPin {
    type Error;
    /// Set pin as input
    fn set_input(&mut self) -> Result<(), <Self as IoPin>::Error>;

    /// Set pin as output
    fn set_output(&mut self) -> Result<(), <Self as IoPin>::Error>;
}

/// ReadWrite pin.
/// Select read or write mode.
pub trait ReadWritePin: OutputPin + Sealed {
    /// Select read mode.
    fn set_read(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_high()
    }

    /// Select write mode.
    fn set_write(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_low()
    }
}
impl<T: OutputPin> ReadWritePin for T {}

// impl From<Self as OutputPin>::Error

/// RegisterSelect pin.
/// Select which of the instruction and data registers are currently selected.
pub trait RegisterSelectPin: OutputPin + Sealed {
    /// Select the instruction register (for write) or [busy flag:address counter] (for read)
    fn select_instruction_register(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_low()
    }

    /// Select the data register (for write and read)
    fn select_data_register(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_high()
    }
}
impl<T: OutputPin> RegisterSelectPin for T {}

/// Enable pin.
/// Starts data read/write.
pub trait EnablePin: OutputPin + Sealed {
    /// Start data read/write
    fn start_transaction(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_high()
    }

    /// Stop data read/write.
    fn end_transaction(&mut self) -> Result<(), <Self as OutputPin>::Error> {
        self.set_low()
    }
}
impl<T: OutputPin> EnablePin for T {}

/// Pins required to control display driver. These pins may be optional and are defined here for
/// reporting missing pins
#[derive(Debug)]
pub enum MissingPin {
    V0,
    RS,
    RW,
    En,
    /* data pins always required */
}

#[derive(Debug)]
pub enum HD44780Error {
    InvalidContrastSetting(usize),
    PinNotProvided(MissingPin),
    PinSetFailure,
}

// impl<RW: ReadWritePin> From<<RW as OutputPin>::Error> for HD44780Error {
//     fn from(error: <RW as OutputPin>::Error) -> Self {
//         HD44780Error::PinSetFailure(error)
//     }
// }

pub struct HD44780<EN, RW, RS, Data>
    where
        EN: EnablePin,
        RW: ReadWritePin,
        RS: RegisterSelectPin,
        Data: IoPin//IoPin,
{
    en: EN,
    rw: RW,
    rs: RS,
    data: [Data; 8],
}

impl<EN, RW, RS, Data> HD44780<EN, RW, RS, Data>
    where
        EN: EnablePin,
        RW: ReadWritePin,
        RS: RegisterSelectPin,
        Data: IoPin,
{
    /// Initialise an instance of the display driver
    pub fn new(en: EN, rw: RW, rs: RS, data: [Data; 8]) -> Self {
        HD44780 {
            en,
            rw,
            rs,
            data,
        }
    }

    /// Write a byte of data to the LCD display
    pub fn write_byte(&mut self, data: u8) -> Result<(), HD44780Error> {
        // Start by verifying the device is ready by polling the busy flag
        // . When RS = 0 and R/W = 1 (Table 1), the busy flag is output to DB7.
        // todo! b7 input and read it

        // Select write mode
        // Set data pins to outputs with bits set appropriately
        // Set enable line to indicate data
        self.rw.set_write();
        for i in 0..8 {
            &self.data[i].set_output();
            if (data & (1 << i)) != 0{
                &self.data[i].set_high();
            } else {
                &self.data[i].set_low();
            }
        }
        self.en.set_high();
        // sleep 250 ns
        self.en.set_low();
        // sleep 250 ns;
        // todo! maybe implement sleep as part of enable pin trait



        Ok(())
    }
}

/// API required for the
pub trait HD44780Driver {
    type Error;

    /// Clear the LCD display
    fn clear_display(&mut self) -> Result<(), Self::Error>;

    // todo other commands
}

/// Commands available to control the display
mod commands {
    /// Command bit representations
    #[repr(u8)]
    pub enum Command {
        ShiftDisplay = 1 << 1,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
