use crate::mos6502::{
    constant::{CARRY_ON_MASK, NEGATIVE_ON_MASK, OVERFLOW_ON_MASK, ZERO_OFF_MASK, ZERO_ON_MASK},
    Mos6502,
};

pub fn update_zero_flag(cpu: &mut Mos6502, is_updated: bool) {
    update_flag(cpu, ZERO_ON_MASK, is_updated);
}

pub fn update_overflow_flag(cpu: &mut Mos6502, is_updated: bool) {
    update_flag(cpu, OVERFLOW_ON_MASK, is_updated);
}

pub fn update_carry_flag(cpu: &mut Mos6502, is_updated: bool) {
    update_flag(cpu, CARRY_ON_MASK, is_updated);
}

pub fn update_negative_flag(cpu: &mut Mos6502, is_updated: bool) {
    update_flag(cpu, NEGATIVE_ON_MASK, is_updated);
}

fn update_flag(cpu: &mut Mos6502, mask: u8, is_updated: bool) {
    if is_updated {
        cpu.sr |= mask
    } else {
        cpu.sr &= !mask
    }
}
