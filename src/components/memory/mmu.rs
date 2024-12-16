#![allow(dead_code)]

use crate::components::{bus::{DRAM_BASE, ROM_BASE, ROM_END}, cpu::{PrivilegeMode, Trap, Xlen}, Bus};

use super::{address::Addressable, Size};

pub struct MMU {
    bus: Bus,
    xlen: Xlen,
    pmode: PrivilegeMode
}

impl MMU {
    pub fn new() -> Self {
        Self { 
            bus: Bus::new(),
            xlen: Xlen::Bit64,
            pmode: PrivilegeMode::Machine,
        }
    }

    /// Updates the privilege mode of the MMU.
    fn set_privilege_mode(&mut self, mode: PrivilegeMode) {
        self.pmode = mode;
    }

    /// Gets the effective address from a given address. In the case that the MMU is running in
    /// 32-bit mode, then the upper 32 bits of the address are zeroed. 
    fn get_effective_address(&self, addr: u64) -> u64 {
		match self.xlen {
			Xlen::Bit32 => addr & 0xffffffff,
			Xlen::Bit64 => addr
		}
	}

    /// Determines if the given virtual address points to a valid physical address on a certain 
    /// device or not.
    /// 
    /// Attempts to translate the virtual address into a physical address, and then determines if
    /// the physical address is mapped to any device.
    fn validate_address(&mut self, vaddr: u64) -> Result<bool, Trap> {
        let eaddr = self.get_effective_address(vaddr);
        let paddr = self.translate(eaddr);
        match paddr >= DRAM_BASE {
            true => Ok(true),
            false => match paddr {
                ROM_BASE..ROM_END => Ok(true),
                _ => Ok(false),
            }
        }
    }

    /// Translates a virtual address into a physical address. If paging is disabled, or if the 
    /// privilege is machine mode, then the virtual address is the same as the physical address.    
    fn translate(&self, vaddr: u64) -> u64 {
        match self.pmode {
            PrivilegeMode::Machine => vaddr,
            _ => unimplemented!(),
        }
    }

    /// Loads byte(s) from a device which is determined by the virtual address.
    /// 
    /// Translates the virtual address into a physical address before attempting to read from
    /// memory. If paging is disabled, or if the privilege is machine mode, then the virtual 
    /// address is the same as the physical address. 
    pub fn load(&self, vaddr: u64, size: Size) -> Result<u64, Trap> {
        let eaddr = self.get_effective_address(vaddr);
        let paddr = self.translate(eaddr);
        self.bus.read(paddr, size)
    }

    /// Stores byte(s) from a device which is determined by the virtual address.
    /// 
    /// Translates the virtual address into a physical address before attempting to read from
    /// memory. If paging is disabled, or if the privilege is machine mode, then the virtual 
    /// address is the same as the physical address. 
    pub fn store(&mut self, vaddr: u64, size: Size, data: Vec<u8>) -> Result<(), Trap> {
        let eaddr = self.get_effective_address(vaddr);
        let paddr = self.translate(eaddr);
        self.bus.write(paddr, size, data)
    }
}

#[cfg(test)]
mod test {

}