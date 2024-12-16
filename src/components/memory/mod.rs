pub mod address;
pub mod image;
pub mod mmu;
pub mod registers;
pub mod dram;
pub mod rom;

pub use mmu::MMU;
pub use registers::RegisterFile;
pub use dram::DRAM;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}