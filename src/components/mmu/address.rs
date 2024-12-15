
/// Addressable trait provides functions to read / write bytes in a given 64-bit address space
trait Addressable {
    
    /// Indicates if the given address belongs to the address space.
    fn contains(addr: u64) -> bool;

    /// Returns the value stored at the given address.
    fn read(addr: u64) -> u64;

    /// Attempts to store the given value at the given address.
    fn write(addr: u64, value: u64);
}