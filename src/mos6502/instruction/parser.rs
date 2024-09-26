use std::{error::Error, fmt::Display};

use crate::constant::{LOWER_NIBBLE_MASK, UPPER_NIBBLE_MASK};

use super::insset::{InsAttr, M6502Ins};

#[derive(Debug, Clone)]
pub struct InvalidOpcodeError {
    pub opcode: u8,
}

impl Error for InvalidOpcodeError {}

impl Display for InvalidOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instructions opcode is not valid")
    }
}

///
/// parse an opcode to create a Mos6502Instruction
///
pub fn parse(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let upper_nibble = opcode & UPPER_NIBBLE_MASK;
    match upper_nibble {
        0x0 => parse_prefix_0_instruction(opcode),
        0x1 => parse_prefix_1_instruction(opcode),
        0x2 => parse_prefix_2_instruction(opcode),
        0x3 => parse_prefix_3_instruction(opcode),
        0x4 => parse_prefix_4_instruction(opcode),
        0x5 => parse_prefix_5_instruction(opcode),
        0x6 => parse_prefix_6_instruction(opcode),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_0_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Brk(InsAttr {
            opcode,
            len: 1,
            ccls: 7,
        })),
        0x1 => Ok(M6502Ins::OraIndX(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x5 => Ok(M6502Ins::OraZP(InsAttr {
            opcode,
            len: 2,
            ccls: 3,
        })),
        0x6 => Ok(M6502Ins::AslZP(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x8 => Ok(M6502Ins::Php(InsAttr {
            opcode,
            len: 1,
            ccls: 3,
        })),
        0x9 => Ok(M6502Ins::OraImm(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0xa => Ok(M6502Ins::AslAcc(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0xd => Ok(M6502Ins::OraAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::AslAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_1_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bpl(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0x1 => Ok(M6502Ins::OraIndY(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x5 => Ok(M6502Ins::OraZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 4,
        })),
        0x6 => Ok(M6502Ins::AslZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 6,
        })),
        0x8 => Ok(M6502Ins::Clc(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0x9 => Ok(M6502Ins::OraAbsY(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xd => Ok(M6502Ins::OraAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::AslAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_2_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Jsr(InsAttr {
            opcode,
            len: 3,
            ccls: 6,
        })),
        0x1 => Ok(M6502Ins::AndIndX(InsAttr {
            opcode,
            len: 2,
            ccls: 6,
        })),
        0x4 => Ok(M6502Ins::BitZP(InsAttr {
            opcode,
            len: 2,
            ccls: 3,
        })),
        0x5 => Ok(M6502Ins::AndZP(InsAttr {
            opcode,
            len: 2,
            ccls: 3,
        })),
        0x6 => Ok(M6502Ins::RolZP(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x8 => Ok(M6502Ins::Plp(InsAttr {
            opcode,
            len: 1,
            ccls: 4,
        })),
        0x9 => Ok(M6502Ins::AndImm(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0xa => Ok(M6502Ins::RolAcc(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0xc => Ok(M6502Ins::BitAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xd => Ok(M6502Ins::AndAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::RolAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_3_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bmi(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0x1 => Ok(M6502Ins::AndIndY(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x5 => Ok(M6502Ins::AndZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 4,
        })),
        0x6 => Ok(M6502Ins::RolZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 6,
        })),
        0x8 => Ok(M6502Ins::Sec(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0x9 => Ok(M6502Ins::AndAbsY(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xd => Ok(M6502Ins::AndAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::RolAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_4_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Rti(InsAttr {
            opcode,
            len: 1,
            ccls: 6,
        })),
        0x1 => Ok(M6502Ins::EorImm(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0x5 => Ok(M6502Ins::EorZP(InsAttr {
            opcode,
            len: 2,
            ccls: 3,
        })),
        0x6 => Ok(M6502Ins::LsrZP(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x8 => Ok(M6502Ins::Pha(InsAttr {
            opcode,
            len: 1,
            ccls: 3,
        })),
        0x9 => Ok(M6502Ins::EorImm(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0xa => Ok(M6502Ins::LsrAcc(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0xc => Ok(M6502Ins::JmpAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 3,
        })),
        0xd => Ok(M6502Ins::EorAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::LsrAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_5_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bvc(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0x1 => Ok(M6502Ins::EorIndY(InsAttr {
            opcode,
            len: 2,
            ccls: 5,
        })),
        0x5 => Ok(M6502Ins::EorZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 4,
        })),
        0x6 => Ok(M6502Ins::LsrZPX(InsAttr {
            opcode,
            len: 2,
            ccls: 6,
        })),
        0x8 => Ok(M6502Ins::Cli(InsAttr {
            opcode,
            len: 1,
            ccls: 2,
        })),
        0x9 => Ok(M6502Ins::EorAbsY(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xd => Ok(M6502Ins::EorAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        0xe => Ok(M6502Ins::LsrAbsX(InsAttr {
            opcode,
            len: 3,
            ccls: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_6_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x1 => Ok(M6502Ins::AdcIndX(InsAttr {
            opcode,
            len: 2,
            ccls: 6,
        })),
        0x5 => Ok(M6502Ins::AdcZP(InsAttr {
            opcode,
            len: 2,
            ccls: 3,
        })),
        0x9 => Ok(M6502Ins::AdcImm(InsAttr {
            opcode,
            len: 2,
            ccls: 2,
        })),
        0xd => Ok(M6502Ins::AdcAbs(InsAttr {
            opcode,
            len: 3,
            ccls: 4,
        })),
        // TODO add more instructions
        _ => Err(InvalidOpcodeError { opcode }),
    }
}
