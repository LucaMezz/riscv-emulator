use crate::util::get_bits;

use super::decode::{BTypeParams, ITypeParams, InstructionFormat, JTypeParams, RTypeParams, STypeParams, UTypeParams, INSTRUCTION_PATTERNS};


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

    ADDW(RTypeParams),
    SUBW(RTypeParams),
    SLLW(RTypeParams),
    SRLW(RTypeParams),
    SRAW(RTypeParams),

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

    ADDIW(ITypeParams),
    SLLIW(ITypeParams),
    SRLIW(ITypeParams),
    SRAIW(ITypeParams),

    /**
     * Load from memory
     */
    LB(ITypeParams),
    LH(ITypeParams),
    LW(ITypeParams),
    LD(ITypeParams),
    LBU(ITypeParams),
    LHU(ITypeParams),
    LWU(ITypeParams),

    /**
     * Store to memory
     */
    SB(STypeParams),
    SH(STypeParams),
    SW(STypeParams),
    SD(STypeParams),

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

    /**
     * Multiplication extension
     */
    MUL(RTypeParams),
    MULH(RTypeParams),
    MULSU(RTypeParams),
    MULU(RTypeParams),
    DIV(RTypeParams),
    DIVU(RTypeParams),
    REM(RTypeParams),
    REMU(RTypeParams),
    
    MULW(RTypeParams),
    DIVW(RTypeParams),
    DIVWU(RTypeParams),
    REMW(RTypeParams),
    REMWU(RTypeParams),
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
                InstructionFormat::IType { opcode, funct3, predicate, make } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        let params = ITypeParams::from(inst);
                        if predicate.is_none_or(|x| x(&params)) {
                            return make(params);
                        }
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

#[cfg(test)]
mod tests {
    use crate::isa::{decode::{BTypeParams, ITypeParams, RTypeParams, STypeParams}, Instruction};

    #[test]
    pub fn it_decodes_add_and_sub_correctly() {
        let inst = Instruction::decode(0b0000000_01100_00111_000_10101_0110011);
        let expected = Instruction::ADD(RTypeParams {
            rs1: 7,
            rs2: 12,
            rd: 21,
        });
        assert_eq!(inst, expected);

        let inst = Instruction::decode(0b0000000_01100_00111_010_10101_0110011);
        let expected = Instruction::ADD(RTypeParams {
            rs1: 7,
            rs2: 12,
            rd: 21,
        });
        assert_ne!(inst, expected);

        let inst = Instruction::decode(0b0100000_01100_00111_000_10101_0110011);
        let expected = Instruction::SUB(RTypeParams {
            rs1: 7,
            rs2: 12,
            rd: 21,
        });
        assert_eq!(inst, expected);
    }

    #[test]
    pub fn it_decodes_srli_and_srai_correctly() {
        let inst = Instruction::decode(0b0000000_01100_00111_101_10101_0010011);
        let expected = Instruction::SRLI(ITypeParams {
            rs1: 7,
            rd: 21,
            imm: 12
        });
        assert_eq!(inst, expected);

        let inst = Instruction::decode(0b0100000_01100_00111_101_10101_0010011);
        let expected = Instruction::SRAI(ITypeParams {
            rs1: 7,
            rd: 21,
            imm: 1036,
        });
        assert_eq!(inst, expected);
    }

    #[test]
    pub fn it_decodes_store_instrs_correctly() {
        let inst = Instruction::decode(0b0100011_01100_00111_001_10101_0100011);
        let expected = Instruction::SH(STypeParams {
            rs1: 7,
            rs2: 12,
            imm: 1141,
        });
        assert_eq!(inst, expected);
    }

    #[test]
    pub fn it_decodes_branch_instrs_correctly() {
        let inst = Instruction::decode(0b0100011_01100_00111_000_10101_1100011);
        let expected = Instruction::BEQ(BTypeParams {
            rs1: 7,
            rs2: 12,
            imm: 1594,
        });
        assert_eq!(inst, expected);
    }

    #[test]
    pub fn it_decodes_ecall_and_ebreak_correctly() {
        let inst = Instruction::decode(0b0000000_00000_00111_000_10101_1110011);
        let expected = Instruction::ECALL(ITypeParams {
            rs1: 7,
            rd: 21,
            imm: 0,
        });
        assert_eq!(inst, expected);

        let inst = Instruction::decode(0b0000000_00001_00111_000_10101_1110011);
        let expected = Instruction::EBREAK(ITypeParams {
            rs1: 7,
            rd: 21,
            imm: 1,
        });
        assert_eq!(inst, expected);
    }

    #[test]
    pub fn it_decodes_correctly() {
        let inst = Instruction::decode(0x00850793);
        let expected = Instruction::ADDI(ITypeParams {
            rs1: 10,
            rd: 15,
            imm: 8,
        });
        assert_eq!(inst, expected);
    }
}