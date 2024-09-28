use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct LdxImm{pub attr: InsAttr}

pub struct LdxZP{pub attr: InsAttr}

pub struct LdxZPY{pub attr: InsAttr}

pub struct LdxAbs{pub attr: InsAttr}

pub struct LdxAbsY{pub attr: InsAttr}

pub struct LdyImm{pub attr: InsAttr}

pub struct LdyZP{pub attr: InsAttr}

pub struct LdyZPX{pub attr: InsAttr}

pub struct LdyAbs{pub attr: InsAttr}

pub struct LdyAbsX{pub attr: InsAttr}

impl Mos6502Ins for LdxImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdxZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdxZPY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdxAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdxAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdyImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdyZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdyZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdyAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for LdyAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
