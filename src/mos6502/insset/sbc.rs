use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct SbcImm {
    pub attr: InsAttr,
}

pub struct SbcZP {
    pub attr: InsAttr,
}

pub struct SbcZPX {
    pub attr: InsAttr,
}

pub struct SbcAbs {
    pub attr: InsAttr,
}

pub struct SbcAbsX {
    pub attr: InsAttr,
}

pub struct SbcAbsY {
    pub attr: InsAttr,
}

pub struct SbcIndX {
    pub attr: InsAttr,
}

pub struct SbcIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for SbcImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for SbcIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
