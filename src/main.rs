use cpu::{Instruction, CPU};

pub mod test;
pub mod cpu;

fn main() {
    let inst = Instruction::decode(0b00000000000000000000011110110011);
    println!("Decoded: {:#?}", inst);

    let mut cpu = CPU::new();

    cpu.run();
}

