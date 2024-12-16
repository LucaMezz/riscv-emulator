pub mod address;
pub mod image;
pub mod mmu;
pub mod registers;
pub mod dram;
pub mod rom;

pub use self::mmu::MMU;
pub use self::registers::RegisterFile;
pub use self::dram::DRAM;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    Byte = 1,
    HalfWord = 2,
    Word = 4,
    DoubleWord = 8,
}