use crate::mos6502::{
    address_mode::{
        absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zero_page,
        zero_page_x, AddressModeFn,
    },
    constant::{CARRY_ON_MASK, NEGATIVE_ON_MASK, ZERO_ON_MASK},
};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct CmpImm {
    pub attr: InsAttr,
}

pub struct CmpZP {
    pub attr: InsAttr,
}

pub struct CmpZPX {
    pub attr: InsAttr,
}

pub struct CmpAbs {
    pub attr: InsAttr,
}

pub struct CmpAbsX {
    pub attr: InsAttr,
}

pub struct CmpAbsY {
    pub attr: InsAttr,
}

pub struct CmpIndX {
    pub attr: InsAttr,
}

pub struct CmpIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for CmpImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for CmpZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for CmpZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, zero_page_x);
    }
}

impl Mos6502Ins for CmpAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for CmpAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, absolute_x);
    }
}

impl Mos6502Ins for CmpAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, absolute_y);
    }
}

impl Mos6502Ins for CmpIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, indirect_x);
    }
}

impl Mos6502Ins for CmpIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare(cpu, &self.attr, indirect_y);
    }
}

fn do_compare(cpu: &mut Mos6502, attr: &InsAttr, address_fn: AddressModeFn) {
    let operand: u8 = address_fn(cpu);
    if cpu.ac >= operand {
        cpu.sr |= CARRY_ON_MASK
    } else {
        cpu.sr |= NEGATIVE_ON_MASK
    }
    if cpu.ac == operand {
        cpu.sr |= ZERO_ON_MASK
    }
    cpu.next_instruction(attr);
}
