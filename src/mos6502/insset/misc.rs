use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Brk {
    pub attr: InsAttr,
}

pub struct JmpAbs {
    pub attr: InsAttr,
}

pub struct JmpInd {
    pub attr: InsAttr,
}

pub struct Jsr {
    pub attr: InsAttr,
}

pub struct Nop {
    pub attr: InsAttr,
}

pub struct Rti {
    pub attr: InsAttr,
}

pub struct Rts {
    pub attr: InsAttr,
}

pub struct Sec {
    pub attr: InsAttr,
}

pub struct Sed {
    pub attr: InsAttr,
}

pub struct Sei {
    pub attr: InsAttr,
}

///
/// Illegal opcode
///
pub struct Ilg {
    pub attr: InsAttr,
}

impl Mos6502Ins for Brk {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for JmpAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        let address_lsb = cpu.mem[cpu.pc as usize + 1] as u16;
        let address_msb = cpu.mem[cpu.pc as usize + 2] as u16;
        let address: u16 = (address_msb << 8) | address_lsb;
        cpu.pc = address
    }
}

#[allow(arithmetic_overflow)]
impl Mos6502Ins for JmpInd {
    fn execute(&self, cpu: &mut Mos6502) {
        let address_lsb: u8 = cpu.mem[cpu.pc as usize + 1];
        let address_msb = cpu.mem[cpu.pc as usize + 2] as u16;

        let next_address_lsb: u8 = address_lsb + 1;

        let address: u16 = (address_msb << 8) | address_lsb as u16;
        let next_address: u16 = (address_msb << 8) | next_address_lsb as u16;

        let effect_address_lsb = cpu.mem[address as usize] as u16;
        let effect_address_msb = cpu.mem[next_address as usize] as u16;

        cpu.pc = (effect_address_msb << 8) | effect_address_lsb
    }
}

impl Mos6502Ins for Jsr {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Nop {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.next_instruction(&self.attr);
    }
}

impl Mos6502Ins for Rti {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Rts {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Sec {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Sed {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Sei {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Ilg {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.stop();
        print!("Illegal opcode {}", self.attr.opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn JmpInd_should_jump_indirect() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0xfe;
        cpu.mem[cpu.pc as usize + 2] = 0x02;
        cpu.mem[0x02fe] = 0xaa;
        cpu.mem[0x02ff] = 0xaa;
        let ins = JmpInd {
            attr: InsAttr {
                opcode: 0x0,
                len: 0x2,
                cyc: 0x1,
            },
        };
        ins.execute(&mut cpu);
        assert_eq!(0xaaaa, cpu.pc)
    }

    #[test]
    fn JmpInd_should_jump_indirect_and_not_turn_page() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0xff;
        cpu.mem[cpu.pc as usize + 2] = 0x02;
        cpu.mem[0x02ff] = 0xaa;
        cpu.mem[0x0200] = 0xaa;
        let ins = JmpInd {
            attr: InsAttr {
                opcode: 0x0,
                len: 0x2,
                cyc: 0x1,
            },
        };
        ins.execute(&mut cpu);
        assert_eq!(0xaaaa, cpu.pc)
    }
}
