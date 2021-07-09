//! Keep track of the current device state to avoid overwriting settings
// todo! sealed trait for these with as_byte fn

use crate::instructions::*;

#[derive(Default)]
pub struct EntryMode {
    pub cursor_direction: IncrementDecrement,
    pub display_shift: AccompaniesDisplayShift,
}

impl EntryMode {
    pub fn as_byte(&self) -> u8 {
        let dir = (*(&self.cursor_direction) as u8) << 1;
        let shift = (*(&self.display_shift) as u8) << 0;
        dir | shift
    }
}

#[derive(Default)]
pub struct DisplayControl {
    pub display: ShowDisplay,
    pub cursor: ShowCursor,
    pub blink: Blink,
}

impl DisplayControl {
    pub fn as_byte(&self) -> u8 {
        let d = (*(&self.display) as u8) << 2;
        let c = (*(&self.cursor) as u8) << 1;
        let b = (*(&self.blink) as u8) << 0;
        d | c | b
    }
}

#[derive(Default)]
pub struct DisplayShift {
    pub shift_type: ShiftType,
    pub shift_direction: ShiftDirection,
}

impl DisplayShift {
    pub fn as_byte(&self) -> u8 {
        let sc = (*(&self.shift_type) as u8) << 3;
        let rl = (*(&self.shift_direction) as u8) << 2;
        sc | rl
    }
}

#[derive(Default)]
pub struct FunctionSet {
    pub data_length: DataLength,
    pub num_lines: NumberOfDisplayLines,
    pub char_font: CharacterFont,
}

impl FunctionSet {
    pub fn as_byte(&self) -> u8 {
        let dl = (*(&self.data_length) as u8) << 4;
        let nl = (*(&self.num_lines) as u8) << 3;
        let cf = (*(&self.char_font) as u8) << 2;
        dl | nl | cf
    }
}
