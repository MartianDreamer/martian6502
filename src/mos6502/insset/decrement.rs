use crate::mos6502::address_mode::{
    absolute_immutable, absolute_x_immutable, zero_page_immutatble, zero_page_x_immutable,
    AddressModeImmutableFn,
};

use super::{
    utils::{update_negative_flag, update_zero_flag},
    InsAttr, Mos6502, Mos6502Ins,
};

pub struct DecZP {
    pub attr: InsAttr,
}

pub struct DecZPX {
    pub attr: InsAttr,
}

pub struct DecAbs {
    pub attr: InsAttr,
}

pub struct DecAbsX {
    pub attr: InsAttr,
}

pub struct Dex {
    pub attr: InsAttr,
}

pub struct Dey {
    pub attr: InsAttr,
}

impl Mos6502Ins for DecZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_decrement_at_mem(cpu, &self.attr, zero_page_immutatble);
    }
}

impl Mos6502Ins for DecZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_decrement_at_mem(cpu, &self.attr, zero_page_x_immutable);
    }
}

impl Mos6502Ins for DecAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_decrement_at_mem(cpu, &self.attr, absolute_immutable);
    }
}

impl Mos6502Ins for DecAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_decrement_at_mem(cpu, &self.attr, absolute_x_immutable);
    }
}

impl Mos6502Ins for Dex {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.xr -= 1;

        update_zero_flag(cpu, cpu.xr == 0);
        update_negative_flag(cpu, (cpu.xr as i8) < 0);

        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Dey {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.yr -= 1;

        update_zero_flag(cpu, cpu.yr == 0);
        update_negative_flag(cpu, (cpu.yr as i8) < 0);

        cpu.next_instruction(&self.attr);
    }
}

fn do_decrement_at_mem<'a>(
    cpu: &'a mut Mos6502,
    attr: &InsAttr,
    address_mode_fn: AddressModeImmutableFn,
) {
    let operand: &mut u8 = address_mode_fn(cpu);
    *operand -= 1;
    let result: u8 = *operand;

    update_zero_flag(cpu, result == 0);
    update_negative_flag(cpu, (result as i8) < 0);

    cpu.next_instruction(attr);
}
