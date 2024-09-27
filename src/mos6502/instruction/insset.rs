///
/// Mos6502 instruction sets
///
pub enum M6502Ins {
    // ADC - Add with Carry
    AdcImm(InsAttr),
    AdcZP(InsAttr),
    AdcZPX(InsAttr),
    AdcAbs(InsAttr),
    AdcAbsX(InsAttr),
    AdcAbsY(InsAttr),
    AdcIndX(InsAttr),
    AdcIndY(InsAttr),

    // AND - Logical AND
    AndImm(InsAttr),
    AndZP(InsAttr),
    AndZPX(InsAttr),
    AndAbs(InsAttr),
    AndAbsX(InsAttr),
    AndAbsY(InsAttr),
    AndIndX(InsAttr),
    AndIndY(InsAttr),

    // ASL - Arithmetic Shift Left
    AslAcc(InsAttr),
    AslZP(InsAttr),
    AslZPX(InsAttr),
    AslAbs(InsAttr),
    AslAbsX(InsAttr),

    // BCC - Branch if Carry Clear
    Bcc(InsAttr),

    // BCS - Branch if Carry Set
    Bcs(InsAttr),

    // BEQ - Branch if Equal
    Beq(InsAttr),

    // BIT - Bit Test
    BitZP(InsAttr),
    BitAbs(InsAttr),

    // BMI - Branch if Minus
    Bmi(InsAttr),

    // BNE - Branch if Not Equal
    Bne(InsAttr),

    // BPL - Branch if Positive
    Bpl(InsAttr),

    // BRK - Force Interrupt
    Brk(InsAttr),

    // BVC - Branch if Overflow Clear
    Bvc(InsAttr),

    // BVS - Branch if Overflow Set
    Bvs(InsAttr),

    // CLC - Clear Carry Flag
    Clc(InsAttr),

    // CLD - Clear Decimal Mode
    Cld(InsAttr),

    // CLI - Clear Interrupt Disable
    Cli(InsAttr),

    // CLV - Clear Overflow Flag
    Clv(InsAttr),

    // CMP - Compare
    CmpImm(InsAttr),
    CmpZP(InsAttr),
    CmpZPX(InsAttr),
    CmpAbs(InsAttr),
    CmpAbsX(InsAttr),
    CmpAbsY(InsAttr),
    CmpIndX(InsAttr),
    CmpIndY(InsAttr),

    // CPX - Compare X Register
    CpxImm(InsAttr),
    CpxZP(InsAttr),
    CpxAbs(InsAttr),

    // CPY - Compare Y Register
    CpyImm(InsAttr),
    CpyZP(InsAttr),
    CpyAbs(InsAttr),

    // DEC - Decrement Memory
    DecZP(InsAttr),
    DecZPX(InsAttr),
    DecAbs(InsAttr),
    DecAbsX(InsAttr),

    // DEX - Decrement X Register
    Dex(InsAttr),

    // DEY - Decrement Y Register
    Dey(InsAttr),

    // EOR - Exclusive OR
    EorImm(InsAttr),
    EorZP(InsAttr),
    EorZPX(InsAttr),
    EorAbs(InsAttr),
    EorAbsX(InsAttr),
    EorAbsY(InsAttr),
    EorIndX(InsAttr),
    EorIndY(InsAttr),

    // INC - Increment Memory
    IncZP(InsAttr),
    IncZPX(InsAttr),
    IncAbs(InsAttr),
    IncAbsX(InsAttr),

    // INX - Increment X Register
    Inx(InsAttr),

    // INY - Increment Y Register
    Iny(InsAttr),

    // JMP - Jump
    JmpAbs(InsAttr),
    JmpInd(InsAttr),

    // JSR - Jump to Subroutine
    Jsr(InsAttr),

    // LDA - Load Accumulator
    LdaImm(InsAttr),
    LdaZP(InsAttr),
    LdaZPX(InsAttr),
    LdaAbs(InsAttr),
    LdaAbsX(InsAttr),
    LdaAbsY(InsAttr),
    LdaIndX(InsAttr),
    LdaIndY(InsAttr),

