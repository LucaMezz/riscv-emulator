use components::CPU;

pub mod util;
pub mod components;
pub mod isa;
pub mod elf;
pub mod emulator;

fn main() {
    let mut cpu = CPU::new();

    cpu.run();
}

