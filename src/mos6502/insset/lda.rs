use crate::mos6502::address_mode::{ absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zero_page, zero_page_x, AddressModeFn};

use super::{
    utils::{update_negative_flag, update_zero_flag},
    InsAttr, Mos6502, Mos6502Ins,
};

pub struct LdaImm {
    pub attr: InsAttr,
}

pub struct LdaZP {
    pub attr: InsAttr,
}

pub struct LdaZPX {
    pub attr: InsAttr,
}

pub struct LdaAbs {
    pub attr: InsAttr,
}

pub struct LdaAbsX {
    pub attr: InsAttr,
}

pub struct LdaAbsY {
    pub attr: InsAttr,
}

pub struct LdaIndX {
    pub attr: InsAttr,
}

pub struct LdaIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for LdaImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for LdaZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for LdaZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, zero_page_x);
    }
}

impl Mos6502Ins for LdaAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for LdaAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, absolute_x);
    }
}

impl Mos6502Ins for LdaAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, absolute_y);
    }
}

impl Mos6502Ins for LdaIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, indirect_x);
    }
}

impl Mos6502Ins for LdaIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_accumulator(cpu, &self.attr, indirect_y);
    }
}

fn do_load_accumulator(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let mem_val: u8 = address_mode_fn(&cpu);
    cpu.ac = mem_val;

    update_zero_flag(cpu, cpu.ac == 0);
    update_negative_flag(cpu, (cpu.ac as i8) < 0);

    cpu.next_instruction(attr);
}
