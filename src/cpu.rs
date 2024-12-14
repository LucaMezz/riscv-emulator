#![allow(dead_code)]

use derive_new::new;
use lazy_static::lazy_static;

macro_rules! i_instruction {
    ($name:ident, $opcode:expr, $funct3:expr, $funct7:expr) => {
        InstructionFormat::new_i_type($opcode, $funct3, $funct7, |i| Instruction::$name(ITypeParams::from(i, true)))
    };
}

macro_rules! r_instruction {
    ($name:ident, $opcode:expr, $funct3:expr, $funct7:expr) => {
        InstructionFormat::new_r_type($opcode, $funct3, $funct7, |i| Instruction::$name(RTypeParams::from(i)))
    };
}

macro_rules! s_instruction {
    ($name:ident, $opcode:expr, $funct3:expr) => {
        InstructionFormat::new_s_type($opcode, $funct3, |i| Instruction::$name(STypeParams::from(i)))
    };
}

macro_rules! b_instruction {
    ($name:ident, $opcode:expr, $funct3:expr) => {
        InstructionFormat::new_b_type($opcode, $funct3, |i| Instruction::$name(BTypeParams::from(i)))
    };
}

macro_rules! j_instruction {
    ($name:ident, $opcode:expr) => {
        InstructionFormat::new_j_type($opcode, |i| Instruction::$name(JTypeParams::from(i)))
    };
}

macro_rules! u_instruction {
    ($name:ident, $opcode:expr) => {
        InstructionFormat::new_u_type($opcode, |i| Instruction::$name(UTypeParams::from(i)))
    };
}

fn get_bits(n: u32, start: u32, end: u32) -> u32 {
    let mask = (1 << (end - start + 1)) - 1;
    (n >> start) & mask
}

