use super::{InsAttr, Mos6502, Mos6502Ins};

pub struct Tax {
    pub attr: InsAttr,
}

pub struct Tay {
    pub attr: InsAttr,
}

pub struct Tsx {
    pub attr: InsAttr,
}

pub struct Txa {
    pub attr: InsAttr,
}

pub struct Txs {
    pub attr: InsAttr,
}

pub struct Tya {
    pub attr: InsAttr,
}

impl Mos6502Ins for Tax {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Tay {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Tsx {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Txa {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Txs {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}

impl Mos6502Ins for Tya {
    fn execute(&self, cpu: &mut Mos6502) {
        todo!()
    }
}
