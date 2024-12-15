#![allow(dead_code)]

use super::{RegisterFile, RAM};

pub struct CPU {
    registers: RegisterFile,
    memory: RAM,
}

impl CPU {
    pub fn new(memory: usize) -> Self {
        CPU { registers: RegisterFile::new(), memory: RAM::new(memory) }
    }

    pub fn run(&mut self) {
        loop {
            
        }
    }
}