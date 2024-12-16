use super::{cpu::Trap, memory::{address::Addressable, rom::ROM, Size, DRAM}};

/// The address which the ROM starts.
pub const ROM_BASE: u64 = 0x1000;
/// The address which the ROM ends.
pub const ROM_END: u64 = ROM_BASE + 0xf000;

/// The address which DRAM starts.
pub const DRAM_BASE: u64 = 0x8000_0000;

#[derive(Debug)]
pub struct Bus {
    rom: ROM,
    dram: DRAM,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            rom: ROM::new(),
            dram: DRAM::new(1024 * 1024 * 1024),
        }
    }

    pub fn rom(&mut self) -> &mut ROM {
        &mut self.rom
    }

    pub fn dram(&mut self) -> &mut DRAM {
        &mut self.dram
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u64, size: Size) -> Result<u64, Trap> {
        match addr {
            ROM_BASE..ROM_END => self.rom.read(addr, size),
            _ => self.dram.read(addr, size)
        }
    }

    fn write(&mut self, addr: u64, size: Size, data: Vec<u8>) -> Result<(), Trap> {
        match addr {
            ROM_BASE..ROM_END => self.rom.write(addr, size, data),
            _ => self.dram.write(addr, size, data),
        }
    }

    fn size(&self) -> u64 {
        2^64-1
    }

    fn contains(&self, addr: u64) -> bool {
        self.rom.contains(addr) || self.dram.contains(addr)
    }
}

#[cfg(test)]
mod test {
    use crate::components::{cpu::Trap, memory::{address::Addressable, Size}};

    use super::Bus;


    #[test]
    fn it_fails_for_invalid_addresses() {
        let bus = Bus::new();

        let result = bus.read(0x0000_0539, Size::DoubleWord);
        assert!(result.is_err_and(|e| e == Trap::LoadAccessFault));

        let result2 = bus.read(0x0000_102c, Size::Byte);
        assert!(result2.is_ok_and(|v| v == 0));
    }

    #[test]
    fn it_reads_from_the_correct_device() {

    }
}