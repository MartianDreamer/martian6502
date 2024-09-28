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

impl Mos6502Ins for Brk {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for JmpAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for JmpInd {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Jsr {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Nop {
    fn execute(&self, cpu: &mut Mos6502) {
        cpu.pc += self.attr.len() as u16
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
