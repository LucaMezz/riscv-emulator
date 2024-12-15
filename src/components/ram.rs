
pub struct RAM {
    size: usize,
    mem: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> Self {
        assert!(size % 4 == 0);
        assert!(size > 0);

        RAM {
            size,
            mem: vec![0; size]
        }
    }

    fn read(&self, base: usize, size: usize) -> u64 {
        unimplemented!()
    }

    fn write(&self, base: usize, size: usize, data: u64) {
        unimplemented!()
    }

    pub fn fetch(&self, pc: u64) -> u64 {
        self.read(pc as usize, 4)
    }

    pub fn read_word(&self, addr: u64) -> u64 {
        self.read(addr as usize, 4)
    }

    pub fn read_half_word(&self, addr: u64) -> u64 {
        self.read(addr as usize, 2)
    }

    pub fn read_byte(&self, addr: u64) -> u64 {
        self.read(addr as usize, 1)
    }

    pub fn write_word(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 4, data)
    }

    pub fn write_half_word(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 2, data as u64)
    }

    pub fn write_byte(&mut self, addr: u64, data: u64) {
        self.write(addr as usize, 1, data as u64)
    }
}