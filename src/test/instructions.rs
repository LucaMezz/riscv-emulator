#![allow(unused_imports)]

use crate::isa::{format::{BTypeParams, ITypeParams, RTypeParams, STypeParams}, Instruction};

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
        imm: 12,
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