    // LDX - Load X Register
    LdxImm(InsAttr),
    LdxZP(InsAttr),
    LdxZPY(InsAttr),
    LdxAbs(InsAttr),
    LdxAbsY(InsAttr),

    // LDY - Load Y Register
    LdyImm(InsAttr),
    LdyZP(InsAttr),
    LdyZPX(InsAttr),
    LdyAbs(InsAttr),
    LdyAbsX(InsAttr),

    // LSR - Logical Shift Right
    LsrAcc(InsAttr),
    LsrZP(InsAttr),
    LsrZPX(InsAttr),
    LsrAbs(InsAttr),
    LsrAbsX(InsAttr),

    // NOP - No Operation
    Nop(InsAttr),

    // ORA - Logical Inclusive OR
    OraImm(InsAttr),
    OraZP(InsAttr),
    OraZPX(InsAttr),
    OraAbs(InsAttr),
    OraAbsX(InsAttr),
    OraAbsY(InsAttr),
    OraIndX(InsAttr),
    OraIndY(InsAttr),

    // PHA - Push Accumulator
    Pha(InsAttr),

    // PHP - Push Processor Status
    Php(InsAttr),

    // PLA - Pull Accumulator
    Pla(InsAttr),

    // PLP - Pull Processor Status
    Plp(InsAttr),

    // ROL - Rotate Left
    RolAcc(InsAttr),
    RolZP(InsAttr),
    RolZPX(InsAttr),
    RolAbs(InsAttr),
    RolAbsX(InsAttr),

    // ROR - Rotate Right
    RorAcc(InsAttr),
    RorZP(InsAttr),
    RorZPX(InsAttr),
    RorAbs(InsAttr),
    RorAbsX(InsAttr),

    // RTI - Return from Interrupt
    Rti(InsAttr),

    // RTS - Return from Subroutine
    Rts(InsAttr),

    // SBC - Subtract with Carry
    SbcImm(InsAttr),
    SbcZP(InsAttr),
    SbcZPX(InsAttr),
    SbcAbs(InsAttr),
    SbcAbsX(InsAttr),
    SbcAbsY(InsAttr),
    SbcIndX(InsAttr),
    SbcIndY(InsAttr),

    // SEC - Set Carry Flag
    Sec(InsAttr),

    // SED - Set Decimal Flag
    Sed(InsAttr),

    // SEI - Set Interrupt Disable
    Sei(InsAttr),

    // STA - Store Accumulator
    StaImm(InsAttr),
    StaZP(InsAttr),
    StaZPX(InsAttr),
    StaAbs(InsAttr),
    StaAbsX(InsAttr),
    StaAbsY(InsAttr),
    StaIndX(InsAttr),
    StaIndY(InsAttr),

    // STX - Store X Register
    StxZP(InsAttr),
    StxZPY(InsAttr),
    StxAbs(InsAttr),

    // STY - Store Y Register
    StyZP(InsAttr),
    StyZPX(InsAttr),
    StyAbs(InsAttr),

    // TAX - Transfer Accumulator to X
    Tax(InsAttr),

    // TAY - Transfer Accumulator to Y
    Tay(InsAttr),

    // TSX - Transfer Stack Pointer to X
    Tsx(InsAttr),

    // TXA - Transfer X to Accumulator
    Txa(InsAttr),

    // TXS - Transfer X to Stack Pointer
    Txs(InsAttr),

    // TYA - Transfer Y to Accumulator
    Tya(InsAttr),
}

pub struct InsAttr {
    opcode: u8, // opcode of this instruction
    len: u8,    // length of this instruction
    cyc: u8,    // number of cpu cycle to complete this instruction
}

impl InsAttr {
    pub fn new(opcode: u8, len: u8, cyc: u8) -> Self {
        Self { opcode, len, cyc }
    }

    pub fn opcode(self: &Self) -> u8 {
        self.opcode
    }

    pub fn len(self: &Self) -> u8 {
        self.len
    }

    pub fn cyc(self: &Self) -> u8 {
        self.cyc
    }
}