#[derive(new)]
enum InstructionFormat {
    RType {
        opcode: u32,
        funct3: u32,
        funct7: u32,
        from: fn(u32) -> Instruction
    },
    IType {
        opcode: u32,
        funct3: u32,
        funct7: Option<u32>,
        from: fn(u32) -> Instruction
    },
    SType {
        opcode: u32,
        funct3: u32,
        from: fn(u32) -> Instruction
    },
    BType {
        opcode: u32,
        funct3: u32,
        from: fn(u32) -> Instruction
    },
    UType {
        opcode: u32,
        from: fn(u32) -> Instruction
    },
    JType {
        opcode: u32,
        from: fn(u32) -> Instruction
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
    pub fn from(inst: u32, short_imm: bool) -> Self {
        if short_imm {
            Self {
                rs1: get_bits(inst, 15, 19),
                rd: get_bits(inst, 7, 11),
                imm: get_bits(inst, 20, 24),
            }
        } else {
            Self {
                rs1: get_bits(inst, 15, 19),
                rd: get_bits(inst, 7, 11),
                imm: get_bits(inst, 20, 31),
            }
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

lazy_static! {
    static ref patterns: Vec<InstructionFormat> = vec![
        r_instruction!(ADD,  0b0110011, 0x0, 0x00),
        r_instruction!(SUB,  0b0110011, 0x0, 0x20),
        r_instruction!(XOR,  0b0110011, 0x4, 0x00),
        r_instruction!(OR,   0b0110011, 0x6, 0x00),
        r_instruction!(AND,  0b0110011, 0x7, 0x00),
        r_instruction!(SLL,  0b0110011, 0x1, 0x00),
        r_instruction!(SRL,  0b0110011, 0x5, 0x00),
        r_instruction!(SRA,  0b0110011, 0x5, 0x20),
        r_instruction!(SLT,  0b0110011, 0x2, 0x00),
        r_instruction!(SLTU, 0b0110011, 0x3, 0x00),

        r_instruction!(MUL,   0b0110011, 0x0, 0x01),
        r_instruction!(MULH,  0b0110011, 0x1, 0x01),
        r_instruction!(MULSU, 0b0110011, 0x2, 0x01),
        r_instruction!(MULU,  0b0110011, 0x3, 0x01),
        r_instruction!(DIV,   0b0110011, 0x4, 0x01),
        r_instruction!(DIVU,  0b0110011, 0x5, 0x01),
        r_instruction!(REM,   0b0110011, 0x6, 0x01),
        r_instruction!(REMU,  0b0110011, 0x7, 0x01),

        i_instruction!(ADDI,  0b0010011, 0x0, None),
        i_instruction!(XORI,  0b0010011, 0x4, None),
        i_instruction!(ORI,   0b0010011, 0x6, None),
        i_instruction!(ANDI,  0b0010011, 0x7, None),
        i_instruction!(SLLI,  0b0010011, 0x1, Some(0x00)),
        i_instruction!(SRLI,  0b0010011, 0x5, Some(0x00)),
        i_instruction!(SRAI,  0b0010011, 0x5, Some(0x20)),
        i_instruction!(SLTI,  0b0010011, 0x2, None),
        i_instruction!(SLTIU, 0b0010011, 0x3, None),

        i_instruction!(LB,  0b0000011, 0x0, None),
        i_instruction!(LH,  0b0000011, 0x1, None),
        i_instruction!(LW,  0b0000011, 0x2, None),
        i_instruction!(LBU, 0b0000011, 0x4, None),
        i_instruction!(LHU, 0b0000011, 0x5, None),

        s_instruction!(SB, 0b0100011, 0x0),
        s_instruction!(SH, 0b0100011, 0x1),
        s_instruction!(SW, 0b0100011, 0x2),

        b_instruction!(BEQ,  0b1100011, 0x0),
        b_instruction!(BNE,  0b1100011, 0x1),
        b_instruction!(BLT,  0b1100011, 0x4),
        b_instruction!(BGE,  0b1100011, 0x5),
        b_instruction!(BLTU, 0b1100011, 0x6),
        b_instruction!(BGEU, 0b1100011, 0x7),

        j_instruction!(JAL,  0b1101111),
        i_instruction!(JALR, 0b1100111, 0x0, None),

        u_instruction!(LUI,   0b0110111),
        u_instruction!(AUIPC, 0b0010111),

        i_instruction!(ECALL,  0b1110011, 0x0, None),
        i_instruction!(EBREAK, 0b1110011, 0x0, None)
    ];
}


impl Instruction {
    pub fn decode(inst: u32) -> Self {
        for pattern in patterns.iter() {
            let inst_opcode = get_bits(inst, 0, 6);
            let inst_funct3 = get_bits(inst, 12, 14);
            let inst_funct7 = get_bits(inst, 25, 31);

            match pattern {
                InstructionFormat::RType { opcode, funct3, funct7, from } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 && *funct7 == inst_funct7 {
                        return from(inst);
                    }
                },
                InstructionFormat::IType { opcode, funct3, funct7, from } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        if funct7.is_some_and(|x| x == inst_funct7) {
                            return from(inst);
                        }
                    }
                },
                InstructionFormat::SType { opcode, funct3, from } | 
                InstructionFormat::BType { opcode, funct3, from } => {
                    if *opcode == inst_opcode && *funct3 == inst_funct3 {
                        return from(inst);
                    }
                }
                InstructionFormat::JType { opcode, from } |
                InstructionFormat::UType { opcode, from } => {
                    if *opcode == inst_opcode {
                        return from(inst);
                    }
                }
            }
        }

        Instruction::UNDEF
    }
}

pub struct DRAM {
    size: usize,
    mem: Vec<u8>,
}

impl DRAM {
    pub fn new(size: usize) -> Self {
        assert!(size % 4 == 0);
        assert!(size > 0);

        DRAM {
            size,
            mem: vec![0; size]
        }
    }

    fn read(&self, base: usize, size: usize) -> u64 {
        unimplemented!()
    }

    fn write(&self, base: usize, size: usize, data: u64) {
        unimplemented!()
    }

    pub fn fetch(&self, pc: u64) -> u64 {
        self.read(pc as usize, 4)
    }

    pub fn read_word(&self, addr: u64) -> u64 {
        self.read(addr as usize, 4)
    }

    pub fn read_half_word(&self, addr: u64) -> u64 {
        self.read(addr as usize, 2)
    }

    pub fn read_byte(&self, addr: u64) -> u64 {
        self.read(addr as usize, 1)
    }

    pub fn write_word(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 4, data)
    }

    pub fn write_half_word(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 2, data as u64)
    }

    pub fn write_byte(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 1, data as u64)
    }
}

pub struct RegisterFile {
    regs: Vec<u64>
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile { 
            regs: vec![0; 31] 
        }
    }

    pub fn write(&mut self, num: u8, data: u64) {
        assert!(num < 32);
        self.regs[num as usize] = data;
    }

    pub fn read(&self, num: u8) -> u64 {
        if num == 0 {
            0
        } else {
            self.regs[num as usize]
        }
    }
}

pub struct CPU {
    registers: RegisterFile,
    memory: DRAM,
}

impl CPU {
    pub fn new(memory: usize) -> Self {
        CPU { registers: RegisterFile::new(), memory: DRAM::new(memory) }
    }

    pub fn run(&mut self) {
        loop {
            
        }
    }
}