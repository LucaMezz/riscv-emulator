#![allow(dead_code)]

use crate::components::CPU;

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
        }
    }

    pub fn start() {

    }
}

#[cfg(test)]
mod test {

}