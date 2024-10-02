use super::Mos6502;

pub fn immediate(cpu: &Mos6502) -> u8 {
    cpu.mem[cpu.pc as usize + 1]
}

pub fn zero_page(cpu: &Mos6502) -> u8 {
    let address = cpu.mem[cpu.pc as usize + 1] as usize;
    cpu.mem[address]
}

#[allow(arithmetic_overflow)]
pub fn zero_page_x(cpu: &Mos6502) -> u8 {
    let address: u8 = cpu.mem[cpu.pc as usize + 1];
    let effective_address: u8 = address + cpu.x;
    cpu.mem[effective_address as usize]
}
