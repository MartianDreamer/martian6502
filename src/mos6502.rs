mod address_mode;
mod constant;
mod insset;

use console::Term;
use constant::BIT_0_MASK;
use insset::parser::parse;
use insset::{InsAttr, Mos6502Ins};

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
            let ins: Box<dyn Mos6502Ins> = parse(self.mem[self.pc as usize] as u8);
            ins.execute(self);
        }
    }

    pub fn debug(self: &mut Self) {
        self.power_on = true;
        let stdout = Term::stdout();
        while self.power_on {
            let ins: Box<dyn Mos6502Ins> = parse(self.mem[self.pc as usize] as u8);
            println!(
                "pc: {}\nsp: {}\nac: {}\nxr: {}\nyr: {}\nsr: {}\nins_opcode: {}\n",
                self.pc, self.sp, self.ac, self.xr, self.yr, self.sr, self.mem[self.pc as usize]
            );
            if let Ok(_) = stdout.read_char() {}
            ins.execute(self);
        }
    }

    pub fn stop(self: &mut Self) {
        self.power_on = false
    }

    fn next_instruction(self: &mut Self, attr: &InsAttr) {
        self.pc += attr.len() as u16;
    }

    fn is_carried(self: &Self) -> u8 {
        self.sr & BIT_0_MASK
    }
}

impl Default for Mos6502 {
    fn default() -> Self {
        Self {
            pc: 0,
            sp: 0,
            ac: 0,
            xr: 0,
            yr: 0,
            sr: 0,
            mem: [0; 64 * 1024],
            power_on: false,
        }
    }
}
