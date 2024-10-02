mod mos6502;

fn main() {
    let lsb: u16 = 0xff;
    let msb: u16 = 0xaa;
    print!("{:#x}", msb << 8 | lsb)
}
