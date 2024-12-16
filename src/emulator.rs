#![allow(dead_code)]

use crate::components::{cpu::Xlen, CPU};

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn new(xlen: Xlen) -> Self {
        Self {
            cpu: CPU::new(xlen),
        }
    }

    pub fn start() {

    }
}