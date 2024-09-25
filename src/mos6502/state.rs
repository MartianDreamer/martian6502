pub struct Mos6502 {
    pc: u16,
    sp: u8,
    accumulator: u8,
    x: u8,
    y: u8,
    processor_status: u8,
    mem: [u8; 64 * 1024],
}
