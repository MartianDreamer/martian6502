mod mos6502;

fn main() {
    let val: u8 = 0x03;
    let mask: u8 = 0x10;
    let masked_val = val & mask;
    println!("{masked_val}");
}
