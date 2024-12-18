use derive_new::new;
use lazy_static::lazy_static;

use crate::util::{get_bits, sign_extend_32};

use super::Instruction::{self, *};

lazy_static! {
    pub static ref INSTRUCTION_PATTERNS: Vec<InstructionFormat> = vec![
        // RV32I Base Instruction Set
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

        InstructionFormat::new_i_type(0b0010011, 0x0, None, ADDI),
        InstructionFormat::new_i_type(0b0010011, 0x4, None, XORI),
        InstructionFormat::new_i_type(0b0010011, 0x6, None, ORI),
        InstructionFormat::new_i_type(0b0010011, 0x7, None, ANDI),
        InstructionFormat::new_i_type(0b0010011, 0x1, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x00), SLLI),
        InstructionFormat::new_i_type(0b0010011, 0x5, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x00), SRLI),
        InstructionFormat::new_i_type(0b0010011, 0x5, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x20), SRAI),
        InstructionFormat::new_i_type(0b0010011, 0x2, None, SLTI),
        InstructionFormat::new_i_type(0b0010011, 0x3, None, SLTIU),

        InstructionFormat::new_i_type(0b0000011, 0x0, None, LB),
        InstructionFormat::new_i_type(0b0000011, 0x1, None, LH),
        InstructionFormat::new_i_type(0b0000011, 0x2, None, LW),
        InstructionFormat::new_i_type(0b0000011, 0x4, None, LBU),
        InstructionFormat::new_i_type(0b0000011, 0x5, None, LHU),

        InstructionFormat::new_s_type(0b0100011, 0x0, SB),
        InstructionFormat::new_s_type(0b0100011, 0x1, SH),
        InstructionFormat::new_s_type(0b0100011, 0x2, SW),

        InstructionFormat::new_b_type(0b1100011, 0x0, BEQ),
        InstructionFormat::new_b_type(0b1100011, 0x1, BNE),
        InstructionFormat::new_b_type(0b1100011, 0x4, BLT),
        InstructionFormat::new_b_type(0b1100011, 0x5, BGE),
        InstructionFormat::new_b_type(0b1100011, 0x6, BLTU),
        InstructionFormat::new_b_type(0b1100011, 0x7, BGEU),

        InstructionFormat::new_j_type(0b1101111,            JAL),
        InstructionFormat::new_i_type(0b1100111, 0x0, None, JALR),

        InstructionFormat::new_u_type(0b0110111, LUI),
        InstructionFormat::new_u_type(0b0010111, AUIPC),

        InstructionFormat::new_i_type(0b1110011, 0x0, Some(|x| x.imm == 0x0), ECALL),
        InstructionFormat::new_i_type(0b1110011, 0x0, Some(|x| x.imm == 0x1), EBREAK),

        // RV64I Base Instruction Set
        InstructionFormat::new_i_type(0b0000011, 0x6, None, LWU),
        InstructionFormat::new_i_type(0b0000011, 0x3, None, LD),

        InstructionFormat::new_s_type(0b0100011, 0x3, SD),
        
        InstructionFormat::new_i_type(0b0011011, 0x0, None, ADDIW),
        InstructionFormat::new_i_type(0b0011011, 0x1, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x00), SLLIW),
        InstructionFormat::new_i_type(0b0011011, 0x5, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x00), SRLIW),
        InstructionFormat::new_i_type(0b0011011, 0x5, Some(|x| get_bits(x.imm as u32, 5, 11) == 0x20), SRAIW),

        InstructionFormat::new_r_type(0b0111011, 0x0, 0x00, ADDW),
        InstructionFormat::new_r_type(0b0111011, 0x0, 0x20, SUBW),
        InstructionFormat::new_r_type(0b0111011, 0x1, 0x00, SLLW),
        InstructionFormat::new_r_type(0b0111011, 0x5, 0x00, SRLW),
        InstructionFormat::new_r_type(0b0111011, 0x5, 0x20, SRAW),

        // M - Multiplication and Division extension
        InstructionFormat::new_r_type(0b0110011, 0x0, 0x01, MUL),
        InstructionFormat::new_r_type(0b0110011, 0x1, 0x01, MULH),
        InstructionFormat::new_r_type(0b0110011, 0x2, 0x01, MULSU),
        InstructionFormat::new_r_type(0b0110011, 0x3, 0x01, MULU),
        InstructionFormat::new_r_type(0b0110011, 0x4, 0x01, DIV),
        InstructionFormat::new_r_type(0b0110011, 0x5, 0x01, DIVU),
        InstructionFormat::new_r_type(0b0110011, 0x6, 0x01, REM),
        InstructionFormat::new_r_type(0b0110011, 0x7, 0x01, REMU),
        InstructionFormat::new_r_type(0b0111011, 0x0, 0x01, MULW),
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
        predicate: Option<fn(&ITypeParams) -> bool>,
        make: fn(ITypeParams) -> Instruction,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RTypeParams {
    pub rs1: u8,
    pub rs2: u8,
    pub rd: u8,
}

impl RTypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rs1: get_bits::<u32>(inst, 15, 19) as u8,
            rs2: get_bits::<u32>(inst, 20, 24) as u8,
            rd: get_bits::<u32>(inst, 7, 11) as u8,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ITypeParams {
    pub rs1: u8,
    pub rd: u8,
    pub imm: i32,
}

impl ITypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rs1: get_bits::<u32>(inst, 15, 19) as u8,
            rd: get_bits::<u32>(inst, 7, 11) as u8,
            imm: sign_extend_32(get_bits(inst, 20, 31), 12),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct STypeParams {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl STypeParams {
    pub fn from(inst: u32) -> Self {
        let imm1 = get_bits(inst, 7, 11);
        let imm2 = get_bits(inst, 25, 31);
        
        Self {
            rs1: get_bits::<u32>(inst, 15, 19) as u8,
            rs2: get_bits::<u32>(inst, 20, 24) as u8,
            imm: sign_extend_32((imm2 << 5) | imm1, 12),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BTypeParams {
    pub rs1: u8,
    pub rs2: u8,
    pub imm: u32,
}

impl BTypeParams {
    pub fn from(inst: u32) -> Self {
        let imm1 = get_bits(inst, 8, 11);
        let imm2 = get_bits(inst, 25, 30);
        let imm3 = get_bits(inst, 7, 7);
        let imm4 = get_bits(inst, 31, 31);

        Self {
            rs1: get_bits::<u32>(inst, 15, 19) as u8,
            rs2: get_bits::<u32>(inst, 20, 24) as u8,
            imm: (imm4 << 11) | (imm3 << 10) | (imm2 << 4) | imm1
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct JTypeParams {
    pub rd: u8,
    pub imm: i32,
}

impl JTypeParams {
    pub fn from(inst: u32) -> Self {
        let imm1 = get_bits(inst, 21, 30);
        let imm2 = get_bits(inst, 20, 20);
        let imm3 = get_bits(inst, 12, 19);
        let imm4 = get_bits(inst, 31, 31);

        Self {
            rd: get_bits::<u32>(inst, 7, 11) as u8,
            imm: sign_extend_32(
                (imm4 << 20) | (imm3 << 12) | (imm2 << 11) | (imm1 << 1),
                 20
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UTypeParams {
    pub rd: u8,
    pub imm: i32,
}

impl UTypeParams {
    pub fn from(inst: u32) -> Self {
        Self {
            rd: get_bits::<u32>(inst, 7, 11) as u8,
            imm: get_bits::<u32>(inst, 12, 31) as i32
        }
    }
}

#[cfg(test)]
mod test {

}