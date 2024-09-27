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
    let upper_nibble: u8 = opcode >> 4;
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
        0xe => parse_prefix_e_instruction(opcode),
        0xf => parse_prefix_f_instruction(opcode),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_0_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Brk(InsAttr::new(opcode, 1, 7))),
        0x1 => Ok(M6502Ins::OraIndX(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::OraZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::AslZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Php(InsAttr::new(opcode, 1, 3))),
        0x9 => Ok(M6502Ins::OraImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::AslAcc(InsAttr::new(opcode, 1, 2))),
        0xd => Ok(M6502Ins::OraAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::AslAbs(InsAttr::new(opcode, 3, 6))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_1_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bpl(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::OraIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::OraZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::AslZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Clc(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::OraAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::OraAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::AslAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_2_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Jsr(InsAttr::new(opcode, 3, 6))),
        0x1 => Ok(M6502Ins::AndIndX(InsAttr::new(opcode, 2, 6))),
        0x4 => Ok(M6502Ins::BitZP(InsAttr::new(opcode, 2, 3))),
        0x5 => Ok(M6502Ins::AndZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::RolZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Plp(InsAttr::new(opcode, 1, 4))),
        0x9 => Ok(M6502Ins::AndImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::RolAcc(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::BitAbs(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::AndAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::RolAbs(InsAttr::new(opcode, 3, 6))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_3_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bmi(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::AndIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::AndZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::RolZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Sec(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::AndAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::AndAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::RolAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_4_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Rti(InsAttr::new(opcode, 1, 6))),
        0x1 => Ok(M6502Ins::EorImm(InsAttr::new(opcode, 2, 2))),
        0x5 => Ok(M6502Ins::EorZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::LsrZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Pha(InsAttr::new(opcode, 1, 3))),
        0x9 => Ok(M6502Ins::EorImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::LsrAcc(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::JmpAbs(InsAttr::new(opcode, 3, 3))),
        0xd => Ok(M6502Ins::EorAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::LsrAbs(InsAttr::new(opcode, 3, 6))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_5_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bvc(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::EorIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::EorZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::LsrZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Cli(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::EorAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::EorAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::LsrAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_6_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Rts(InsAttr::new(opcode, 1, 6))),
        0x1 => Ok(M6502Ins::AdcIndX(InsAttr::new(opcode, 2, 6))),
        0x5 => Ok(M6502Ins::AdcZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::RorZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Pla(InsAttr::new(opcode, 1, 4))),
        0x9 => Ok(M6502Ins::AdcImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::RorAcc(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::JmpInd(InsAttr::new(opcode, 3, 5))),
        0xd => Ok(M6502Ins::AdcAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::RorAbs(InsAttr::new(opcode, 3, 6))),
        // TODO add more instructions
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_7_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bvs(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::AdcIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::AdcZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::RorZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Sei(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::AdcAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::AdcAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::RorAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_8_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x1 => Ok(M6502Ins::StaIndX(InsAttr::new(opcode, 2, 6))),
        0x4 => Ok(M6502Ins::StyZP(InsAttr::new(opcode, 2, 3))),
        0x5 => Ok(M6502Ins::StaZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::StxZP(InsAttr::new(opcode, 2, 3))),
        0x8 => Ok(M6502Ins::Dey(InsAttr::new(opcode, 1, 2))),
        0xa => Ok(M6502Ins::Txa(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::StyAbs(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::StaAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::StxAbs(InsAttr::new(opcode, 3, 4))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_9_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bcc(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::StaIndY(InsAttr::new(opcode, 2, 6))),
        0x4 => Ok(M6502Ins::StyZPX(InsAttr::new(opcode, 2, 4))),
        0x5 => Ok(M6502Ins::StaZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::StxZPY(InsAttr::new(opcode, 2, 4))),
        0x8 => Ok(M6502Ins::Tya(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::StaAbsY(InsAttr::new(opcode, 3, 5))),
        0xa => Ok(M6502Ins::Txs(InsAttr::new(opcode, 1, 2))),
        0xd => Ok(M6502Ins::StaAbsX(InsAttr::new(opcode, 3, 5))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_a_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::LdyImm(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::LdaIndX(InsAttr::new(opcode, 2, 6))),
        0x2 => Ok(M6502Ins::LdxImm(InsAttr::new(opcode, 2, 2))),
        0x4 => Ok(M6502Ins::LdyZP(InsAttr::new(opcode, 2, 3))),
        0x5 => Ok(M6502Ins::LdaZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::LdxZP(InsAttr::new(opcode, 2, 3))),
        0x8 => Ok(M6502Ins::Tay(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::LdaImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::Tax(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::LdyAbs(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::LdaAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::LdxAbs(InsAttr::new(opcode, 3, 4))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_b_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bcs(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::LdaIndY(InsAttr::new(opcode, 2, 5))),
        0x4 => Ok(M6502Ins::LdyZPX(InsAttr::new(opcode, 2, 4))),
        0x5 => Ok(M6502Ins::LdaZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::LdxZPY(InsAttr::new(opcode, 2, 4))),
        0x8 => Ok(M6502Ins::Clv(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::LdaAbsX(InsAttr::new(opcode, 3, 4))),
        0xa => Ok(M6502Ins::Tsx(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::LdyAbsX(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::LdaAbsY(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::LdxAbsY(InsAttr::new(opcode, 3, 4))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_c_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::CpyImm(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::CmpIndX(InsAttr::new(opcode, 2, 6))),
        0x4 => Ok(M6502Ins::CpyZP(InsAttr::new(opcode, 2, 3))),
        0x5 => Ok(M6502Ins::CmpZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::DecZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Iny(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::CmpImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::Dex(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::CpyAbs(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::CmpAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::CpyAbs(InsAttr::new(opcode, 3, 6))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_d_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Bne(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::CmpIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::CmpZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::DecZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Cld(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::CmpAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::CmpAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::DecAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_e_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::CpxImm(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::SbcIndX(InsAttr::new(opcode, 2, 6))),
        0x4 => Ok(M6502Ins::CpxZP(InsAttr::new(opcode, 2, 3))),
        0x5 => Ok(M6502Ins::SbcZP(InsAttr::new(opcode, 2, 3))),
        0x6 => Ok(M6502Ins::IncZP(InsAttr::new(opcode, 2, 5))),
        0x8 => Ok(M6502Ins::Inx(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::SbcImm(InsAttr::new(opcode, 2, 2))),
        0xa => Ok(M6502Ins::Nop(InsAttr::new(opcode, 1, 2))),
        0xc => Ok(M6502Ins::CpxAbs(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::SbcAbs(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::IncAbs(InsAttr::new(opcode, 3, 6))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_f_instruction(opcode: u8) -> Result<M6502Ins, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(M6502Ins::Beq(InsAttr::new(opcode, 2, 2))),
        0x1 => Ok(M6502Ins::SbcIndY(InsAttr::new(opcode, 2, 5))),
        0x5 => Ok(M6502Ins::SbcZPX(InsAttr::new(opcode, 2, 4))),
        0x6 => Ok(M6502Ins::IncZPX(InsAttr::new(opcode, 2, 6))),
        0x8 => Ok(M6502Ins::Sed(InsAttr::new(opcode, 1, 2))),
        0x9 => Ok(M6502Ins::SbcAbsY(InsAttr::new(opcode, 3, 4))),
        0xd => Ok(M6502Ins::SbcAbsX(InsAttr::new(opcode, 3, 4))),
        0xe => Ok(M6502Ins::IncAbsX(InsAttr::new(opcode, 3, 7))),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_given_x00_should_return_brk() {
        if let M6502Ins::Brk(ia) = parse(0x00).unwrap() {
            assert_eq!(ia.opcode(), 0x00);
            assert_eq!(ia.len(), 1);
            assert_eq!(ia.cyc(), 7);
        }
    }
}
