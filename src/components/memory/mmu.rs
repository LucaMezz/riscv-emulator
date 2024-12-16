#![allow(dead_code)]

use crate::components::{cpu::Trap, Bus};

pub struct MMU {
    bus: Bus,
}

impl MMU {
    pub fn new() -> Self {
        Self { 
            bus: Bus::new()
        }
    }

    pub fn fetch_word(&mut self, _pc: u64) -> Result<u32, Trap> {
        unimplemented!();
    }
}