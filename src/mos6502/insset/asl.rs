use crate::mos6502::{
    address_mode::{
        absolute_immutable, absolute_x_immutable, zero_page_immutatble, zero_page_x_immutable,
        AddressModeImmutableFn,
    },
    constant::{
        CARRY_OFF_MASK, CARRY_ON_MASK, NEGATIVE_OFF_MASK, NEGATIVE_ON_MASK, ZERO_OFF_MASK,
        ZERO_ON_MASK,
    },
};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct AslAcc {
    pub attr: InsAttr,
}
pub struct AslZP {
    pub attr: InsAttr,
}
pub struct AslZPX {
    pub attr: InsAttr,
}
pub struct AslAbs {
    pub attr: InsAttr,
}
pub struct AslAbsX {
    pub attr: InsAttr,
}

impl Mos6502Ins for AslAcc {
    fn execute(&self, cpu: &mut Mos6502) {
        let result: u8 = cpu.ac << 1;
        let old_val_bit_7 = cpu.ac >> 7;

        if old_val_bit_7 == 0b1 {
            cpu.sr |= CARRY_ON_MASK
        } else {
            cpu.sr &= CARRY_OFF_MASK
        }

        if result == 0 {
            cpu.sr |= ZERO_ON_MASK
        } else {
            cpu.sr &= ZERO_OFF_MASK
        }

        if result << 7 == 0b1 {
            cpu.sr |= NEGATIVE_ON_MASK
        } else {
            cpu.sr &= NEGATIVE_OFF_MASK
        }

        cpu.ac = result;
        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for AslZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_asl(cpu, &self.attr, zero_page_immutatble);
    }
}

impl Mos6502Ins for AslZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_asl(cpu, &self.attr, zero_page_x_immutable);
    }
}

impl Mos6502Ins for AslAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_asl(cpu, &self.attr, absolute_immutable);
    }
}

impl Mos6502Ins for AslAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_asl(cpu, &self.attr, absolute_x_immutable);
    }
}

fn do_asl(cpu: &mut Mos6502, attr: &InsAttr, immutable_fn: AddressModeImmutableFn) {
    let address: &mut u8 = immutable_fn(cpu);
    let old_val_bit_7: u8 = *address >> 7;
    let result: u8 = *address << 1;
    *address = result;

    if old_val_bit_7 == 0b1 {
        cpu.sr |= CARRY_ON_MASK
    } else {
        cpu.sr &= CARRY_OFF_MASK
    }

    if result == 0 {
        cpu.sr |= ZERO_ON_MASK
    } else {
        cpu.sr &= ZERO_OFF_MASK
    }

    if result << 7 == 0b1 {
        cpu.sr |= NEGATIVE_ON_MASK
    } else {
        cpu.sr &= NEGATIVE_OFF_MASK
    }

    cpu.next_instruction(attr);
}
