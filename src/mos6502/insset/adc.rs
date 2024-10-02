use crate::{
    constant::{CARRY_ON_MASK, NEGATIVE_ON_MASK, OVERFLOW_ON_MASK, ZERO_ON_MASK},
    mos6502::address_mode::{immediate, zero_page},
};

use super::{InsAttr, Mos6502, Mos6502Ins};

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
        let operand: u8 = immediate(cpu);
        add_and_update_status_register(cpu, operand);
    }
}

impl Mos6502Ins for AdcZP {
    fn execute(&self, cpu: &mut Mos6502) {
        let operand: u8 = zero_page(cpu);
        add_and_update_status_register(cpu, operand);
    }
}

impl Mos6502Ins for AdcZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

#[allow(arithmetic_overflow)]
fn add_and_update_status_register(cpu: &mut Mos6502, operand: u8) {
    let result: u8 = cpu.acc + operand;
    let acc_bit7: u8 = cpu.acc >> 7;
    let operand_bit7: u8 = operand >> 7;
    let result_bit7: u8 = result >> 7;

    if acc_bit7 == operand_bit7 && acc_bit7 != result_bit7 {
        cpu.sr |= OVERFLOW_ON_MASK;
        cpu.sr |= CARRY_ON_MASK;
    }
    if result == 0 {
        cpu.sr |= ZERO_ON_MASK;
    } else if result_bit7 == 0b1 {
        cpu.sr |= NEGATIVE_ON_MASK;
    }

    cpu.acc = result
}
