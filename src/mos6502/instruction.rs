use std::{error::Error, fmt::Display};

use super::constant::{LOWER_NIBBLE_MASK, UPPER_NIBBLE_MASK};

/**
 * referrence document
 * http://www.6502.org/users/obelisk/
 * this module holds all code to describe instructions of 6502 and how to parse them
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

enum Mos6502Instruction {
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
    Brk(InstructionAttribute),

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
    Clv(InstructionAttribute),

    // CMP - Compare
    CmpImm(InstructionAttribute),
    CmpZP(InstructionAttribute),
    CmpZPX(InstructionAttribute),
    CmpAbs(InstructionAttribute),
    CmpAbsX(InstructionAttribute),
    CmpAbsY(InstructionAttribute),
    CmpIndX(InstructionAttribute),
    CmpIndY(InstructionAttribute),

    // CPX - Compare X Register
    CpxImm(InstructionAttribute),
    CpxZP(InstructionAttribute),
    CpxAbs(InstructionAttribute),

    // CPY - Compare Y Register
    CpyImm(InstructionAttribute),
    CpyZP(InstructionAttribute),
    CpyAbs(InstructionAttribute),

    // DEC - Decrement Memory
    DecZP(InstructionAttribute),
    DecZPX(InstructionAttribute),
    DecAbs(InstructionAttribute),
    DecAbsX(InstructionAttribute),

    // DEX - Decrement X Register
    Dex(InstructionAttribute),

    // DEY - Decrement Y Register
    Dey(InstructionAttribute),

    // EOR - Exclusive OR
    EorImm(InstructionAttribute),
    EorZP(InstructionAttribute),
    EorZPX(InstructionAttribute),
    EorAbs(InstructionAttribute),
    EorAbsX(InstructionAttribute),
    EorAbsY(InstructionAttribute),
    EorIndX(InstructionAttribute),
    EorIndY(InstructionAttribute),

    // INC - Increment Memory
    IncZP(InstructionAttribute),
    IncZPX(InstructionAttribute),
    IncAbs(InstructionAttribute),
    IncAbsX(InstructionAttribute),

    // INX - Increment X Register
    Inx(InstructionAttribute),

    // INY - Increment Y Register
    Iny(InstructionAttribute),

    // JMP - Jump
    JmpAbs(InstructionAttribute),
    JmpInd(InstructionAttribute),

    // JSR - Jump to Subroutine
    Jsr(InstructionAttribute),

    // LDA - Load Accumulator
    LdaImm(InstructionAttribute),
    LdaZP(InstructionAttribute),
    LdaZPX(InstructionAttribute),
    LdaAbs(InstructionAttribute),
    LdaAbsX(InstructionAttribute),
    LdaAbsY(InstructionAttribute),
    LdaIndX(InstructionAttribute),
    LdaIndY(InstructionAttribute),

    // LDX - Load X Register
    LdxImm(InstructionAttribute),
    LdxZP(InstructionAttribute),
    LdxZPY(InstructionAttribute),
    LdxAbs(InstructionAttribute),
    LdxAbsY(InstructionAttribute),

    // LDY - Load Y Register
    LdyImm(InstructionAttribute),
    LdyZP(InstructionAttribute),
    LdyZPX(InstructionAttribute),
    LdyAbs(InstructionAttribute),
    LdyAbsX(InstructionAttribute),

    // LSR - Logical Shift Right
    LsrAcc(InstructionAttribute),
    LsrZP(InstructionAttribute),
    LsrZPX(InstructionAttribute),
    LsrAbs(InstructionAttribute),
    LsrAbsX(InstructionAttribute),

    // NOP - No Operation
    Nop(InstructionAttribute),

    // ORA - Logical Inclusive OR
    OraImm(InstructionAttribute),
    OraZP(InstructionAttribute),
    OraZPX(InstructionAttribute),
    OraAbs(InstructionAttribute),
    OraAbsX(InstructionAttribute),
    OraAbsY(InstructionAttribute),
    OraIndX(InstructionAttribute),
    OraIndY(InstructionAttribute),

    // PHA - Push Accumulator
    Pha(InstructionAttribute),

    // PHP - Push Processor Status
    Php(InstructionAttribute),

    // PLA - Pull Accumulator
    Pla(InstructionAttribute),

    // PLP - Pull Processor Status
    Plp(InstructionAttribute),

    // ROL - Rotate Left
    RolAcc(InstructionAttribute),
    RolZP(InstructionAttribute),
    RolZPX(InstructionAttribute),
    RolAbs(InstructionAttribute),
    RolAbsX(InstructionAttribute),

    // ROR - Rotate Right
    RorAcc(InstructionAttribute),
    RorZP(InstructionAttribute),
    RorZPX(InstructionAttribute),
    RorAbs(InstructionAttribute),
    RorAbsX(InstructionAttribute),

    // RTI - Return from Interrupt
    Rti(InstructionAttribute),

    // RTS - Return from Subroutine
    Rts(InstructionAttribute),

    // SBC - Subtract with Carry
    SbcImm(InstructionAttribute),
    SbcZP(InstructionAttribute),
    SbcZPX(InstructionAttribute),
    SbcAbs(InstructionAttribute),
    SbcAbsX(InstructionAttribute),
    SbcAbsY(InstructionAttribute),
    SbcIndX(InstructionAttribute),
    SbcIndY(InstructionAttribute),

    // SEC - Set Carry Flag
    Sec(InstructionAttribute),

    // SED - Set Decimal Flag
    Sed(InstructionAttribute),

    // SEI - Set Interrupt Disable
    Sei(InstructionAttribute),

    // STA - Store Accumulator
    StaImm(InstructionAttribute),
    StaZP(InstructionAttribute),
    StaZPX(InstructionAttribute),
    StaAbs(InstructionAttribute),
    StaAbsX(InstructionAttribute),
    StaAbsY(InstructionAttribute),
    StaIndX(InstructionAttribute),
    StaIndY(InstructionAttribute),

    // STX - Store X Register
    StxZP(InstructionAttribute),
    StxZPY(InstructionAttribute),
    StxAbs(InstructionAttribute),

    // STY - Store Y Register
    StyZP(InstructionAttribute),
    StyZPX(InstructionAttribute),
    StyAbs(InstructionAttribute),

    // TAX - Transfer Accumulator to X
    Tax(InstructionAttribute),

    // TAY - Transfer Accumulator to Y
    Tay(InstructionAttribute),

    // TSX - Transfer Stack Pointer to X
    Tsx(InstructionAttribute),

    // TXA - Transfer X to Accumulator
    Txa(InstructionAttribute),

    // TXS - Transfer X to Stack Pointer
    Txs(InstructionAttribute),

    // TYA - Transfer Y to Accumulator
    Tya(InstructionAttribute),
}

struct InstructionAttribute {
    opcode: u8,
    length: u8,
    cycles: u8,
}

impl Mos6502Instruction {
    /**
     * parse a byte to a 6502 instruction
     */
    pub fn parse(opcode: u8) -> Result<Mos6502Instruction, InvalidOpcodeError> {
        let upper_nibble = opcode & UPPER_NIBBLE_MASK;
        match upper_nibble {
            0x0 => Self::parse_prefix_0_instruction(opcode),
            0x1 => Self::parse_prefix_1_instruction(opcode),
            0x2 => Self::parse_prefix_2_instruction(opcode),
            _ => Err(InvalidOpcodeError { opcode }),
        }
    }

    fn parse_prefix_0_instruction(opcode: u8) -> Result<Mos6502Instruction, InvalidOpcodeError> {
        let lower_nibble = opcode & LOWER_NIBBLE_MASK;
        match lower_nibble {
            0x0 => Ok(Self::Brk(InstructionAttribute {
                opcode,
                length: 1,
                cycles: 7,
            })),
            0x1 => Ok(Self::OraIndX(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 5,
            })),
            0x5 => Ok(Self::OraZP(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 3,
            })),
            0x6 => Ok(Self::AslZP(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 5,
            })),
            0x8 => Ok(Self::Php(InstructionAttribute {
                opcode,
                length: 1,
                cycles: 3,
            })),
            0x9 => Ok(Self::OraImm(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 2,
            })),
            0xa => Ok(Self::AslAcc(InstructionAttribute {
                opcode,
                length: 1,
                cycles: 2,
            })),
            0xd => Ok(Self::OraAbs(InstructionAttribute {
                opcode,
                length: 3,
                cycles: 4,
            })),
            0xe => Ok(Self::AslAbs(InstructionAttribute {
                opcode,
                length: 3,
                cycles: 6,
            })),
            _ => Err(InvalidOpcodeError { opcode }),
        }
    }

    fn parse_prefix_1_instruction(opcode: u8) -> Result<Mos6502Instruction, InvalidOpcodeError> {
        let lower_nibble = opcode & LOWER_NIBBLE_MASK;
        match lower_nibble {
            0x0 => Ok(Self::Bpl(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 2,
            })),
            0x1 => Ok(Self::OraIndY(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 5,
            })),
            0x5 => Ok(Self::OraZPX(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 4,
            })),
            0x6 => Ok(Self::AslZPX(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 6,
            })),
            0x8 => Ok(Self::Clc(InstructionAttribute {
                opcode,
                length: 1,
                cycles: 2,
            })),
            0x9 => Ok(Self::OraAbsY(InstructionAttribute {
                opcode,
                length: 3,
                cycles: 4,
            })),
            0xd => Ok(Self::OraAbsX(InstructionAttribute {
                opcode,
                length: 3,
                cycles: 4,
            })),
            0xe => Ok(Self::AslAbsX(InstructionAttribute {
                opcode,
                length: 3,
                cycles: 7,
            })),
            _ => Err(InvalidOpcodeError { opcode }),
        }
    }

    fn parse_prefix_2_instruction(opcode: u8) -> Result<Mos6502Instruction, InvalidOpcodeError> {
        let lower_nibble = opcode & LOWER_NIBBLE_MASK;
        match lower_nibble {
            0x1 => Ok(Self::AndIndX(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 6,
            })),
            0x5 => Ok(Self::AndZP(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 3,
            })),
            _ => Err(InvalidOpcodeError { opcode }),
        }
    }
}
