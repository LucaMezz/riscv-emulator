use crate::components::cpu::Trap;


pub struct MMU {

}

impl MMU {
    pub fn new() -> Self {
        Self { }
    }

    pub fn fetch_word(&self, _pc: u64) -> Result<u32, Trap> {
        unimplemented!()
    }
}