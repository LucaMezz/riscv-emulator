#![allow(dead_code)]

use crate::components::{bus::DRAM_BASE, cpu::Trap};

use super::{address::Addressable, image::ImageStorage, Size};

#[derive(Debug)]
pub struct DRAM {
    dram: Vec<u8>,
    code_len: u64,
}

impl DRAM {
    pub fn new(size: usize) -> Self {
        Self {
            dram: vec![0; size],
            code_len: 0,
        }
    }

    fn write_bytes(&mut self, addr: u64, size: u8, data: Vec<u8>) -> Result<(), Trap> {
        let index = (addr - DRAM_BASE) as usize;
    
        for (i, &byte) in data.iter().enumerate().take(size as usize) {
            self.dram[index + i] = byte;
        }

        Ok(())
    }

    fn read_bytes(&self, addr: u64, size: usize) -> Result<u64, Trap> {
        let index = (addr - DRAM_BASE) as usize;
        Ok((self.dram[index..index + size])
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u64) << (i * 8))
            .sum())
    }
}

impl Addressable for DRAM {
    fn read(&self, addr: u64, size: Size) -> Result<u64, Trap> {
        if self.contains(addr) && self.contains(addr + size.clone() as u64) {
            self.read_bytes(addr, size as usize)
        } else {
            Err(Trap::LoadAccessFault)
        }
    }

    fn write(&mut self, addr: u64, size: Size, data: Vec<u8>) -> Result<(), Trap> {
        assert!(data.len() == size.clone() as usize);
        if self.contains(addr) && self.contains(addr + size.clone() as u64) {
            self.write_bytes(addr, size as u8, data)
        } else {
            Err(Trap::StoreAccessFault)
        }
    }

    fn size(&self) -> u64 {
        self.dram.len() as u64
    }

    fn contains(&self, addr: u64) -> bool {
        addr >= DRAM_BASE && addr < DRAM_BASE + self.size()
    }
}

impl ImageStorage for DRAM {
    fn load_image(&mut self, image: Vec<u8>) {
        self.code_len = image.len() as u64;
        self.dram.splice(..image.len(), image.iter().cloned());
    }

    fn clear_image(&mut self) {
        self.load_image(vec![]);
    }

    fn save_image(&self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::components::memory::image::ImageStorage;
    use crate::components::memory::{address::Addressable, Size};
    use crate::components::cpu::Trap;

    use super::DRAM;

    #[test]
    pub fn it_reads_correctly() {
        let size = 1024;
        let mut dram: DRAM = DRAM::new(size);

        dram.load_image(vec![0x81, 0x23, 0x47, 0xa4, 0x7b, 0x00, 0x81, 0x20, 0x45]);

        let result1 = dram.read(0x8000_0001, Size::Word);
        assert!(result1.is_ok_and(|v| v == 0x7b_a4_47_23));

        let result2 = dram.read(0x8000_0004, Size::DoubleWord);
        assert!(result2.is_ok_and(|v| v == 0x00_00_00_45_20_81_00_7b));

        let result3 = dram.read(0x5000_3492, Size::HalfWord);
        assert!(result3.is_err_and(|e| e == Trap::LoadAccessFault));
        
        
    }

    #[test]
    pub fn it_writes_correctly() {
        let size = 1024;
        let mut dram: DRAM = DRAM::new(size);

        dram.load_image(vec![0x81, 0x23, 0x47, 0xa4, 0x7b, 0x00, 0x81, 0x20, 0x45]);

        let result4 = dram.write(0x8000_1024, Size::Byte, vec![0xaa]);
        assert!(result4.is_err_and(|e| e == Trap::StoreAccessFault));

        let result5 = dram.write(0x8000_0003, Size::Byte, vec![0xff]);
        assert!(result5.is_ok());
        let read5 = dram.read(0x8000_0000, Size::Word);
        assert!(read5.is_ok_and(|v| v == 0xff_47_23_81));
    }
}