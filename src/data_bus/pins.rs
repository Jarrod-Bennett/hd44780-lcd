//! Special pin definitions for the pins featured on the data bus.

use embedded_hal as hal;
// use hal::blocking::digital::OutputPin;
use crate::DataBusError;
use hal::digital::v2::OutputPin;

mod private {
    use super::*;
    pub trait Sealed {}

    impl<T: OutputPin> Sealed for T {}
}

/// ReadWrite pin.
/// Select read or write mode.
pub struct ReadWritePin<T: OutputPin> {
    pin: T,
}

impl<T: OutputPin> ReadWritePin<T> {
    /// Create a new read/write pin.
    pub fn from_output_pin(pin: T) -> ReadWritePin<T> {
        ReadWritePin { pin }
    }

    /// Destroy pin.
    pub fn release(self) -> T {
        self.pin
    }

    /// Select read mode.
    pub fn set_read(&mut self) -> Result<(), DataBusError> {
        self.pin.set_high().map_err(|_| DataBusError::PinSetError)
    }

    /// Select write mode.
    pub fn set_write(&mut self) -> Result<(), DataBusError> {
        self.pin.set_low().map_err(|_| DataBusError::PinSetError)
    }
}

/// RegisterSelect pin.
/// Select which of the instruction and data registers are currently selected.
pub struct RegisterSelectPin<T: OutputPin> {
    pin: T,
}

impl<T: OutputPin> RegisterSelectPin<T> {
    /// Create a new register selection pin
    pub fn from_output_pin(pin: T) -> Self {
        RegisterSelectPin { pin }
    }

    /// Destroy pin.
    pub fn release(self) -> T {
        self.pin
    }

    /// Select the instruction register (for write) or [busy flag:address counter] (for read)
    pub fn select_instruction_register(&mut self) -> Result<(), DataBusError> {
        self.pin.set_low().map_err(|_| DataBusError::PinSetError)
    }

    /// Select the data register (for write and read)
    pub fn select_data_register(&mut self) -> Result<(), DataBusError> {
        self.pin.set_high().map_err(|_| DataBusError::PinSetError)
    }
}

/// Enable pin.
/// Starts data read/write.
pub struct EnablePin<T: OutputPin> {
    pin: T,
}

impl<T: OutputPin> EnablePin<T> {
    /// Create a new enable pin
    pub fn from_output_pin(pin: T) -> Self {
        EnablePin { pin }
    }

    /// Destroy pin.
    pub fn release(self) -> T {
        self.pin
    }

    /// Start data read/write
    pub fn start_transaction(&mut self) -> Result<(), DataBusError> {
        self.pin.set_high().map_err(|_| DataBusError::PinSetError)
    }

    /// Stop data read/write.
    pub fn end_transaction(&mut self) -> Result<(), DataBusError> {
        self.pin.set_low().map_err(|_| DataBusError::PinSetError)
    }
}
