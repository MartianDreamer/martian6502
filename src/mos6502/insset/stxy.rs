use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct StxZP {
    pub attr: InsAttr,
}

pub struct StxZPY {
    pub attr: InsAttr,
}

pub struct StxAbs {
    pub attr: InsAttr,
}

pub struct StyZP {
    pub attr: InsAttr,
}

pub struct StyZPX {
    pub attr: InsAttr,
}

pub struct StyAbs {
    pub attr: InsAttr,
}

impl Mos6502Ins for StxZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StxZPY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StxAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StyZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StyZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StyAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
