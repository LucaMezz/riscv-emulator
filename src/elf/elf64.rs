//! Documentation source: https://refspecs.linuxfoundation.org/elf/elf.pdf

#![allow(dead_code)]

pub use super::EI_NIDENT;
use num_enum::TryFromPrimitive;

type Half = u16;
type Word = u64;
type Sword = i64;
type Addr = u64;
type Off = u64;

#[derive(Debug)]
pub struct Header {
    /// The initial bytes mark the file as an object file and provide machine-independent data with 
    /// which to decode and interpret the file's contents.
    pub ident: [u8; EI_NIDENT],

    pub typ: Type,
    pub machine: Machine,
    pub version: Version,

    /// The virtual address to which the system first transfers control, thus starting the process. 
    /// If the file has no associated entry point, this member holds zero.
    pub entry: Addr,

    /// Program header table's file offset in bytes. If the file has no program header table, this 
    /// member holds zero.
    pub phoff: Off,

    /// Section header table's file offset in bytes. If the file has no section header table, this 
    /// member holds zero.
    pub shoff: Off,

    /// Processor-specific flags associated with the file.
    pub flags: Word,

    /// ELF header's size in bytes. 
    pub ehsize: Half,

    /// The size in bytes of one entry in the file's program header table; all entries are the same
    /// size.
    pub phentsize: Half,

    /// The number of entries in the program header table.
    pub phnum: Half,

    /// A section header's size in bytes.
    pub shentsize: Half,

    /// The number of entries in the section header table.
    pub shnum: Half,

    /// The section header table index of the entry associated with the section name string table.
    pub shstrndx: Half,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum Identification {
    /// First 4 bytes hold a magic number identifying the file as an ELF object file.
    Mag0,
    Mag1,
    Mag2,
    Mag3,

    /// File class or capacity
    Class,

    /// The data encoding of the processor-specific data in the object file.
    Data,

    /// ELF header version number. 
    Version,

    /// marks the beginning of the unused bytes.
    Pad,

    /// Stores the number of bites in this identifier.
    NIdent = 16
}

/// Identifies the file's class or capacity. Indicates if the file supports machines with 32-bit or
/// 64-bit objects.
#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Class {
    /// Invalid class.
    None,

    /// Supports 32-bit objects.
    Class32,

    /// Supports 64-bit objects.
    Class64,
}

/// The data encoding of the processor-specific data in the object file.
#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Data {
    None,
    LSB2,
    MSB2,
}

/// Identifies the object file type.
#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum Type {
    None,
    Relocatable,
    Executable,
    Dynamic,
    Core,
    LoProc = 0xff00,
    HiProc = 0xffff,
}

/// Represents the architecture of a machine.
#[derive(Debug, TryFromPrimitive)]
#[repr(u16)]
pub enum Machine {
    NONE = 0x00,
    M32 = 0x01,
    SPARC = 0x02,
    I386 = 0x03,
    M68K = 0x04,
    M88K = 0x05,
    IAMCU = 0x06,
    I860 = 0x07,
    MIPS = 0x08,
    PARISC = 0x0F,
    SPARC32PLUS = 0x13,
    PPC = 0x14,
    PPC64 = 0x15,
    S390 = 0x16,
    ARM = 0x28,
    SH = 0x2A,
    IA64 = 0x32,
    X86_64 = 0x3E,
    V800 = 0x18,
    FR20 = 0x19,
    RH32 = 0x1A,
    RCE = 0x1B,
    V850 = 0x24,
    M32R = 0x25,
    MN10300 = 0x26,
    MN10200 = 0x27,
    ARC = 0x2C,
    H8_300 = 0x2E,
    AVR = 0xB9,
    AARCH64 = 0xB7,
    RISCV = 0xF3,
    BPF = 0xF7,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum Version {
    /// Invalid version
    None,
    /// Current version
    Current,
}

#[derive(Debug)]
pub struct SectionHeader {

    /// the name of the section. Its value is an index into the section header string table section,
    /// giving the location of a null-terminated string.
    pub name: Word,

