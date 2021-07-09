//! Driver for a HD44780-compatible LCD.
//!
//! Built upon embedded-hal pins to provide generic support for any device.
//! Supports (will support) both embedded-hal v0.2.5 and embedded-hal-alpha v1.0.4alpha (select
//! feature = hal-alpha to activate).
//!
//! Supports both full feature device control and communication and simplified model with write only
//! capability (select feature = write-only). This API is (most likely anyway) less power expensive
//! as pin direction changes are reduced, instead blocking to provide sufficient time for operations
//! to occur. Write-only is more modular as well, requiring only Output pins to be provided, whereas
//! full-feature requires custom IoPin trait to be implemented, allowing the data pins to be both
//! read from and written to, required to poll the device busy flag and addresses. When/if
//! embedded-hal provides an official IoPin trait, this will be used instead. An implementation of
//! the IoPin trait is provided for the stm32f3xx series of devices (config feature = stm32f3xx-io).

// todo! examples
// todo! non-blocking mode
// todo! remove driver trait and directly implement instead
// todo! features for embedded-hal-alpha, read-write/write-only modes
// todo! improve error handling/propagation

#![no_std]

pub mod commands;
mod data_bus;
pub mod instructions;
mod state;

use crate::commands::*;
use crate::data_bus::bus8::WriteOnlyBus8;
use crate::data_bus::*;
use crate::instructions::*;
use crate::state::*;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::OutputPin;

/// Errors
#[derive(Debug)]
pub enum DataBusError {
    // Error setting a pin high or low
    PinSetError,

    // Issue with blocking
    DelayError,

    // Busy flag remained high for excessive period of time
    BlockTooLongError,

    // RAM address out of range
    AddressOutOfRange,
}

/// Fully supported with bidirectional pins
pub struct ReadableHd44780<B, D>
where
    B: DataBus + ReadableDataBus,
    D: DelayMs<u16> + DelayUs<u16>,
{
    bus: B,
    delay: D,
    entry_mode: EntryMode,
    display_mode: DisplayControl,
}

impl<B, D> ReadableHd44780<B, D>
where
    B: DataBus + ReadableDataBus,
    D: DelayMs<u16> + DelayUs<u16>,
{
    pub fn from_pins() {}

    pub fn release() {}
}

/// Write only commands, no IO pins required.
pub struct WriteOnlyHD44780<B, D>
where
    B: DataBus,
    D: DelayMs<u16> + DelayUs<u16>,
{
    bus: B,
    delay: D,
    entry_mode: EntryMode,
    display_control: DisplayControl,
    display_shift: DisplayShift,
    function_set: FunctionSet,
}

impl<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7, D>
    WriteOnlyHD44780<WriteOnlyBus8<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7>, D>
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
    D: DelayMs<u16> + DelayUs<u16>,
{
    /// Create an instance of a write-only capable HD44780 with an 8-bit wide data bus.
    pub fn new_bus8(
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
        delay: D,
    ) -> WriteOnlyHD44780<WriteOnlyBus8<EN, RS, D0, D1, D2, D3, D4, D5, D6, D7>, D> {
        let mut lcd = WriteOnlyHD44780 {
            bus: WriteOnlyBus8::from_pins(en, rs, d0, d1, d2, d3, d4, d5, d6, d7),
            delay,
            entry_mode: Default::default(),
            display_control: Default::default(),
            display_shift: Default::default(),
            function_set: Default::default(),
        };

        lcd.delay.delay_ms(15);
        lcd.function_set(
            lcd.function_set.data_length,
            lcd.function_set.num_lines,
            lcd.function_set.char_font,
        );
        lcd.delay.delay_ms(100);
        lcd.set_entry_mode(
            lcd.entry_mode.cursor_direction,
            lcd.entry_mode.display_shift,
        );
        lcd.delay.delay_ms(100);
        lcd.set_display_control(ShowDisplay::On, ShowCursor::On, Blink::On);
        lcd.delay.delay_ms(5);

        lcd
    }

    /// Destroy the HD44780 instance and return the pins and delays provided.
    pub fn release(self) -> ((EN, RS, D0, D1, D2, D3, D4, D5, D6, D7), D) {
        (self.bus.release(), self.delay)
    }
}

impl<B, D> Driver for WriteOnlyHD44780<B, D>
where
    B: DataBus,
    D: DelayMs<u16> + DelayUs<u16>,
{
    type Error = DataBusError;

    fn clear_display(&mut self) -> Result<(), DataBusError> {
        self.bus.write_byte(
            Opcodes::ClearDisplay as u8,
            TransactionType::Instruction,
            &mut self.delay,
        );
        let data = Opcodes::ClearDisplay as u8;
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn return_home(&mut self) -> Result<(), DataBusError> {
        let data = Opcodes::ReturnHome as u8;
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn set_entry_mode(
        &mut self,
        direction: IncrementDecrement,
        display_shift: AccompaniesDisplayShift,
    ) -> Result<(), DataBusError> {
        let entry_mode = EntryMode {
            cursor_direction: direction,
            display_shift,
        };
        let data = Opcodes::EntryMode as u8 | entry_mode.as_byte();
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn set_display_control(
        &mut self,
        display_on: ShowDisplay,
        cursor_displayed: ShowCursor,
        cursor_blink: Blink,
    ) -> Result<(), DataBusError> {
        let display_control = DisplayControl {
            display: display_on,
            cursor: cursor_displayed,
            blink: cursor_blink,
        };
        let data = Opcodes::DisplayControl as u8 | display_control.as_byte();
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn shift_cursor(&mut self, shift_direction: ShiftDirection) -> Result<(), DataBusError> {
        let operands = DisplayShift {
            shift_type: ShiftType::CursorMove,
            shift_direction,
        };
        let data = Opcodes::DisplayShift as u8 | operands.as_byte();
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn shift_display(&mut self, shift_direction: ShiftDirection) -> Result<(), DataBusError> {
        let operands = DisplayShift {
            shift_type: ShiftType::DisplayShift,
            shift_direction,
        };
        let data = Opcodes::DisplayShift as u8 | operands.as_byte();
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn function_set(
        &mut self,
        data_length: DataLength,
        num_lines: NumberOfDisplayLines,
        font: CharacterFont,
    ) -> Result<(), DataBusError> {
        let function_set = FunctionSet {
            data_length,
            num_lines,
            char_font: font,
        };
        let data = Opcodes::FunctionSet as u8 | function_set.as_byte();
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn set_cgram_address(&mut self, address: u8) -> Result<(), DataBusError> {
        if address >= Opcodes::SetCgramAddress as u8 {
            return Err(DataBusError::AddressOutOfRange);
        }
        let data = Opcodes::SetCgramAddress as u8 | address;
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn set_position(&mut self, address: u8) -> Result<(), DataBusError> {
        // todo! proper out of bounds checking
        let data = Opcodes::SetDdramAddress as u8 | address;
        self.bus
            .write_byte(data, TransactionType::Instruction, &mut self.delay);
        Ok(())
    }

    fn write_char(&mut self, data: char) -> Result<(), DataBusError> {
        self.write_byte(data as u8)
    }

    fn write_str(&mut self, str: &str) -> Result<(), DataBusError> {
        self.write_bytes(str.as_bytes())
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), DataBusError> {
        self.bus
            .write_byte(byte, TransactionType::Data, &mut self.delay);
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), DataBusError> {
        self.bus
            .write_bytes(bytes, TransactionType::Data, &mut self.delay);
        Ok(())
    }
}
