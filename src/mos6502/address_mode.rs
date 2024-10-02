use super::emulator::Mos6502;

pub fn immediate(cpu: &Mos6502) -> u8 {
    cpu.mem[cpu.pc as usize + 1]
}

pub fn zero_page(cpu: &Mos6502) -> u8 {
    let address = cpu.mem[cpu.pc as usize + 1] as usize;
    cpu.mem[address]
}
