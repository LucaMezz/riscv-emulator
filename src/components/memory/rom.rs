use crate::components::{bus::{ROM_BASE, ROM_END}, cpu::Trap};

use super::{address::Addressable, image::Imageable, Size};

pub const SIZE: usize = 0xf000;

#[derive(Debug)] 
pub struct ROM {
    rom: Vec<u8>    
}

impl ROM {
    pub fn new() -> Self {
        Self {
            rom: vec![0; SIZE]
        }
    }

    fn read_bytes(&self, addr: u64, size: usize) -> Result<u64, Trap> {
        let index = (addr - ROM_BASE) as usize;
        Ok((self.rom[index..index + size])
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u64) << (i * 8))
            .sum())
    }
}

impl Addressable for ROM {
    fn contains(&self, addr: u64) -> bool {
        match addr {
            ROM_BASE..ROM_END => true,
            _ => false
        }
    }
    
    fn size(&self) -> u64 {
        self.rom.len() as u64
    }

    fn read(&self, addr: u64, size: Size) -> Result<u64, Trap> {
        if self.contains(addr) && self.contains(addr + size.clone() as u64) {
            self.read_bytes(addr, size as usize)
        } else {
            Err(Trap::LoadAccessFault)
        }
    }

    fn write(&mut self, _addr: u64, _size: Size, _data: Vec<u8>) -> Result<(), Trap> {
        Err(Trap::StoreAccessFault)
    }
}

impl Imageable for ROM {
    fn load_image(&mut self, image: Vec<u8>) {
        assert!(image.len() <= self.size() as usize);
        self.rom.splice(..image.len(), image.iter().cloned());
    }

    fn clear_image(&mut self) {
        self.load_image(vec![])
    }

    fn save_image(&self) -> Vec<u8> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::components::memory::image::Imageable;
    use crate::components::memory::{address::Addressable, Size};
    use crate::components::cpu::Trap;

    use super::ROM;

    #[test]
    pub fn it_reads_correctly() {
        let mut rom: ROM = ROM::new();

        rom.load_image(vec![0x81, 0x23, 0x47, 0xa4, 0x7b, 0x00, 0x81, 0x20, 0x45]);

        let result1 = rom.read(0x0000_1001, Size::Word);
        assert!(result1.is_ok_and(|v| v == 0x7b_a4_47_23));

        let result2 = rom.read(0x0000_1004, Size::DoubleWord);
        assert!(result2.is_ok_and(|v| v == 0x00_00_00_45_20_81_00_7b));

        let result3 = rom.read(0x0001_f000, Size::HalfWord);
        assert!(result3.is_err_and(|e| e == Trap::LoadAccessFault));
    }

    #[test]
    pub fn it_fails_to_write() {
        let mut rom: ROM = ROM::new();

        rom.load_image(vec![0x81, 0x23, 0x47, 0xa4, 0x7b, 0x00, 0x81, 0x20, 0x45]);

        let result4 = rom.write(0x0000_f000, Size::Byte, vec![0xaa]);
        assert!(result4.is_err_and(|e| e == Trap::StoreAccessFault));

        let result5 = rom.write(0x0000_1003, Size::Byte, vec![0xff]);
        assert!(result5.is_err_and(|e| e == Trap::StoreAccessFault));
        let read5 = rom.read(0x0000_1000, Size::Word);
        assert!(read5.is_ok_and(|v| v == 0xa4_47_23_81));
    }
}