mod insset;
mod address_mode;
mod constant;

use insset::InsAttr;
use insset::parser::parse;

pub struct Mos6502 {
    pc: u16,
    sp: u8,
    ac: u8, // Accumulator
    xr: u8,
    yr: u8,
    sr: u8, // Processing status layout: NV-BDIZC
    mem: [u8; 64 * 1024],
    power_on: bool,
}

impl Mos6502 {
    pub fn start(self: &mut Self) {
        self.power_on = true;
        while self.power_on {
            let ins = parse(self.mem[self.pc as usize] as u8);
            ins.execute(self);
        }
    }

    pub fn stop(self: &mut Self) {
        self.power_on = false
    }

    fn next_instruction(self: &mut Self, attr: &InsAttr) {
        self.pc += attr.len() as u16;
    }
}