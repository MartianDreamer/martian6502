use super::Mos6502;

pub type AddressModeFn = fn(&Mos6502) -> u8;

pub type AddressModeImmutableFn = fn(&mut Mos6502) -> &mut u8;

pub fn immediate(cpu: &Mos6502) -> u8 {
    next_nth_byte_from_pc(cpu, 1)
}

pub fn zero_page(cpu: &Mos6502) -> u8 {
    let address = next_nth_byte_from_pc(cpu, 1) as usize;
    cpu.mem[address]
}

pub fn zero_page_immutatble(cpu: &mut Mos6502) -> &mut u8 {
    let address = next_nth_byte_from_pc(cpu, 1) as usize;
    &mut cpu.mem[address]
}

#[allow(arithmetic_overflow)]
pub fn zero_page_x(cpu: &Mos6502) -> u8 {
    let address: u8 = next_nth_byte_from_pc(cpu, 1);
    let effective_address: u8 = address + cpu.xr;
    cpu.mem[effective_address as usize]
}

#[allow(arithmetic_overflow)]
pub fn zero_page_x_immutable(cpu: &mut Mos6502) -> &mut u8 {
    let address: u8 = next_nth_byte_from_pc(cpu, 1);
    let effective_address: u8 = address + cpu.xr;
    &mut cpu.mem[effective_address as usize]
}

pub fn absolute(cpu: &Mos6502) -> u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = (address_msb << 8 | address_lsb) as usize;
    cpu.mem[address]
}

pub fn absolute_immutable(cpu: &mut Mos6502) -> &mut u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = (address_msb << 8 | address_lsb) as usize;
    &mut cpu.mem[address]
}

pub fn absolute_x(cpu: &Mos6502) -> u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = address_msb << 8 | address_lsb;
    let effective_address = address + cpu.xr as u16;
    cpu.mem[effective_address as usize]
}

pub fn absolute_x_immutable(cpu: &mut Mos6502) -> &mut u8 {
    let address_lsb = next_nth_byte_from_pc(cpu, 1) as u16;
    let address_msb = next_nth_byte_from_pc(cpu, 2) as u16;
    let address = address_msb << 8 | address_lsb;
    let effective_address = address + cpu.xr as u16;
    &mut cpu.mem[effective_address as usize]
}

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
pub fn indirect_y(cpu: &Mos6502) -> u8 {
    let indirect_position: u8 = next_nth_byte_from_pc(cpu, 1);
    let indirect_position_next: u8 = indirect_position + 1;
    let address_lsb = cpu.mem[indirect_position as usize] as u16;
    let address_msb = cpu.mem[indirect_position_next as usize] as u16;
    let address = address_msb << 8 | address_lsb;
    let effective_address = address + cpu.yr as u16;
    cpu.mem[effective_address as usize]
}

#[allow(arithmetic_overflow)]
pub fn relative(cpu: &Mos6502) -> u16 {
    let offset: u8 = next_nth_byte_from_pc(cpu, 1);
    let mut pc_lsb = cpu.pc as u8;
    let pc_msb = (cpu.pc >> 8) as u8;
    pc_lsb += offset;
    ((pc_msb as u16) << 8) | pc_lsb as u16
}

fn next_nth_byte_from_pc(cpu: &Mos6502, nth: u16) -> u8 {
    let nth_byte = cpu.pc + nth;
    cpu.mem[nth_byte as usize]
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_immediate() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0xaa;
        let actual = immediate(&cpu);
        assert_eq!(0xaa, actual);
    }

    #[test]
    fn test_zero_page() {
        let mut cpu = Mos6502::default();
        cpu.pc = 30;
        cpu.mem[31] = 0xaa;
        cpu.mem[0xaa] = 0x12;
        let actual = zero_page(&cpu);
        assert_eq!(0x12, actual);
    }

    #[test]
    fn test_zero_page_x() {
        let mut cpu = Mos6502::default();
        cpu.pc = 30;
        cpu.mem[31] = 0xaa;
        cpu.xr = 0x1;
        cpu.mem[0xaa + 0x1] = 0x12;
        let actual = zero_page_x(&cpu);
        assert_eq!(0x12, actual);
    }

    #[test]
    fn test_absolute() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x30;
        cpu.mem[2] = 0x10;
        cpu.mem[0x1030] = 0xaa;
        let actual = absolute(&cpu);
        assert_eq!(0xaa, actual)
    }

    #[test]
    fn test_absolute_x() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x30;
        cpu.mem[2] = 0x10;
        cpu.xr = 0x12;
        cpu.mem[0x1030 + 0x12] = 0xaa;
        let actual = absolute_x(&cpu);
        assert_eq!(0xaa, actual)
    }

    #[test]
    fn test_absolute_y() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x30;
        cpu.mem[2] = 0x10;
        cpu.yr = 0x12;
        cpu.mem[0x1030 + 0x12] = 0xaa;
        let actual = absolute_y(&cpu);
        assert_eq!(0xaa, actual)
    }

    #[test]
    fn test_indirect_x() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x70;
        cpu.xr = 0x10;
        cpu.mem[0x70 + 0x10] = 0x20;
        cpu.mem[0x70 + 0x10 + 0x1] = 0x10;
        cpu.mem[0x1020] = 0xaa;
        let actual = indirect_x(&cpu);
        assert_eq!(0xaa, actual)
    }

    #[test]
    fn test_indirect_y() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x70;
        cpu.yr = 0x10;
        cpu.mem[0x70] = 0x20;
        cpu.mem[0x71] = 0x10;
        cpu.mem[0x1020 + 0x10] = 0xaa;
        let actual = indirect_y(&cpu);
        assert_eq!(0xaa, actual)
    }

    #[test]
    fn test_zero_page_immutable() {
        let mut cpu = Mos6502::default();
        cpu.pc = 30;
        cpu.mem[31] = 0xaa;
        cpu.mem[0xaa] = 0x12;
        let actual = zero_page_immutatble(&mut cpu);
        assert_eq!(0x12, *actual);
        *actual = 0xff;
        assert_eq!(0xff, *actual);
    }

    #[test]
    fn test_zero_page_x_immutable() {
        let mut cpu = Mos6502::default();
        cpu.pc = 30;
        cpu.mem[31] = 0xaa;
        cpu.xr = 0x1;
        cpu.mem[0xaa + 0x1] = 0x12;
        let actual = zero_page_x_immutable(&mut cpu);
        assert_eq!(0x12, *actual);
        *actual = 0xff;
        assert_eq!(0xff, *actual);
    }

    #[test]
    fn test_absolute_immutable() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x30;
        cpu.mem[2] = 0x10;
        cpu.mem[0x1030] = 0xaa;
        let actual = absolute_immutable(&mut cpu);
        assert_eq!(0xaa, *actual);
        *actual = 0xff;
        assert_eq!(0xff, *actual);
    }

    #[test]
    fn test_absolute_x_immutable() {
        let mut cpu = Mos6502::default();
        cpu.mem[cpu.pc as usize + 1] = 0x30;
        cpu.mem[2] = 0x10;
        cpu.xr = 0x12;
        cpu.mem[0x1030 + 0x12] = 0xaa;
        let actual = absolute_x_immutable(&mut cpu);
        assert_eq!(0xaa, *actual);
        *actual = 0xff;
        assert_eq!(0xff, *actual);
    }

    #[test]
    fn test_relative() {
        let mut cpu = Mos6502::default();
        cpu.pc = 0x00f8;
        cpu.mem[cpu.pc as usize + 1] = 0xf9;
        let actual = relative(&cpu);
        assert_eq!(0xf1, actual)
    }
}
