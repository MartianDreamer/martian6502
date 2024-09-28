use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct BitZP {
    pub attr: InsAttr,
}

pub struct BitAbs {
    pub attr: InsAttr,
}

impl Mos6502Ins for BitZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for BitAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
