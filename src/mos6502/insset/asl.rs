use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct AslAcc {
    pub attr: InsAttr,
}
pub struct AslZP {
    pub attr: InsAttr,
}
pub struct AslZPX {
    pub attr: InsAttr,
}
pub struct AslAbs {
    pub attr: InsAttr,
}
pub struct AslAbsX {
    pub attr: InsAttr,
}

impl Mos6502Ins for AslAcc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AslZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AslZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AslAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for AslAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
