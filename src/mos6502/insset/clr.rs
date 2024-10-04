use crate::mos6502::constant::{
    CARRY_OFF_MASK, DECIMAL_OFF_MASK, INTERRUPT_OFF_MASK, OVERFLOW_OFF_MASK,
};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Clc {
    pub attr: InsAttr,
}

pub struct Cld {
    pub attr: InsAttr,
}

pub struct Cli {
    pub attr: InsAttr,
}

pub struct Clv {
    pub attr: InsAttr,
}

impl Mos6502Ins for Clc {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.sr &= CARRY_OFF_MASK;
        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Cld {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.sr &= DECIMAL_OFF_MASK;
        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Cli {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.sr &= INTERRUPT_OFF_MASK;
        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Clv {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.sr &= OVERFLOW_OFF_MASK;
        cpu.next_instruction(&self.attr);
    }
}
