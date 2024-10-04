///
/// this module hold all necessary constants
///
pub const LOWER_NIBBLE_MASK: u8 = 0x01;

// NV-BDIZC
pub const NEGATIVE_ON_MASK: u8 = 0b10000000;
pub const NEGATIVE_OFF_MASK: u8 = 0b01111111;

pub const OVERFLOW_ON_MASK: u8 = 0b01000000;
pub const OVERFLOW_OFF_MASK: u8 = 0b10111111;

pub const BREAK_ON_MASK: u8 = 0b00010000;
pub const BREAK_OFF_MASK: u8 = 0b11101111;

pub const DECIMAL_ON_MASK: u8 = 0b00001000;
pub const DECIMAL_OFF_MASK: u8 = 0b11110111;

pub const INTERRUPT_ON_MASK: u8 = 0b00000100;
pub const INTERRUPT_OFF_MASK: u8 = 0b11111011;

pub const ZERO_ON_MASK: u8 = 0b00000010;
pub const ZERO_OFF_MASK: u8 = 0b11111101;

pub const CARRY_ON_MASK: u8 = 0b00000001;
pub const CARRY_OFF_MASK: u8 = 0b11111110;

pub const LAST_BIT_MASK: u8 = 0b00000001;