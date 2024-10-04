use mos6502::Mos6502;

mod mos6502;

fn main() {
    let mut cpu = Mos6502::default();
    cpu.debug();
}
