//! Collection of pins connected to and controlling the HD44780.

pub mod bus4;
pub mod bus8;
mod pins;

use embedded_hal as hal;
use hal::blocking::delay::{DelayMs, DelayUs};

/// Select the instruction or data register
#[repr(u8)]
pub enum TransactionType {
    Instruction = 0,
    Data = 1,
}

/// API of a data bus for the HD44780.
pub trait DataBus {
    type Error;

    /// Write a single byte of data to the instruction or data register.
    fn write_byte<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        byte: u8,
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error>;

    /// Write multiple bytes of data to the instruction or data register.
    fn write_bytes<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        bytes: &[u8],
        transaction: TransactionType,
        delay: &mut D,
    ) -> Result<(), Self::Error>;
}

/// Additional API of a data bus with bidirectional data bit pins, allowing read operations from the
/// HD44780 as well as write operations.
pub trait ReadableDataBus {
    type Error;

    /// Poll busy flag. Returns true if busy otherwise false.
    fn read_busy_flag<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<bool, Self::Error>;

    /// Read address counter.
    fn read_address_counter<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<u8, Self::Error>;

    /// Read both busy flag and address counter
    fn read_busy_flag_and_address_counter<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<(bool, u8), Self::Error>;

    /// Reads data from DDRAM or CGRAM.
    fn read_data<D: DelayMs<u16> + DelayUs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<u8, Self::Error>;
}