    /// categorizes the section's contents and semantics.
    pub section_type: SectionType,

    /// Sections support 1-bit flags that describe miscellaneous attributes.
    pub flags: Word,

    /// If the section will appear in the memory image of a process, this member gives the address 
    /// at which the section's first byte should reside. Otherwise, the member contains 0.
    pub addr: Addr,

    /// Byte offset from the beginning of the file to the first byte in the section.
    pub offset: Off,

    /// Section size in bytes.
    pub size: Word,

    /// A section header table index link, whose interpretation depends 
    /// on the section type.
    pub link: Word,

    /// Extra information, whose interpretation depends on the section type.
    pub info: Word,

    /// Address alignment constraints. The value of addr must be congruent to 0 modulo the value of 
    /// addralign.
    pub addralign: Word,

    /// Some sections hold a table of fixed-size entries, such as a symbol table. For such a 
    /// section, this member gives the size in bytes of each entry. 
    pub entsize: Word,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u64)]
pub enum SectionType {
    /// Marks the section header as inactive; it does not have an associated section. Other members
    /// of the section header have undefined values.
    Null,

    /// Information defined by the program, whose format and meaning are determined solely by the 
    /// program.
    ProgBits,

    /// Symbol table.
    SymTab,

    /// String table.
    StrTab,

    /// Relocation entries with explicit addends.
    Rela,

    /// Symbol hash table. 
    Hash,

    /// Information for dynamic linking.
    Dynamic,

    /// Information that marks the file in some way. 
    Note,

    /// A section of this type occupies no space in the file but otherwise resembles ProgBits.
    NoBits,

    /// Relocation entries without explicit addends.
    Rel,

    /// This section type is reserved but has unspecified semantics.
    ShLib,

    /// Symbol table.
    DynSym,

    /// Values in this inclusive range are reserved for processor-specific semantics
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,

    /// This value specifies the lower bound of the range of indexes reserved for application 
    /// programs.
    LoUser = 0x80000000,

    /// This value specifies the upper bound of the range of indexes reserved for application 
    /// programs.
    HiUser = 0xffffffff,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u32)]
pub enum SectionAttributeFlags {
    /// The section contains data that should be writable during process execution.
    Write = 0x1,

    /// The section occupies memory during process execution. Some control sections do not reside in
    /// the memory image of an object file; this attribute is off for those sections.
    Alloc = 0x2,

    /// The section contains executable machine instructions
    ExecInstr = 0x4,

    /// All bits included in this mask are reserved for processor-specific semantics
    MaskProc = 0xf0000000,
}

#[derive(Debug)]
pub struct Symbol {
    /// An index into the object file's symbol string table, which holds the character 
    /// representations of the symbol names.
    name: Word,

    /// The value of the associated symbol. Depending on the context, this may be an absolute value, 
    /// an address, and so on.
    value: Addr,

    /// The syze of the symbol. Different meanings depending on the context.
    size: Word,

    /// This member specifies the symbol's type and binding attributes.
    info: u8,

    /// This member currently holds 0 and has no defined meaning.
    other: u8,


