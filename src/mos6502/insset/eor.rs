use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct EorImm {
    pub attr: InsAttr,
}

pub struct EorZP {
    pub attr: InsAttr,
}

pub struct EorZPX {
    pub attr: InsAttr,
}

pub struct EorAbs {
    pub attr: InsAttr,
}

pub struct EorAbsX {
    pub attr: InsAttr,
}

pub struct EorAbsY {
    pub attr: InsAttr,
}

pub struct EorIndX {
    pub attr: InsAttr,
}

pub struct EorIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for EorImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for EorIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
