use super::insset::{InsAttr, Mos6502Ins};
use crate::constant::LOWER_NIBBLE_MASK;
use std::{error::Error, fmt::Display};

use super::insset::adc::*;
use super::insset::and::*;
use super::insset::asl::*;
use super::insset::bit::*;
use super::insset::branch::*;
use super::insset::clr::*;
use super::insset::cmp::*;
use super::insset::cpxy::*;
use super::insset::decrement::*;
use super::insset::eor::*;
use super::insset::increment::*;
use super::insset::lda::*;
use super::insset::ldxy::*;
use super::insset::lsr::*;
use super::insset::misc::*;
use super::insset::ora::*;
use super::insset::phpl::*;
use super::insset::rorl::*;
use super::insset::sbc::*;
use super::insset::sta::*;
use super::insset::stxy::*;
use super::insset::transfer::*;

///
/// parse an opcode to create a M6502Ins
///
pub fn parse(opcode: u8) -> Box<dyn Mos6502Ins> {
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
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_0_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Brk {
            attr: InsAttr::new(opcode, 1, 7),
        }),
        0x1 => Box::new(OraIndX {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(OraZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(AslZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Php {
            attr: InsAttr::new(opcode, 1, 3),
        }),
        0x9 => Box::new(OraImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(AslAcc {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xd => Box::new(OraAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(AslAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_1_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bpl {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(OraIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(OraZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(AslZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Clc {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(OraAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(OraAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(AslAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_2_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Jsr {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        0x1 => Box::new(AndIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x4 => Box::new(BitZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x5 => Box::new(AndZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(RolZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Plp {
            attr: InsAttr::new(opcode, 1, 4),
        }),
        0x9 => Box::new(AndImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(RolAcc {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(BitAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(AndAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(RolAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_3_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bmi {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(AndIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(AndZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(RolZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Sec {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(AndAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(AndAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(RolAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_4_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Rti {
            attr: InsAttr::new(opcode, 1, 6),
        }),
        0x1 => Box::new(EorImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x5 => Box::new(EorZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(LsrZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Pha {
            attr: InsAttr::new(opcode, 1, 3),
        }),
        0x9 => Box::new(EorImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(LsrAcc {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(JmpAbs {
            attr: InsAttr::new(opcode, 3, 3),
        }),
        0xd => Box::new(EorAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(LsrAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_5_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bvc {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(EorIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(EorZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(LsrZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Cli {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(EorAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(EorAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(LsrAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_6_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Rts {
            attr: InsAttr::new(opcode, 1, 6),
        }),
        0x1 => Box::new(AdcIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x5 => Box::new(AdcZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(RorZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Pla {
            attr: InsAttr::new(opcode, 1, 4),
        }),
        0x9 => Box::new(AdcImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(RorAcc {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(JmpInd {
            attr: InsAttr::new(opcode, 3, 5),
        }),
        0xd => Box::new(AdcAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(RorAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        // TODO add more instructions
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_7_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bvs {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(AdcIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(AdcZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(RorZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Sei {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(AdcAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(AdcAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(RorAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_8_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x1 => Box::new(StaIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x4 => Box::new(StyZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x5 => Box::new(StaZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(StxZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x8 => Box::new(Dey {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xa => Box::new(Txa {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(StyAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(StaAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(StxAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_9_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bcc {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(StaIndY {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x4 => Box::new(StyZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x5 => Box::new(StaZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(StxZPY {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x8 => Box::new(Tya {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(StaAbsY {
            attr: InsAttr::new(opcode, 3, 5),
        }),
        0xa => Box::new(Txs {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xd => Box::new(StaAbsX {
            attr: InsAttr::new(opcode, 3, 5),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_a_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(LdyImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(LdaIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x2 => Box::new(LdxImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x4 => Box::new(LdyZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x5 => Box::new(LdaZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(LdxZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x8 => Box::new(Tay {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(LdaImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(Tax {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(LdyAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(LdaAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(LdxAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_b_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bcs {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(LdaIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x4 => Box::new(LdyZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x5 => Box::new(LdaZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(LdxZPY {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x8 => Box::new(Clv {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(LdaAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xa => Box::new(Tsx {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(LdyAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(LdaAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(LdxAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_c_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(CpyImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(CmpIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x4 => Box::new(CpyZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x5 => Box::new(CmpZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(DecZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Iny {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(CmpImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(Dex {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(CpyAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(CmpAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(DecAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_d_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Bne {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(CmpIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(CmpZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(DecZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Cld {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(CmpAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(CmpAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(DecAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_e_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(CpxImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(SbcIndX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x4 => Box::new(CpxZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x5 => Box::new(SbcZP {
            attr: InsAttr::new(opcode, 2, 3),
        }),
        0x6 => Box::new(IncZP {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x8 => Box::new(Inx {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(SbcImm {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0xa => Box::new(Nop {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0xc => Box::new(CpxAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(SbcAbs {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(IncAbs {
            attr: InsAttr::new(opcode, 3, 6),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}

fn parse_prefix_f_instruction(opcode: u8) -> Box<dyn Mos6502Ins> {
    let lower_nibble: u8 = opcode & LOWER_NIBBLE_MASK;
    match lower_nibble {
        0x0 => Box::new(Beq {
            attr: InsAttr::new(opcode, 2, 2),
        }),
        0x1 => Box::new(SbcIndY {
            attr: InsAttr::new(opcode, 2, 5),
        }),
        0x5 => Box::new(SbcZPX {
            attr: InsAttr::new(opcode, 2, 4),
        }),
        0x6 => Box::new(IncZPX {
            attr: InsAttr::new(opcode, 2, 6),
        }),
        0x8 => Box::new(Sed {
            attr: InsAttr::new(opcode, 1, 2),
        }),
        0x9 => Box::new(SbcAbsY {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xd => Box::new(SbcAbsX {
            attr: InsAttr::new(opcode, 3, 4),
        }),
        0xe => Box::new(IncAbsX {
            attr: InsAttr::new(opcode, 3, 7),
        }),
        _ => Box::new(Unr {
            attr: InsAttr::new(opcode, 1, 2),
        }),
    }
}
