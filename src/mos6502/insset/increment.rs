use crate::mos6502::address_mode::{absolute_immutable, absolute_x_immutable, zero_page_immutatble, zero_page_x_immutable, AddressModeImmutableFn};

use super::{utils::{update_negative_flag, update_zero_flag}, InsAttr, Mos6502, Mos6502Ins};

pub struct IncZP {
    pub attr: InsAttr,
}
pub struct IncZPX {
    pub attr: InsAttr,
}
pub struct IncAbs {
    pub attr: InsAttr,
}
pub struct IncAbsX {
    pub attr: InsAttr,
}
pub struct Inx {
    pub attr: InsAttr,
}
pub struct Iny {
    pub attr: InsAttr,
}

impl Mos6502Ins for IncZP {
    fn execute(&self, cpu: &mut Mos6502) {
        do_increment_at_mem(cpu, &self.attr, zero_page_immutatble);
    }
}

impl Mos6502Ins for IncZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_increment_at_mem(cpu, &self.attr, zero_page_x_immutable);
    }
}

impl Mos6502Ins for IncAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        do_increment_at_mem(cpu, &self.attr, absolute_immutable);
    }
}

impl Mos6502Ins for IncAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        do_increment_at_mem(cpu, &self.attr, absolute_x_immutable);
    }
}

impl Mos6502Ins for Inx {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.xr += 1;

        update_zero_flag(cpu, cpu.xr == 0);
        update_negative_flag(cpu, (cpu.xr as i8) < 0);

        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Iny {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.yr += 1;

        update_zero_flag(cpu, cpu.yr == 0);
        update_negative_flag(cpu, (cpu.yr as i8) < 0);

        cpu.next_instruction(&self.attr);
    }
}

fn do_increment_at_mem(cpu: &mut Mos6502, attr: &InsAttr, address_mode_fn: AddressModeImmutableFn) {
    let mem: &mut u8 = address_mode_fn(cpu);
    *mem += 1;
    let rs: u8 = *mem;

    update_zero_flag(cpu, rs == 0);
    update_negative_flag(cpu, (rs as i8) < 0);

    cpu.next_instruction(attr);
}