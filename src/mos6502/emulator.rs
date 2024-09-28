use super::parser::parse;

pub struct Mos6502 {
    pub pc: u16,
    pub sp: u8,
    pub acc: u8, // Accumulator
    pub x: u8,
    pub y: u8,
    pub ps: u8, // Processing status layout: NV-BDIZC
    pub mem: [u8; 64 * 1024],
    power_on: bool,
}

impl Mos6502 {
    pub fn start(self: &mut Self) {
        self.power_on = true;
        while self.power_on {
            let ins = parse(self.mem[self.pc as usize]);
            ins.execute(self);
        }
    }

    pub fn stop(self: &mut Self) {
        self.power_on = false
    }
}
