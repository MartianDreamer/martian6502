use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct StaImm {
    pub attr: InsAttr,
}

pub struct StaZP {
    pub attr: InsAttr,
}

pub struct StaZPX {
    pub attr: InsAttr,
}

pub struct StaAbs {
    pub attr: InsAttr,
}

pub struct StaAbsX {
    pub attr: InsAttr,
}

pub struct StaAbsY {
    pub attr: InsAttr,
}

pub struct StaIndX {
    pub attr: InsAttr,
}

pub struct StaIndY {
    pub attr: InsAttr,
}

impl Mos6502Ins for StaImm {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaAbsY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaIndX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for StaIndY {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
