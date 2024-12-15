use crate::util::get_bits;

use super::format::{BTypeParams, ITypeParams, InstructionFormat, JTypeParams, RTypeParams, STypeParams, UTypeParams, INSTRUCTION_PATTERNS};


#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    // UNDEF: Undefined instruction.
    UNDEF,

    /**
     * Binary Operations
     */
    ADD(RTypeParams),
    SUB(RTypeParams),
    XOR(RTypeParams),
    OR(RTypeParams),
    AND(RTypeParams),
    SLL(RTypeParams),
    SRL(RTypeParams),
    SRA(RTypeParams),
    SLT(RTypeParams),
    SLTU(RTypeParams),
    /**
     * Muleiplication extension
     */
    MUL(RTypeParams),
    MULH(RTypeParams),
    MULSU(RTypeParams),
    MULU(RTypeParams),
    DIV(RTypeParams),
    DIVU(RTypeParams),
    REM(RTypeParams),
    REMU(RTypeParams),

    /**
     * Binary Operations with an Immediate operand
     */
    ADDI(ITypeParams),
    XORI(ITypeParams),
    ORI(ITypeParams),
    ANDI(ITypeParams),
    SLLI(ITypeParams),
    SRLI(ITypeParams),
    SRAI(ITypeParams),
    SLTI(ITypeParams),
    SLTIU(ITypeParams),

    /**
     * Load from memory
     */
    LB(ITypeParams),
    LH(ITypeParams),
    LW(ITypeParams),
    LBU(ITypeParams),
    LHU(ITypeParams),

    /**
     * Store to memory
     */
    SB(STypeParams),
    SH(STypeParams),
    SW(STypeParams),

    /**
     * Branching
     */
    BEQ(BTypeParams),
    BNE(BTypeParams),
    BLT(BTypeParams),
    BGE(BTypeParams),
    BLTU(BTypeParams),
    BGEU(BTypeParams),

    /**
     * Jumping
     */
    JAL(JTypeParams),
    JALR(ITypeParams),

    /**
     * Upper immediates
     */
    LUI(UTypeParams),
    AUIPC(UTypeParams),

    /**
     * Environment
     */
    ECALL(ITypeParams),
    EBREAK(ITypeParams),
}

impl Instruction {
    pub fn decode(inst: u32) -> Self {
        for pattern in INSTRUCTION_PATTERNS.iter() {
            let inst_opcode = get_bits(inst, 0, 6);
            let inst_funct3 = get_bits(inst, 12, 14);
            let inst_funct7 = get_bits(inst, 25, 31);

            match pattern {
                InstructionFormat::RType { opcode, funct3, funct7, make } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 && *funct7 == inst_funct7 {
                        let params = RTypeParams::from(inst);
                        return make(params);
                    }
                },
                InstructionFormat::IType { opcode, funct3, make } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        let params = ITypeParams::from(inst);
                        return make(params);
                    }
                },
                InstructionFormat::SType { opcode, funct3, make } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        let params = STypeParams::from(inst);
                        return make(params);
                    }
                },
                InstructionFormat::BType { opcode, funct3, make } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        let params = BTypeParams::from(inst);
                        return make(params);
                    }
                },
                InstructionFormat::JType { opcode, make } => {
                    if *opcode == inst_opcode {
                        let params = JTypeParams::from(inst);
                        return make(params);
                    }
                },
                InstructionFormat::UType { opcode, make } => {
                    if *opcode == inst_opcode {
                        let params = UTypeParams::from(inst);
                        return make(params);
                    }
                }
            }
        }

        Instruction::UNDEF
    }
}