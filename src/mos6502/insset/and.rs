use super::{InsAttr, Mos6502Ins, Mos6502};

pub struct AndImm {
    pub attr: InsAttr,
}
pub struct AndZP {
    pub attr: InsAttr,
}
pub struct AndZPX {
    pub attr: InsAttr,
}
pub struct AndAbs {
    pub attr: InsAttr,
}
pub struct AndAbsX {
    pub attr: InsAttr,
}
pub struct AndAbsY {
    pub attr: InsAttr,
}
pub struct AndIndX {
    pub attr: InsAttr,
}
pub struct AndIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for AndImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
impl Mos6502Ins for AndIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
