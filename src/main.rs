use components::CPU;

pub mod util;
pub mod components;
pub mod isa;
pub mod elf;
pub mod emulator;

fn main() {
    let mut cpu = CPU::new();

    let image = std::fs::read("../emulator_test/binary")
        .expect("no file found");

    let _result = cpu
        .mmu()
        .load_dram_image(image);
    
    cpu.run();
}

