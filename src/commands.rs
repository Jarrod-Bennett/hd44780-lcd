//! Commands available to use on the HD44780 device.

use crate::instructions::*;

/// Commands every driver must support
/// /// Commands that only require a write capable bus
pub trait Driver {
    type Error;

    /// Clears entire display and sets DDRAM address 0 in address counter.
    fn clear_display(&mut self) -> Result<(), Self::Error>;

    /// Sets DDRAM address 0 in address counter. Also returns display from being shifted to original
    /// position. DDRAM contents remain unchanged.
    ///
    /// ```rust, ignore
    /// lcd.return_home();
    /// ```
    fn return_home(&mut self) -> Result<(), Self::Error>;

    /// Sets cursor move direction and specifies display shift. These operations are performed
    /// during data write and read.
    fn set_entry_mode(
        &mut self,
        direction: IncrementDecrement,
        display_shift: AccompaniesDisplayShift,
    ) -> Result<(), Self::Error>;

    // todo! individual entry mode settings

    /// Sets entire display (D) on/off, cursor on/off (C), and blinking of cursor position character
    /// (B).
    fn set_display_control(
        &mut self,
        display_on: ShowDisplay,
        cursor_displayed: ShowCursor,
        cursor_blink: Blink,
    ) -> Result<(), Self::Error>;

    // todo! individual display control settings

    // todo! could provide mutation methods for cursor/display shift like entry mode etc.

    /// Move the cursor left or right once.
    fn shift_cursor(&mut self, direction: ShiftDirection) -> Result<(), Self::Error>;

    /// Move the display left or right once.
    fn shift_display(&mut self, direction: ShiftDirection) -> Result<(), Self::Error>;

    /// Sets interface data length (DL), number of display lines (N), and character font (F).
    fn function_set(
        &mut self,
        data_length: DataLength,
        num_lines: NumberOfDisplayLines,
        font: CharacterFont,
    ) -> Result<(), Self::Error>;

    // todo! function set methods

    /// Sets CGRAM address. CGRAM data is sent and received after this setting.
    fn set_cgram_address(&mut self, address: u8) -> Result<(), Self::Error>;

    /// Sets DDRAM address, moving the cursor to the specified position. The next character written
    /// will appear at this position. DDRAM data is sent and received after this setting. Line 2
    /// begins at address 40.
    ///
    /// ```rust, ignore
    /// lcd.position(5);        // Line 1, column 5
    /// lcd.write_char('X');    // 'X' at line 1, column 5
    /// lcd.position(45);       // Line 2, column 5
    /// lcd.write_char('Y');    // 'Y' at line 2, column 5, right below 'X'
    /// ```
    fn set_position(&mut self, address: u8) -> Result<(), Self::Error>;

    /// Writes a byte of data into DDRAM or CGRAM. Type is selected by setting either DDRAM or CGRAM
    /// address. Note: character will be truncated to fit into u8.
    fn write_char(&mut self, data: char) -> Result<(), Self::Error>;

    /// Writes a string of data to the display. Note: each character in the string is written as the
    /// corresponding byte and will be truncated if does not fit into u8.
    fn write_str(&mut self, str: &str) -> Result<(), Self::Error>;

    /// Write a byte of data to the display.
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Error>;

    /// Writes a series of bytes to the display.
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;

    // todo! cgram stuff
}

/// Commands that require a read-write bus
pub trait ReadableDriver {
    type Error;
    /// Reads busy flag (BF) indicating internal operation is being performed and reads address
    /// counter contents.
    fn read_busy_flag_and_address_counter(&mut self) -> Result<u8, Self::Error>;

    /// Reads data from DDRAM or CGRAM. Type is selected by setting either DDRAM or CGRAM address.
    fn read_data(&mut self) -> Result<u8, Self::Error>;
}
