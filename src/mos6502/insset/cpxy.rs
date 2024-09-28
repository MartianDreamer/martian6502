use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct CpxImm {
    pub attr: InsAttr,
}

pub struct CpxZP {
    pub attr: InsAttr,
}

pub struct CpxAbs {
    pub attr: InsAttr,
}

pub struct CpyImm {
    pub attr: InsAttr,
}

pub struct CpyZP {
    pub attr: InsAttr,
}

pub struct CpyAbs {
    pub attr: InsAttr,
}

impl Mos6502Ins for CpxImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CpxZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CpxAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CpyImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CpyZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CpyAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
