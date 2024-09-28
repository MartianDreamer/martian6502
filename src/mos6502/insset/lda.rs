use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct LdaImm {
    pub attr: InsAttr,
}

pub struct LdaZP {
    pub attr: InsAttr,
}

pub struct LdaZPX {
    pub attr: InsAttr,
}

pub struct LdaAbs {
    pub attr: InsAttr,
}

pub struct LdaAbsX {
    pub attr: InsAttr,
}

pub struct LdaAbsY {
    pub attr: InsAttr,
}

pub struct LdaIndX {
    pub attr: InsAttr,
}

pub struct LdaIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for LdaImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdaIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
