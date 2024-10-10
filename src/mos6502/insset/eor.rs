use crate::mos6502::address_mode::{absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zero_page, zero_page_x, AddressModeFn};

use super::{
    utils::{update_negative_flag, update_zero_flag},
    InsAttr, Mos6502, Mos6502Ins,
};

pub struct EorImm {
    pub attr: InsAttr,
}

pub struct EorZP {
    pub attr: InsAttr,
}

pub struct EorZPX {
    pub attr: InsAttr,
}

pub struct EorAbs {
    pub attr: InsAttr,
}

pub struct EorAbsX {
    pub attr: InsAttr,
}

pub struct EorAbsY {
    pub attr: InsAttr,
}

pub struct EorIndX {
    pub attr: InsAttr,
}

pub struct EorIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for EorImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for EorZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for EorZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, zero_page_x);
    }
}

impl Mos6502Ins for EorAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for EorAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, absolute_x);
    }
}

impl Mos6502Ins for EorAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, absolute_y);
    }
}

impl Mos6502Ins for EorIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, indirect_x);
    }
}

impl Mos6502Ins for EorIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_exclusive_or(cpu, &self.attr, indirect_y);
    }
}

fn do_exclusive_or(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let operand: u8 = address_mode_fn(&cpu);
    cpu.ac ^= operand;

    update_zero_flag(cpu, cpu.ac == 0);
    update_negative_flag(cpu, (cpu.ac as i8) < 0);

    cpu.next_instruction(attr);
}
