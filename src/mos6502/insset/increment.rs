use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct IncZP {
    pub attr: InsAttr,
}
pub struct IncZPX {
    pub attr: InsAttr,
}
pub struct IncAbs {
    pub attr: InsAttr,
}
pub struct IncAbsX {
    pub attr: InsAttr,
}
pub struct Inx {
    pub attr: InsAttr,
}
pub struct Iny {
    pub attr: InsAttr,
}

impl Mos6502Ins for IncZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for IncZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for IncAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for IncAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Inx {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Iny {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
