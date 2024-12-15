use components::CPU;

pub mod test;
pub mod util;
pub mod components;
pub mod isa;

fn main() {
    let mut cpu = CPU::new(512);

    cpu.run();
}

