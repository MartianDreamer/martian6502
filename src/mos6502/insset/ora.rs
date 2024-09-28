use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct OraImm {
    pub attr: InsAttr,
}

pub struct OraZP {
    pub attr: InsAttr,
}

pub struct OraZPX {
    pub attr: InsAttr,
}

pub struct OraAbs {
    pub attr: InsAttr,
}

pub struct OraAbsX {
    pub attr: InsAttr,
}

pub struct OraAbsY {
    pub attr: InsAttr,
}

pub struct OraIndX {
    pub attr: InsAttr,
}

pub struct OraIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for OraImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for OraIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
