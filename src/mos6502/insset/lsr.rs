use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct LsrAcc {
    pub attr: InsAttr,
}

pub struct LsrZP {
    pub attr: InsAttr,
}

pub struct LsrZPX {
    pub attr: InsAttr,
}

pub struct LsrAbs {
    pub attr: InsAttr,
}

pub struct LsrAbsX {
    pub attr: InsAttr,
}

impl Mos6502Ins for LsrAcc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LsrZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LsrZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LsrAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LsrAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
