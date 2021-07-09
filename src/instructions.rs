//! Instruction options to configure the display.

const CLEAR_DISPLAY_OPCODE: u8 = 1 << 0;
const RETURN_HOME_OPCODE: u8 = 1 << 1;
const ENTRY_MODE_OPCODE: u8 = 1 << 2;
const DISPLAY_CONTROL_OPCODE: u8 = 1 << 3;
const DISPLAY_SHIFT_OPCODE: u8 = 1 << 4;
const FUNCTION_SET_OPCODE: u8 = 1 << 5;
const SET_CGRAM_ADDRESS_OPCODE: u8 = 1 << 6;
const SET_DDRAM_ADDRESS_OPCODE: u8 = 1 << 7;

#[repr(u8)]
pub enum Opcodes {
    ClearDisplay = CLEAR_DISPLAY_OPCODE,
    ReturnHome = RETURN_HOME_OPCODE,
    EntryMode = ENTRY_MODE_OPCODE,
    DisplayControl = DISPLAY_CONTROL_OPCODE,
    DisplayShift = DISPLAY_SHIFT_OPCODE,
    FunctionSet = FUNCTION_SET_OPCODE,
    SetCgramAddress = SET_CGRAM_ADDRESS_OPCODE,
    SetDdramAddress = SET_DDRAM_ADDRESS_OPCODE,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum IncrementDecrement {
    Increment = 1,
    Decrement = 0,
}

impl Default for IncrementDecrement {
    fn default() -> Self {
        IncrementDecrement::Increment
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AccompaniesDisplayShift {
    Shift = 1,
    NoShift = 0,
}

impl Default for AccompaniesDisplayShift {
    fn default() -> Self {
        AccompaniesDisplayShift::NoShift
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShiftType {
    DisplayShift = 1,
    CursorMove = 0,
}

impl Default for ShiftType {
    fn default() -> Self {
        ShiftType::DisplayShift
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShiftDirection {
    ShiftRight = 1,
    ShiftLeft = 0,
}

impl Default for ShiftDirection {
    fn default() -> Self {
        ShiftDirection::ShiftLeft
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShowDisplay {
    On = 1,
    Off = 0,
}

impl Default for ShowDisplay {
    fn default() -> Self {
        ShowDisplay::Off
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShowCursor {
    On = 1,
    Off = 0,
}

impl Default for ShowCursor {
    fn default() -> Self {
        ShowCursor::Off
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Blink {
    On = 1,
    Off = 0,
}

impl Default for Blink {
    fn default() -> Self {
        Blink::Off
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DataLength {
    EightBits = 1,
    FourBits = 0,
}

impl Default for DataLength {
    fn default() -> Self {
        DataLength::EightBits
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum NumberOfDisplayLines {
    TwoLines = 1,
    OneLine = 0,
}

impl Default for NumberOfDisplayLines {
    fn default() -> Self {
        NumberOfDisplayLines::OneLine
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum CharacterFont {
    FiveByTen = 1,
    FiveByEight = 0,
}

impl Default for CharacterFont {
    fn default() -> Self {
        CharacterFont::FiveByEight
    }
}
