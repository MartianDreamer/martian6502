use crate::mos6502::{
    address_mode::{absolute, immediate, zero_page, AddressModeFn},
    constant::{CARRY_ON_MASK, NEGATIVE_ON_MASK, ZERO_ON_MASK},
};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct CpxImm {
    pub attr: InsAttr,
}

pub struct CpxZP {
    pub attr: InsAttr,
}

pub struct CpxAbs {
    pub attr: InsAttr,
}

pub struct CpyImm {
    pub attr: InsAttr,
}

pub struct CpyZP {
    pub attr: InsAttr,
}

pub struct CpyAbs {
    pub attr: InsAttr,
}

impl Mos6502Ins for CpxImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_x(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for CpxZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_x(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for CpxAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_x(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for CpyImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_y(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for CpyZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_y(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for CpyAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_compare_y(cpu, &self.attr, absolute);
    }
}

fn do_compare_x(cpu: &mut Mos6502, attr: &InsAttr, address_fn: AddressModeFn) {
    let operand: u8 = address_fn(cpu);
    if cpu.xr >= operand {
        cpu.sr |= CARRY_ON_MASK
    } else {
        cpu.sr |= NEGATIVE_ON_MASK
    }
    if cpu.xr == operand {
        cpu.sr |= ZERO_ON_MASK
    }
    cpu.next_instruction(attr);
}

fn do_compare_y(cpu: &mut Mos6502, attr: &InsAttr, address_fn: AddressModeFn) {
    let operand: u8 = address_fn(cpu);
    if cpu.yr >= operand {
        cpu.sr |= CARRY_ON_MASK
    } else {
        cpu.sr |= NEGATIVE_ON_MASK
    }
    if cpu.yr == operand {
        cpu.sr |= ZERO_ON_MASK
    }
    cpu.next_instruction(attr);
}
