use crate::mos6502::address_mode::{ absolute, absolute_x, absolute_y, immediate, zero_page, zero_page_x, zero_page_y, AddressModeFn};

use super::{utils::{update_negative_flag, update_zero_flag}, InsAttr, Mos6502, Mos6502Ins};

pub struct LdxImm {
    pub attr: InsAttr,
}

pub struct LdxZP {
    pub attr: InsAttr,
}

pub struct LdxZPY {
    pub attr: InsAttr,
}

pub struct LdxAbs {
    pub attr: InsAttr,
}

pub struct LdxAbsY {
    pub attr: InsAttr,
}

pub struct LdyImm {
    pub attr: InsAttr,
}

pub struct LdyZP {
    pub attr: InsAttr,
}

pub struct LdyZPX {
    pub attr: InsAttr,
}

pub struct LdyAbs {
    pub attr: InsAttr,
}

pub struct LdyAbsX {
    pub attr: InsAttr,
}

impl Mos6502Ins for LdxImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_x(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for LdxZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_x(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for LdxZPY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_x(cpu, &self.attr, zero_page_y);
    }
}

impl Mos6502Ins for LdxAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_x(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for LdxAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_x(cpu, &self.attr, absolute_y);
    }
}

impl Mos6502Ins for LdyImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_y(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for LdyZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_y(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for LdyZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_y(cpu, &self.attr, zero_page_x);
    }
}

impl Mos6502Ins for LdyAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_y(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for LdyAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_load_y(cpu, &self.attr, absolute_x);
    }
}

fn do_load_x(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let mem_val: u8 = address_mode_fn(&cpu);
    cpu.xr = mem_val;

    update_zero_flag(cpu, cpu.xr == 0);
    update_negative_flag(cpu, (cpu.xr as i8) < 0);

    cpu.next_instruction(attr);
}

fn do_load_y(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let mem_val: u8 = address_mode_fn(&cpu);
    cpu.yr = mem_val;

    update_zero_flag(cpu, cpu.yr == 0);
    update_negative_flag(cpu, (cpu.yr as i8) < 0);

    cpu.next_instruction(attr);
}
