use super::{InsAttr, Mos6502Ins, Mos6502};

pub struct AdcImm {
    pub attr: InsAttr,
}
pub struct AdcZP {
    pub attr: InsAttr,
}
pub struct AdcZPX {
    pub attr: InsAttr,
}
pub struct AdcAbs {
    pub attr: InsAttr,
}
pub struct AdcAbsX {
    pub attr: InsAttr,
}
pub struct AdcAbsY {
    pub attr: InsAttr,
}
pub struct AdcIndX {
    pub attr: InsAttr,
}
pub struct AdcIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for AdcImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}


impl Mos6502Ins for AdcAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AdcIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
