use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct CmpImm {
    pub attr: InsAttr,
}

pub struct CmpZP {
    pub attr: InsAttr,
}

pub struct CmpZPX {
    pub attr: InsAttr,
}

pub struct CmpAbs {
    pub attr: InsAttr,
}

pub struct CmpAbsX {
    pub attr: InsAttr,
}

pub struct CmpAbsY {
    pub attr: InsAttr,
}

pub struct CmpIndX {
    pub attr: InsAttr,
}

pub struct CmpIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for CmpImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for CmpIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
