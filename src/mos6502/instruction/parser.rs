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
        0x8 => parse_prefix_8_instruction(opcode),
        0x9 => parse_prefix_9_instruction(opcode),
        0xa => parse_prefix_a_instruction(opcode),
        0xb => parse_prefix_b_instruction(opcode),
        0xc => parse_prefix_c_instruction(opcode),
        0xd => parse_prefix_d_instruction(opcode),
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

fn parse_prefix_8_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x1 => Ok(M6502Ins::StaIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x4 => Ok(M6502Ins::StyZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x5 => Ok(M6502Ins::StaZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::StxZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x8 => Ok(M6502Ins::Dey(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::Txa(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::StyAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::StaAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::StxAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_9_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bcc(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::StaIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x4 => Ok(M6502Ins::StyZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x5 => Ok(M6502Ins::StaZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::StxZPY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x8 => Ok(M6502Ins::Tya(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::StaAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 5,
        })),
        0xa => Ok(M6502Ins::Txs(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xd => Ok(M6502Ins::StaAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 5,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_a_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::LdyImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::LdaIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x2 => Ok(M6502Ins::LdxImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x4 => Ok(M6502Ins::LdyZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x5 => Ok(M6502Ins::LdaZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::LdxZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x8 => Ok(M6502Ins::Tay(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::LdaImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::Tax(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::LdyAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::LdaAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::LdxAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_b_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bcs(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::LdaIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x4 => Ok(M6502Ins::LdyZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x5 => Ok(M6502Ins::LdaZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::LdxZPY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x8 => Ok(M6502Ins::Clv(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::LdaAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xa => Ok(M6502Ins::Tsx(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::LdyAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::LdaAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::LdxAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_c_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::CpyImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::CmpIndX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x4 => Ok(M6502Ins::CpyZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x5 => Ok(M6502Ins::CmpZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0x6 => Ok(M6502Ins::DecZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x8 => Ok(M6502Ins::Iny(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::CmpImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0xa => Ok(M6502Ins::Dex(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0xc => Ok(M6502Ins::CpyAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::CmpAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::CpyAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 6,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_d_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bne(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x1 => Ok(M6502Ins::CmpIndY(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 5,
        })),
        0x5 => Ok(M6502Ins::CmpZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 4,
        })),
        0x6 => Ok(M6502Ins::DecZPX(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 6,
        })),
        0x8 => Ok(M6502Ins::Cld(InsAttr {
            opc: opcode,
            len: 1,
            cyc: 2,
        })),
        0x9 => Ok(M6502Ins::CmpAbsY(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xd => Ok(M6502Ins::CmpAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        0xe => Ok(M6502Ins::DecAbsX(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 7,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_e_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::CpxImm(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 2,
        })),
        0x4 => Ok(M6502Ins::CpxZP(InsAttr {
            opc: opcode,
            len: 2,
            cyc: 3,
        })),
        0xc => Ok(M6502Ins::CpxAbs(InsAttr {
            opc: opcode,
            len: 3,
            cyc: 4,
        })),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}
