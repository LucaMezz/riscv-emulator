use derive_new::new;
use lazy_static::lazy_static;

use crate::util::get_bits;

use super::instruction::Instruction::{self, *};

lazy_static! {
    pub static ref INSTRUCTION_PATTERNS: Vec<InstructionFormat> = vec![
        InstructionFormat::new_r_type(0b0110011, 0x0, 0x00, ADD),
        InstructionFormat::new_r_type(0b0110011, 0x0, 0x20, SUB),
        InstructionFormat::new_r_type(0b0110011, 0x4, 0x00, XOR),
        InstructionFormat::new_r_type(0b0110011, 0x6, 0x00, OR),
        InstructionFormat::new_r_type(0b0110011, 0x7, 0x00, AND),
        InstructionFormat::new_r_type(0b0110011, 0x1, 0x00, SLL),
        InstructionFormat::new_r_type(0b0110011, 0x5, 0x00, SRL),
        InstructionFormat::new_r_type(0b0110011, 0x5, 0x20, SRA),
        InstructionFormat::new_r_type(0b0110011, 0x2, 0x00, SLT),
        InstructionFormat::new_r_type(0b0110011, 0x3, 0x00, SLTU),

        InstructionFormat::new_r_type(0b0110011, 0x0, 0x01, MUL),
        InstructionFormat::new_r_type(0b0110011, 0x1, 0x01, MULH),
        InstructionFormat::new_r_type(0b0110011, 0x2, 0x01, MULSU),
        InstructionFormat::new_r_type(0b0110011, 0x3, 0x01, MULU),
        InstructionFormat::new_r_type(0b0110011, 0x4, 0x01, DIV),
        InstructionFormat::new_r_type(0b0110011, 0x5, 0x01, DIVU),
        InstructionFormat::new_r_type(0b0110011, 0x6, 0x01, REM),
        InstructionFormat::new_r_type(0b0110011, 0x7, 0x01, REMU),

        InstructionFormat::new_i_type(0b0010011, 0x0, ADDI),
        InstructionFormat::new_i_type(0b0010011, 0x4, XORI),
        InstructionFormat::new_i_type(0b0010011, 0x6, ORI),
        InstructionFormat::new_i_type(0b0010011, 0x7, ANDI),
        InstructionFormat::new_i_type(0b0010011, 0x1, SLLI),
        InstructionFormat::new_i_type(0b0010011, 0x5, SRLI),
        InstructionFormat::new_i_type(0b0010011, 0x5, SRAI),
        InstructionFormat::new_i_type(0b0010011, 0x2, SLTI),
        InstructionFormat::new_i_type(0b0010011, 0x3, SLTIU),

        InstructionFormat::new_i_type(0b0000011, 0x0, LB),
        InstructionFormat::new_i_type(0b0000011, 0x1, LH),
        InstructionFormat::new_i_type(0b0000011, 0x2, LW),
        InstructionFormat::new_i_type(0b0000011, 0x4, LBU),
        InstructionFormat::new_i_type(0b0000011, 0x5, LHU),

        InstructionFormat::new_s_type(0b0100011, 0x0, SB),
        InstructionFormat::new_s_type(0b0100011, 0x1, SH),
        InstructionFormat::new_s_type(0b0100011, 0x2, SW),

        InstructionFormat::new_b_type(0b1100011, 0x0, BEQ),
        InstructionFormat::new_b_type(0b1100011, 0x1, BNE),
        InstructionFormat::new_b_type(0b1100011, 0x4, BLT),
        InstructionFormat::new_b_type(0b1100011, 0x5, BGE),
        InstructionFormat::new_b_type(0b1100011, 0x6, BLTU),
        InstructionFormat::new_b_type(0b1100011, 0x7, BGEU),

        InstructionFormat::new_j_type(0b1101111, JAL),
        InstructionFormat::new_i_type(0b1100111, 0x0, JALR),

        InstructionFormat::new_u_type(0b0110111, LUI),
        InstructionFormat::new_u_type(0b0010111, AUIPC),

        InstructionFormat::new_i_type(0b1110011, 0x0, ECALL),
        InstructionFormat::new_i_type(0b1110011, 0x0, EBREAK)
    ];
}

#[derive(new)]
pub enum InstructionFormat {
    RType {
        opcode: u32,
        funct3: u32,
        funct7: u32,
        make: fn(RTypeParams) -> Instruction
    },
    IType {
        opcode: u32,
        funct3: u32,
        make: fn(ITypeParams) -> Instruction
    },
    SType {
        opcode: u32,
        funct3: u32,
        make: fn(STypeParams) -> Instruction
    },
    BType {
        opcode: u32,
        funct3: u32,
        make: fn(BTypeParams) -> Instruction
    },
    UType {
        opcode: u32,
        make: fn(UTypeParams) -> Instruction
    },
    JType {
        opcode: u32,
        make: fn(JTypeParams) -> Instruction
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RTypeParams {
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,
}

impl RTypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rs1: get_bits(inst, 15, 19),
            rs2: get_bits(inst, 20, 24),
            rd: get_bits(inst, 7, 11),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ITypeParams {
    pub rs1: u32,
    pub rd: u32,
    pub imm: u32,
}

impl ITypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rs1: get_bits(inst, 15, 19),
            rd: get_bits(inst, 7, 11),
            imm: get_bits(inst, 20, 31),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct STypeParams {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,
}

impl STypeParams {
    pub fn from(inst: u32) -> Self {
        let imm1 = get_bits(inst, 7, 11);
        let imm2 = get_bits(inst, 25, 31);
        
        Self {
            rs1: get_bits(inst, 15, 19),
            rs2: get_bits(inst, 20, 24),
            imm: (imm2 << 5) | imm1
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BTypeParams {
    pub rs1: u32,
    pub rs2: u32,
    pub imm: u32,
}

impl BTypeParams {
    pub fn from(inst: u32) -> Self {
        let imm1 = get_bits(inst, 8, 11);
        let imm2 = get_bits(inst, 25, 30);
        let imm3 = get_bits(inst, 7, 7);
        let imm4 = get_bits(inst, 31, 31);

        Self {
            rs1: get_bits(inst, 15, 19),
            rs2: get_bits(inst, 20, 24),
            imm: (imm4 << 11) | (imm3 << 10) | (imm2 << 4) | imm1
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct JTypeParams {
    pub rd: u32,
    pub imm: u32,
}

impl JTypeParams {
    pub fn from(_inst: u32) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UTypeParams {
    pub rd: u32,
    pub imm: u32,
}

impl UTypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rd: get_bits(inst, 7, 11),
            imm: get_bits(inst, 12, 31)
        }
    }
}