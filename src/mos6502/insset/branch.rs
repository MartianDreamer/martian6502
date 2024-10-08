use crate::mos6502::{address_mode::relative, constant::BIT_0_MASK};

use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Bcc {
    pub attr: InsAttr,
}

pub struct Bcs {
    pub attr: InsAttr,
}

pub struct Beq {
    pub attr: InsAttr,
}

pub struct Bmi {
    pub attr: InsAttr,
}

pub struct Bne {
    pub attr: InsAttr,
}

pub struct Bpl {
    pub attr: InsAttr,
}

pub struct Bvc {
    pub attr: InsAttr,
}

pub struct Bvs {
    pub attr: InsAttr,
}

impl Mos6502Ins for Bcc {
    fn execute(&self, cpu: &mut Mos6502) {
        let carry_flag: u8 = cpu.sr & BIT_0_MASK;
        if carry_flag == 0b0 {
            move_to_offset(cpu, &self.attr);
        }
    }
}

impl Mos6502Ins for Bcs {
    fn execute(&self, cpu: &mut Mos6502) {
        let carry_flag: u8 = cpu.sr & BIT_0_MASK;
        if carry_flag == 0b1 {
            move_to_offset(cpu, &self.attr);
        }
    }
}

impl Mos6502Ins for Beq {
    fn execute(&self, cpu: &mut Mos6502) {
        let zero_flag: u8 = (cpu.sr >> 1) & BIT_0_MASK;
        if zero_flag == 0b1 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

impl Mos6502Ins for Bmi {
    fn execute(&self, cpu: &mut Mos6502) {
        let negative_flag: u8 = (cpu.sr >> 7) & BIT_0_MASK;
        if negative_flag == 0b1 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

impl Mos6502Ins for Bne {
    fn execute(&self, cpu: &mut Mos6502) {
        let zero_flag: u8 = (cpu.sr >> 1) & BIT_0_MASK;
        if zero_flag == 0b0 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

impl Mos6502Ins for Bpl {
    fn execute(&self, cpu: &mut Mos6502) {
        let negative_flag: u8 = (cpu.sr >> 7) & BIT_0_MASK;
        if negative_flag == 0b0 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

impl Mos6502Ins for Bvc {
    fn execute(&self, cpu: &mut Mos6502) {
        let overflow_flag: u8 = (cpu.sr >> 6) & BIT_0_MASK;
        if overflow_flag == 0b0 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

impl Mos6502Ins for Bvs {
    fn execute(&self, cpu: &mut Mos6502) {
        let overflow_flag: u8 = (cpu.sr >> 6) & BIT_0_MASK;
        if overflow_flag == 0b1 {
            move_to_offset(cpu, &self.attr)
        }
    }
}

fn move_to_offset(cpu: &mut Mos6502, attr: &InsAttr) {
    let offset: u16 = relative(cpu);
    cpu.pc += offset;
    cpu.next_instruction(attr);
}
