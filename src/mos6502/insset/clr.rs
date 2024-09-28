use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Clc {
    pub attr: InsAttr,
}

pub struct Cld {
    pub attr: InsAttr,
}

pub struct Cli {
    pub attr: InsAttr,
}

pub struct Clv {
    pub attr: InsAttr,
}

impl Mos6502Ins for Clc {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Cld {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Cli {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Clv {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
