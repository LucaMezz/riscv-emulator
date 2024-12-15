use components::CPU;

pub mod util;
pub mod components;
pub mod isa;

fn main() {
    let mut cpu = CPU::new();

    cpu.run();
}

