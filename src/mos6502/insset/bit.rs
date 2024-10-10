use crate::mos6502::{
    address_mode::{absolute, zero_page, AddressModeFn},
    constant::BIT_0_MASK,
};

use super::{
    utils::{update_negative_flag, update_overflow_flag, update_zero_flag},
    InsAttr, Mos6502, Mos6502Ins,
};

pub struct BitZP {
    pub attr: InsAttr,
}

pub struct BitAbs {
    pub attr: InsAttr,
}

impl Mos6502Ins for BitZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_bit(cpu, &self.attr, zero_page);
    }
}

impl Mos6502Ins for BitAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_bit(cpu, &self.attr, absolute);
    }
}

fn do_bit(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeFn) {
    let operand: u8 = address_mode_fn(cpu);
    let operand_bit6: u8 = (operand >> 6) & BIT_0_MASK;

    update_zero_flag(cpu, operand & cpu.ac == 0);
    update_negative_flag(cpu, (operand as i8) < 0);
    update_overflow_flag(cpu, operand_bit6 == 0b1);

    cpu.next_instruction(attr);
}
