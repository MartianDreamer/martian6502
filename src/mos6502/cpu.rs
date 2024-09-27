pub struct Mos6502 {
    pc: u16,
    sp: u8,
    acc: u8, // Accumulator
    x: u8,
    y: u8,
    ps: u8, // Processing status layout: NV-BDIZC
    mem: [u8; 64 * 1024],
}
