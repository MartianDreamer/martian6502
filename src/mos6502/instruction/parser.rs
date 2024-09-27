use std::{error::Error, fmt::Display};

use crate::constant::LOWER_NIBBLE_MASK;

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
/// parse an opcode to create a M6502Ins
///
pub fn parse(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let upper_nibble = opcode >> 4;
    match upper_nibble {
        0x0 => parse_prefix_0_instruction(opcode),
        0x1 => parse_prefix_1_instruction(opcode),
        0x2 => parse_prefix_2_instruction(opcode),
        0x3 => parse_prefix_3_instruction(opcode),
        0x4 => parse_prefix_4_instruction(opcode),
        0x5 => parse_prefix_5_instruction(opcode),
        0x6 => parse_prefix_6_instruction(opcode),
        0x7 => parse_prefix_7_instruction(opcode),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_0_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Brk(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 7,
        })),
        0x1 => Ok(M6502Ins::OraIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::OraZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::AslZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x8 => Ok(M6502Ins::Php(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 3,
        })),
        0x9 => Ok(M6502Ins::OraImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::AslAcc(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xd => Ok(M6502Ins::OraAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::AslAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_1_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bpl(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::OraIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::OraZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::AslZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x8 => Ok(M6502Ins::Clc(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::OraAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::OraAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::AslAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_2_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Jsr(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        0x1 => Ok(M6502Ins::AndIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x4 => Ok(M6502Ins::BitZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x5 => Ok(M6502Ins::AndZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::RolZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x8 => Ok(M6502Ins::Plp(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 4,
        })),
        0x9 => Ok(M6502Ins::AndImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::RolAcc(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::BitAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::AndAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::RolAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_3_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bmi(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::AndIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::AndZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::RolZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x8 => Ok(M6502Ins::Sec(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::AndAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::AndAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::RolAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_4_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Rti(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 6,
        })),
        0x1 => Ok(M6502Ins::EorImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x5 => Ok(M6502Ins::EorZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::LsrZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x8 => Ok(M6502Ins::Pha(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 3,
        })),
        0x9 => Ok(M6502Ins::EorImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::LsrAcc(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::JmpAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 3,
        })),
        0xd => Ok(M6502Ins::EorAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::LsrAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_5_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bvc(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::EorIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::EorZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::LsrZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x8 => Ok(M6502Ins::Cli(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::EorAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::EorAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::LsrAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_6_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Rts(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 6,
        })),
        0x1 => Ok(M6502Ins::AdcIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x5 => Ok(M6502Ins::AdcZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::RorZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x8 => Ok(M6502Ins::Pla(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 4,
        })),
        0x9 => Ok(M6502Ins::AdcImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::RorAcc(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::JmpInd(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 5,
        })),
        0xd => Ok(M6502Ins::AdcAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::RorAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        // TODO add more instructions
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_7_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bvs(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::AdcIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::AdcZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::RorZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x8 => Ok(M6502Ins::Sei(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::AdcAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::AdcAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::RorAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}
