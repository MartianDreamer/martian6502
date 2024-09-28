use super::{InsAttr, Mos6502, Mos6502Ins};

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
        todo!()
    }
}

impl Mos6502Ins for DecZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for DecAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for DecAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Dex {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Dey {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
