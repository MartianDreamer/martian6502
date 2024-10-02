pub mod adc;
pub mod and;
pub mod asl;
pub mod branch;
pub mod bit;
pub mod misc;
pub mod clr;
pub mod cmp;
pub mod cpxy;
pub mod decrement;
pub mod eor;
pub mod increment;
pub mod lda;
pub mod ldxy;
pub mod lsr;
pub mod ora;
pub mod phpl;
pub mod rorl;
pub mod sbc;
pub mod sta;
pub mod stxy;
pub mod transfer;
use super::Mos6502;

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