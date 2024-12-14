use cpu::CPU;

pub mod test;
pub mod cpu;

fn main() {
    let mut cpu = CPU::new(512);

    cpu.run();
}

