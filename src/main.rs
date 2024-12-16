use components::{cpu::Xlen, CPU};

pub mod util;
pub mod components;
pub mod isa;
pub mod elf;

fn main() {
    let mut cpu = CPU::new(Xlen::Bit64);

    cpu.run();
}

