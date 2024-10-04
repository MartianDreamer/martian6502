use crate::mos6502::{
    address_mode::{immediate, relative},
    constant::BIT_0_MASK,
};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Bcc {
    pub attr: InsAttr,
}

pub struct Bcs {
    pub attr: InsAttr,
}

pub struct Beq {
    pub attr: InsAttr,
}

pub struct Bmi {
    pub attr: InsAttr,
}

pub struct Bne {
    pub attr: InsAttr,
}

pub struct Bpl {
    pub attr: InsAttr,
}

pub struct Bvc {
    pub attr: InsAttr,
}

pub struct Bvs {
    pub attr: InsAttr,
}

impl Mos6502Ins for Bcc {
    fn execute(&self, cpu: &mut Mos6502) {
        let carry_bit: u8 = cpu.sr & BIT_0_MASK;
        if carry_bit == 0b1 {
            let new_pc: u16 = relative(cpu);
            cpu.pc = new_pc
        }
    }
}

impl Mos6502Ins for Bcs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Beq {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Bmi {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Bne {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Bpl {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Bvc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Bvs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
