use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct RolAcc {
    pub attr: InsAttr,
}

pub struct RolZP {
    pub attr: InsAttr,
}

pub struct RolZPX {
    pub attr: InsAttr,
}

pub struct RolAbs {
    pub attr: InsAttr,
}

pub struct RolAbsX {
    pub attr: InsAttr,
}

pub struct RorAcc {
    pub attr: InsAttr,
}

pub struct RorZP {
    pub attr: InsAttr,
}

pub struct RorZPX {
    pub attr: InsAttr,
}

pub struct RorAbs {
    pub attr: InsAttr,
}

pub struct RorAbsX {
    pub attr: InsAttr,
}

impl Mos6502Ins for RolAcc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RolZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RolZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RolAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RolAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RorAcc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RorZP {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RorZPX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RorAbs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for RorAbsX {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
