use std::{error::Error, fmt::Display};
use crate::constant::LOWER_NIBBLE_MASK;
use super::insset::{Mos6502Ins, InsAttr};

use super::insset::adc::{*};
use super::insset::and::{*};
use super::insset::asl::{*};
use super::insset::bit::{*};
use super::insset::branch::{*};
use super::insset::clr::{*};
use super::insset::cmp::{*};
use super::insset::cpxy::{*};
use super::insset::decrement::{*};
use super::insset::eor::{*};
use super::insset::increment::{*};
use super::insset::lda::{*};
use super::insset::ldxy::{*};
use super::insset::lsr::{*};
use super::insset::misc::{*};
use super::insset::ora::{*};
use super::insset::phpl::{*};
use super::insset::rorl::{*};
use super::insset::sbc::{*};
use super::insset::sta::{*};
use super::insset::stxy::{*};
use super::insset::transfer::{*};


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
pub fn parse(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
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

fn parse_prefix_0_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Brk{attr: InsAttr::new(opcode, 1, 7)})),
        0x1 => Ok(Box::new(OraIndX{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(OraZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(AslZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Php{attr: InsAttr::new(opcode, 1, 3)})),
        0x9 => Ok(Box::new(OraImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(AslAcc{attr: InsAttr::new(opcode, 1, 2)})),
        0xd => Ok(Box::new(OraAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(AslAbs{attr: InsAttr::new(opcode, 3, 6)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_1_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bpl{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(OraIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(OraZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(AslZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Clc{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(OraAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(OraAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(AslAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_2_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Jsr{attr: InsAttr::new(opcode, 3, 6)})),
        0x1 => Ok(Box::new(AndIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x4 => Ok(Box::new(BitZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x5 => Ok(Box::new(AndZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(RolZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Plp{attr: InsAttr::new(opcode, 1, 4)})),
        0x9 => Ok(Box::new(AndImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(RolAcc{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(BitAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(AndAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(RolAbs{attr: InsAttr::new(opcode, 3, 6)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_3_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bmi{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(AndIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(AndZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(RolZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Sec{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(AndAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(AndAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(RolAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_4_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Rti{attr: InsAttr::new(opcode, 1, 6)})),
        0x1 => Ok(Box::new(EorImm{attr: InsAttr::new(opcode, 2, 2)})),
        0x5 => Ok(Box::new(EorZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(LsrZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Pha{attr: InsAttr::new(opcode, 1, 3)})),
        0x9 => Ok(Box::new(EorImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(LsrAcc{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(JmpAbs{attr: InsAttr::new(opcode, 3, 3)})),
        0xd => Ok(Box::new(EorAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(LsrAbs{attr: InsAttr::new(opcode, 3, 6)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_5_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bvc{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(EorIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(EorZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(LsrZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Cli{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(EorAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(EorAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(LsrAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_6_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Rts{attr: InsAttr::new(opcode, 1, 6)})),
        0x1 => Ok(Box::new(AdcIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x5 => Ok(Box::new(AdcZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(RorZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Pla{attr: InsAttr::new(opcode, 1, 4)})),
        0x9 => Ok(Box::new(AdcImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(RorAcc{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(JmpInd{attr: InsAttr::new(opcode, 3, 5)})),
        0xd => Ok(Box::new(AdcAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(RorAbs{attr: InsAttr::new(opcode, 3, 6)})),
        // TODO add more instructions
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_7_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bvs{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(AdcIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(AdcZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(RorZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Sei{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(AdcAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(AdcAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(RorAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_8_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x1 => Ok(Box::new(StaIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x4 => Ok(Box::new(StyZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x5 => Ok(Box::new(StaZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(StxZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x8 => Ok(Box::new(Dey{attr: InsAttr::new(opcode, 1, 2)})),
        0xa => Ok(Box::new(Txa{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(StyAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(StaAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(StxAbs{attr: InsAttr::new(opcode, 3, 4)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_9_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bcc{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(StaIndY{attr: InsAttr::new(opcode, 2, 6)})),
        0x4 => Ok(Box::new(StyZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x5 => Ok(Box::new(StaZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(StxZPY{attr: InsAttr::new(opcode, 2, 4)})),
        0x8 => Ok(Box::new(Tya{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(StaAbsY{attr: InsAttr::new(opcode, 3, 5)})),
        0xa => Ok(Box::new(Txs{attr: InsAttr::new(opcode, 1, 2)})),
        0xd => Ok(Box::new(StaAbsX{attr: InsAttr::new(opcode, 3, 5)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_a_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(LdyImm{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(LdaIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x2 => Ok(Box::new(LdxImm{attr: InsAttr::new(opcode, 2, 2)})),
        0x4 => Ok(Box::new(LdyZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x5 => Ok(Box::new(LdaZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(LdxZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x8 => Ok(Box::new(Tay{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(LdaImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(Tax{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(LdyAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(LdaAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(LdxAbs{attr: InsAttr::new(opcode, 3, 4)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_b_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bcs{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(LdaIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x4 => Ok(Box::new(LdyZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x5 => Ok(Box::new(LdaZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(LdxZPY{attr: InsAttr::new(opcode, 2, 4)})),
        0x8 => Ok(Box::new(Clv{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(LdaAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xa => Ok(Box::new(Tsx{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(LdyAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(LdaAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(LdxAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_c_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(CpyImm{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(CmpIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x4 => Ok(Box::new(CpyZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x5 => Ok(Box::new(CmpZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(DecZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Iny{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(CmpImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(Dex{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(CpyAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(CmpAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(DecAbs{attr: InsAttr::new(opcode, 3, 6)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_d_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Bne{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(CmpIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(CmpZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(DecZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Cld{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(CmpAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(CmpAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(DecAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_e_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(CpxImm{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(SbcIndX{attr: InsAttr::new(opcode, 2, 6)})),
        0x4 => Ok(Box::new(CpxZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x5 => Ok(Box::new(SbcZP{attr: InsAttr::new(opcode, 2, 3)})),
        0x6 => Ok(Box::new(IncZP{attr: InsAttr::new(opcode, 2, 5)})),
        0x8 => Ok(Box::new(Inx{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(SbcImm{attr: InsAttr::new(opcode, 2, 2)})),
        0xa => Ok(Box::new(Nop{attr: InsAttr::new(opcode, 1, 2)})),
        0xc => Ok(Box::new(CpxAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(SbcAbs{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(IncAbs{attr: InsAttr::new(opcode, 3, 6)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

fn parse_prefix_f_instruction(opcode: u8) -> Result<Box<dyn Mos6502Ins>, InvalidOpcodeError> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Ok(Box::new(Beq{attr: InsAttr::new(opcode, 2, 2)})),
        0x1 => Ok(Box::new(SbcIndY{attr: InsAttr::new(opcode, 2, 5)})),
        0x5 => Ok(Box::new(SbcZPX{attr: InsAttr::new(opcode, 2, 4)})),
        0x6 => Ok(Box::new(IncZPX{attr: InsAttr::new(opcode, 2, 6)})),
        0x8 => Ok(Box::new(Sed{attr: InsAttr::new(opcode, 1, 2)})),
        0x9 => Ok(Box::new(SbcAbsY{attr: InsAttr::new(opcode, 3, 4)})),
        0xd => Ok(Box::new(SbcAbsX{attr: InsAttr::new(opcode, 3, 4)})),
        0xe => Ok(Box::new(IncAbsX{attr: InsAttr::new(opcode, 3, 7)})),
        _ => Err(InvalidOpcodeError { opcode }),
    }
}

#[cfg(test)]
mod tests {
    use std::{any::{Any, TypeId}, fmt::Debug};

    use crate::mos6502::parser::Brk;

    use super::parse;

    #[test]
    fn parse_should_return_brk_given_opcode_0x00() {
        let ins_type = parse(0x00).unwrap().as_ref();
    }
}