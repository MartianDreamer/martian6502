use super::Mos6502;

pub type AddressModeFn = fn(&Mos6502) -> u8;

pub fn immediate(cpu: &Mos6502) -> u8 {
    next_nth_byte_from_pc(cpu, 1)
}

pub fn zero_page(cpu: &Mos6502) -> u8 {
    let address = next_nth_byte_from_pc(cpu, 1) as usize;
    cpu.mem[address]
}

#[allow(arithmetic_overflow)]
pub fn zero_page_x(cpu: &Mos6502) -> u8 {
    let address: u8 = next_nth_byte_from_pc(cpu, 1);
    let effective_address: u8 = address + cpu.xr;
    cpu.mem[effective_address as usize]
}

pub fn absolute(cpu: &Mos6502) -> u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = (address_msb << 8 | address_lsb) as usize;
    return cpu.mem[address];
}

#[allow(arithmetic_overflow)]
pub fn absolute_x(cpu: &Mos6502) -> u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = address_msb << 8 | address_lsb;
    let effective_address = address + cpu.xr as u16;
    cpu.mem[effective_address as usize]
}

#[allow(arithmetic_overflow)]
pub fn absolute_y(cpu: &Mos6502) -> u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = address_msb << 8 | address_lsb;
    let effective_address = address + cpu.yr as u16;
    cpu.mem[effective_address as usize]
}

#[allow(arithmetic_overflow)]
pub fn indirect_x(cpu: &Mos6502) -> u8 {
    let table: u8 = next_nth_byte_from_pc(cpu, 1);
    let record_first_byte: u8 = table + cpu.xr;
    let record_second_byte: u8 = record_first_byte + 1;
    let address_lsb = cpu.mem[record_first_byte as usize] as u16;
    let address_msb = cpu.mem[record_second_byte as usize] as u16;
    let effective_address = (address_msb << 8 | address_lsb) as usize;
    cpu.mem[effective_address]
}

#[allow(arithmetic_overflow)]
fn next_nth_byte_from_pc(cpu: &Mos6502, nth: u16) -> u8 {
    let nth_byte = cpu.pc + nth;
    cpu.mem[nth_byte as usize]
}
