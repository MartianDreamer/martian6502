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
    AdcImm(InstructionAttribute),
    AdcZP(InstructionAttribute),
    AdcZPX(InstructionAttribute),
    AdcAbs(InstructionAttribute),
    AdcAbsX(InstructionAttribute),
    AdcAbsY(InstructionAttribute),
    AdcIndX(InstructionAttribute),
    AdcIndY(InstructionAttribute),
}

struct InstructionAttribute {
    opcode: u8,
    length: u8,
    cycles: u8,
}

impl Instruction {
    pub fn new(opcode: u8) -> Result<Instruction, InvalidOpcodeError> {
        match opcode {
            0x69 => Ok(Instruction::AdcImm(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 2,
            })),
            0x65 => Ok(Instruction::AdcZP(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 3,
            })),
            0x75 => Ok(Instruction::AdcZPX(InstructionAttribute {
                opcode,
                length: 2,
                cycles: 4,
            })),
            
            _ => Err(InvalidOpcodeError { opcode }),
        }
    }
}
