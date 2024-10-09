mod adc;
mod and;
mod asl;
mod bit;
mod branch;
mod clr;
mod cmp;
mod cpxy;
mod decrement;
mod eor;
mod increment;
mod lda;
mod ldxy;
mod lsr;
mod misc;
mod ora;
mod phpl;
mod rorl;
mod sbc;
mod sta;
mod stxy;
mod transfer;
mod utils;
use super::Mos6502;

pub mod parser;

pub trait Mos6502Ins {
    fn execute(&self, cpu: &mut Mos6502);
}

#[derive(Debug)]
pub struct InsAttr {
    opcode: u8, // opcode of this instruction
    len: u8,    // length of this instruction
    cyc: u8,    // number of cpu cycle to complete this instruction
}

impl InsAttr {
    pub fn new(opcode: u8, len: u8, cyc: u8) -> Self {
        Self { opcode, len, cyc }
    }

    pub fn opcode(self: &Self) -> u8 {
        self.opcode
    }

    pub fn len(self: &Self) -> u8 {
        self.len
    }

    pub fn cyc(self: &Self) -> u8 {
        self.cyc
    }
}
