use crate::mos6502::address_mode::{
    absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zero_page, zero_page_x,
    AddressModeFn,
};

use super::{utils::{update_negative_flag, update_zero_flag}, InsAttr, Mos6502, Mos6502Ins};

pub struct AndImm {
    pub attr: InsAttr,
}
pub struct AndZP {
    pub attr: InsAttr,
}
pub struct AndZPX {
    pub attr: InsAttr,
}
pub struct AndAbs {
    pub attr: InsAttr,
}
pub struct AndAbsX {
    pub attr: InsAttr,
}
pub struct AndAbsY {
    pub attr: InsAttr,
}
pub struct AndIndX {
    pub attr: InsAttr,
}
pub struct AndIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for AndImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, immediate);
    }
}
impl Mos6502Ins for AndZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, zero_page);
    }
}
impl Mos6502Ins for AndZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, zero_page_x);
    }
}
impl Mos6502Ins for AndAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, absolute);
    }
}
impl Mos6502Ins for AndAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, absolute_x);
    }
}
impl Mos6502Ins for AndAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, absolute_y);
    }
}
impl Mos6502Ins for AndIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, indirect_x);
    }
}
impl Mos6502Ins for AndIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_and(cpu, &self.attr, indirect_y);
    }
}

fn do_and(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let operand: u8 = address_mode_fn(cpu);
    let result: u8 = cpu.ac & operand;

    update_negative_flag(cpu, (result as i8) < 0);
    update_zero_flag(cpu, result == 0);

    cpu.ac = result;
    cpu.next_instruction(attr);
}