    shndx: Half,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum SymbolBinding {
    /// Local symbols are not visible outside the object file containing their definition. Local 
    /// symbols of the same name may exist in multiple files without interfering with each other.
    Local,

    /// Global symbols are visible to all object files being combined. One file's definition of a 
    /// global symbol will satisfy another file's undefined reference to the same global symbol.
    Global,

    /// Weak symbols resemble global symbols, but their definitions have lower
    /// precedence.
    Weak,

    /// Values in this inclusive range are reserved for processor-specific semantics.
    LoProc = 13,
    HiProc = 15,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum SymbolType {
    /// The symbol's type is not specified.
    NoType,

    /// The symbol is associated with a data object, such as a variable, an array, and so on.
    Object,

    /// The symbol is associated with a function or other executable code.
    Func,

    /// The symbol is associated with a section. Symbol table entries of this type exist primarily 
    /// for relocation and normally have SymbolBinding::Local.
    Section,

    /// A file symbol has STB_LOCAL binding, its section index is SHN_ABS, and
    /// it precedes the other STB_LOCAL symbols for the file, if it is present.
    File,

    LoProc = 13,
    HiProc = 15,
}

#[derive(Debug)]
pub struct Relocation {
    /// This member gives the location at which to apply the relocation action. For a relocatable 
    /// file, the value is the byte offset from the beginning of the section to the storage unit 
    /// affected by the relocation. For an executable file or a shared object, the value is the 
    /// virtual address of the storage unit affected by the relocation.
    offset: Addr,

    /// This member gives both the symbol table index with respect to which the relocation must be 
    /// made, and the type of relocation to apply. For example, a call instruction's relocation 
    /// entry would hold the symbol table index of the function being called. If the index is 
    /// Undefined, the undefined symbol index, the relocation uses 0 as the "symbol value." 
    /// Relocation types are processor-specific; descriptions of their behavior appear in the 
    /// processor supplement.
    info: Word,
}

#[derive(Debug)]
pub struct RelocationAddend {
    /// This member gives the location at which to apply the relocation action. For a relocatable 
    /// file, the value is the byte offset from the beginning of the section to the storage unit 
    /// affected by the relocation. For an executable file or a shared object, the value is the 
    /// virtual address of the storage unit affected by the relocation.
    offset: Addr,

    /// This member gives both the symbol table index with respect to which the relocation must be 
    /// made, and the type of relocation to apply. For example, a call instruction's relocation 
    /// entry would hold the symbol table index of the function being called. If the index is 
    /// Undefined, the undefined symbol index, the relocation uses 0 as the "symbol value." 
    /// Relocation types are processor-specific; descriptions of their behavior appear in the 
    /// processor supplement.
    info: Word,

    /// This member specifies a constant addend used to compute the value to be stored into the 
    /// relocatable field.
    addend: Sword,
}

#[derive(Debug)]
struct ProgramHeader {
    /// This member tells what kind of segment this array element describes or how to interpret the
    /// array element's information. Type values and their meanings appear below.
    typ: Word,

    /// This member gives the offset from the beginning of the file at which the first byte of the 
    /// segment resides.
    offset: Off,

    /// This member gives the virtual address at which the first byte of the segment resides in 
    /// memory.
    vaddr: Addr,

    /// On systems for which physical addressing is relevant, this member is reserved for the 
    /// segment's physical address.
    paddr: Addr,

    /// This member gives the number of bytes in the file image of the segment; it may be zero.
    filesz: Word,

    /// This member gives the number of bytes in the memory image of the segment; it may be zero.
    memsz: Word,

    /// This member gives flags relevant to the segment. Defined flag values appear below.
    flags: Word,

    /// Loadable process segments must have congruent values for vaddr and offset, modulo the 
    /// page size.This member gives the value to which the segments are aligned in memory and in the
    /// file. Values 0 and 1 mean that no alignment is required. Otherwise, align should be a 
    /// positive, integral power of 2, and addr should equal offset, modulo align.
    align: Word,
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u64)]
enum SegmentType {
    /// The array element is unused.
    Null,

    /// The array element specifies a loadable segment.
    Load,

    /// The array element specifies dynamic linking information.
    Dynamic,

    /// The array element specifies the location and size of a null-terminated path name to invoke 
    /// as an interpreter.
    Interp,

    /// The array element specifies the location and size of auxiliary information.
    Note,

    /// This segment type is reserved but has unspecified semantics.
    ShLib,

    /// The array element, if present, specifies the location and size of the program header table 
    /// itself, both in the file and in the memory image of the program. This segment type may not
    /// occur more than once in a file. Moreover, it may occur only if the program header table is 
    /// part of the memory image of the program. If it is present, it must precede any loadable 
    /// segment entry.
    Phdr,

    /// Values in this inclusive range are reserved for processor-specific semantics.
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,
}