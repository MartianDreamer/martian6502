use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Pha {
    pub attr: InsAttr,
}

pub struct Php {
    pub attr: InsAttr,
}

pub struct Pla {
    pub attr: InsAttr,
}

pub struct Plp {
    pub attr: InsAttr,
}

impl Mos6502Ins for Pha {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Php {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Pla {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Plp {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
