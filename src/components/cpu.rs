#![allow(dead_code)]

use crate::isa::Instruction;

use super::RegisterFile;

pub struct CPU {
    clock: u64,
    pc: u64,
    registers: RegisterFile,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            clock: 0,
            pc: 0,
            registers: RegisterFile::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        } 
    }

    fn read_pc(&self) -> u64 {
        self.pc
    }    

    fn incr_clock(&mut self) {
        self.clock = self.clock.wrapping_add(1);
    }

    fn fetch(&mut self) -> u64 {
        unimplemented!()
    }

    fn decode(&mut self, inst: u64) -> Instruction {
        unimplemented!()
    }

    fn execute(&mut self, inst: Instruction) {

    }

    fn tick(&mut self) {
        let raw_inst = self.fetch();
        let inst: Instruction = self.decode(raw_inst);
        self.execute(inst);

        self.incr_clock();        
    }
}