#![allow(dead_code)]

use crate::components::cpu::Trap;

use super::Size;

/// Addressable trait provides functions to read / write bytes in a given 64-bit address space
pub trait Addressable {
    
    /// Indicates if the given address belongs to the address space.
    fn contains(&self, addr: u64) -> bool;

    /// The number of addresses in the address space
    fn size(&self) -> u64;

    /// Returns the value stored at the given address.
    fn read(&self, addr: u64, size: Size) -> Result<u64, Trap>;

    /// Attempts to store the given value at the given address.
    fn write(&mut self, addr: u64, size: Size, data: Vec<u8>) -> Result<(), Trap>;
}