use crate::mos6502::address_mode::{
    absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zero_page, zero_page_x,
    AddressModeFn,
};

use super::{
    utils::{update_carry_flag, update_negative_flag, update_overflow_flag, update_zero_flag},
    InsAttr, Mos6502, Mos6502Ins,
};

pub struct AdcImm {
    pub attr: InsAttr,
}
pub struct AdcZP {
    pub attr: InsAttr,
}
pub struct AdcZPX {
    pub attr: InsAttr,
}
pub struct AdcAbs {
    pub attr: InsAttr,
}
pub struct AdcAbsX {
    pub attr: InsAttr,
}
pub struct AdcAbsY {
    pub attr: InsAttr,
}
pub struct AdcIndX {
    pub attr: InsAttr,
}
pub struct AdcIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for AdcImm {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, immediate);
    }
}

impl Mos6502Ins for AdcZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for AdcZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, zero_page_x);
    }
}

impl Mos6502Ins for AdcAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, absolute);
    }
}

impl Mos6502Ins for AdcAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, absolute_x);
    }
}

impl Mos6502Ins for AdcAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, absolute_y);
    }
}

impl Mos6502Ins for AdcIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, indirect_x);
    }
}

impl Mos6502Ins for AdcIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        do_add(cpu, &self.attr, indirect_y);
    }
}

fn do_add(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let operand: u8 = address_mode_fn(cpu);
    add_and_update_status_register(cpu, operand);
    cpu.next_instruction(attr);
}

#[allow(arithmetic_overflow)]
fn add_and_update_status_register(cpu: &mut Mos6502, operand: u8) {
    let carry: u8 = cpu.is_carried();
    let result: u8 = cpu.ac + operand + carry;
    let acc_bit7: u8 = cpu.ac >> 7;
    let operand_bit7: u8 = operand >> 7;
    let result_bit7: u8 = result >> 7;

    update_overflow_flag(cpu, acc_bit7 == operand_bit7 && acc_bit7 != result_bit7);
    update_carry_flag(cpu, u8::MAX - cpu.ac - carry < operand);
    update_zero_flag(cpu, result == 0);
    update_negative_flag(cpu, result_bit7 == 0b1);

    cpu.ac = result
}
