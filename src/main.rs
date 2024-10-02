mod mos6502;
mod constant;

fn main() {
    let int8: i8 = -127;
    let uns8: u8 = int8 as u8;
    println!("{:#b} -> {:#b}", int8, uns8);
    println!("{} -> {}", int8, uns8);
}
