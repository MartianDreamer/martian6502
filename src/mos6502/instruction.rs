use std::{error::Error, fmt::Display};

/**
 * http://www.6502.org/users/obelisk/
 * go to 6502 then Reference
*/

#[derive(Debug, Clone)]
struct InvalidOpcodeError {
    opcode: u8,
}

impl Error for InvalidOpcodeError {}

impl Display for InvalidOpcodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instructions opcode is not valid")
    }
}

enum Instruction {
    // ADC - Add with Carry
    AdcImm(InstructionAttribute),
    AdcZP(InstructionAttribute),
    AdcZPX(InstructionAttribute),
    AdcAbs(InstructionAttribute),
    AdcAbsX(InstructionAttribute),
    AdcAbsY(InstructionAttribute),
    AdcIndX(InstructionAttribute),
    AdcIndY(InstructionAttribute),

    // AND - Logical AND
    AndImm(InstructionAttribute),
    AndZP(InstructionAttribute),
    AndZPX(InstructionAttribute),
    AndAbs(InstructionAttribute),
    AndAbsX(InstructionAttribute),
    AndAbsY(InstructionAttribute),
    AndIndX(InstructionAttribute),
    AndIndY(InstructionAttribute),

    // ASL - Arithmetic Shift Left
    AslAcc(InstructionAttribute),
    AslZP(InstructionAttribute),
    AslZPX(InstructionAttribute),
    AslAbs(InstructionAttribute),
    AslAbsX(InstructionAttribute),

    // BCC - Branch if Carry Clear
    Bcc(InstructionAttribute),

    // BCS - Branch if Carry Set
    Bcs(InstructionAttribute),

    // BEQ - Branch if Equal
    Beq(InstructionAttribute),

    // BIT - Bit Test
    BitZP(InstructionAttribute),
    BitAbs(InstructionAttribute),

    // BMI - Branch if Minus
    Bmi(InstructionAttribute),

    // BNE - Branch if Not Equal
    Bne(InstructionAttribute),

    // BPL - Branch if Positive
    Bpl(InstructionAttribute),

    // BRK - Force Interrupt
    Bkr(InstructionAttribute),

    // BVC - Branch if Overflow Clear
    Bvc(InstructionAttribute),

    // BVS - Branch if Overflow Set
    Bvs(InstructionAttribute),

    // CLC - Clear Carry Flag
    Clc(InstructionAttribute),

    // CLD - Clear Decimal Mode
    Cld(InstructionAttribute),

    // CLI - Clear Interrupt Disable
    Cli(InstructionAttribute),

    // CLV - Clear Overflow Flag
    Clv(InstructionAttribute)

}

struct InstructionAttribute {
    opcode: u8,
    length: u8,
    cycles: u8,
}

impl Instruction {}
