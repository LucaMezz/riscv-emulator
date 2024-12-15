
pub struct RegisterFile {
    regs: Vec<u64>
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile { 
            regs: vec![0; 31] 
        }
    }

    pub fn write(&mut self, num: u8, data: u64) {
        assert!(num < 32);
        self.regs[num as usize] = data;
    }

    pub fn read(&self, num: u8) -> u64 {
        if num == 0 {
            0
        } else {
            self.regs[num as usize]
        }
    }
}