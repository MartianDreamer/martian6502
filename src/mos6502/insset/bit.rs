use crate::mos6502::{
    address_mode::{absolute, zero_page, AddressModeFn},
    constant::{
        LAST_BIT_MASK, NEGATIVE_OFF_MASK, NEGATIVE_ON_MASK, OVERFLOW_OFF_MASK, OVERFLOW_ON_MASK,
        ZERO_OFF_MASK, ZERO_ON_MASK,
    },
};

use super::{InsAttr, Mos6502, Mos6502Ins};

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
    let operand_bit6: u8 = (operand >> 6) & LAST_BIT_MASK;
    let operand_bit7: u8 = operand >> 7;

    if operand & cpu.ac == 0 {
        cpu.sr |= ZERO_ON_MASK
    } else {
        cpu.sr &= ZERO_OFF_MASK
    }

    if operand_bit7 == 0b1 {
        cpu.sr |= NEGATIVE_ON_MASK
    } else {
        cpu.sr &= NEGATIVE_OFF_MASK
    }

    if operand_bit6 == 0b1 {
        cpu.sr |= OVERFLOW_ON_MASK
    } else {
        cpu.sr &= OVERFLOW_OFF_MASK
    }

    cpu.next_instruction(attr);
}
