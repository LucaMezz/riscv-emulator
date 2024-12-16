#![allow(dead_code)]

use crate::isa::Instruction;

use super::memory::{MMU, RegisterFile, registers::Register::*};

#[derive(Clone, PartialEq, Eq)]
pub enum Xlen {
	Bit32,
	Bit64
}

#[derive(Clone, PartialEq, Eq)]
pub enum PrivilegeMode {
	User,
	Supervisor,
	Reserved,
	Machine
}

#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
	Breakpoint,
    EnvironmentCallFromMMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromUMode,
    IllegalInstruction,
    InstructionAccessFault,
    InstructionAddressMisaligned,
    InstructionPageFault,
    LoadAccessFault,
    LoadAddressMisaligned,
    LoadPageFault,
    MachineExternalInterrupt,
    MachineSoftwareInterrupt,
    MachineTimerInterrupt,
    StoreAccessFault,
    StoreAddressMisaligned,
    StorePageFault,
    SupervisorExternalInterrupt,
    SupervisorSoftwareInterrupt,
    SupervisorTimerInterrupt,
    UserExternalInterrupt,
    UserSoftwareInterrupt,
    UserTimerInterrupt
}

pub struct CPU {
    clock: u64,
    xlen: Xlen,
    pc: u64,
    xregs: RegisterFile<u64>,
    fregs: RegisterFile<f64>, 
    mmu: MMU,
}

impl CPU {
    pub fn new(xlen: Xlen) -> Self {
        let mut cpu = Self {
            clock: 0,
            xlen,
            pc: 0,
            xregs: RegisterFile::new(),
            fregs: RegisterFile::new(),
            mmu: MMU::new(),
        };
        cpu.xregs.write(X11, 0x1020);
        cpu
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        } 
    }

    fn read_pc(&self) -> u64 {
        self.pc
    }    

    fn update_pc(&mut self, addr: u64) {
        self.pc = addr;
    }

    fn incr_pc(&mut self, amount: u64) {
        self.pc = self.pc.wrapping_add(amount);
    }

    fn incr_clock(&mut self) {
        self.clock = self.clock.wrapping_add(1);
    }

    fn fetch(&mut self) -> Result<u32, Trap> {
        let word = self.mmu.fetch_word(self.pc);
        if word.is_err() {
            self.incr_pc(4);
        }
        word
    }

    fn decode(&mut self, inst: u32) -> Instruction {
        Instruction::decode(inst)
    }

    fn execute(&mut self, _inst: Instruction) {
        unimplemented!()
    }

    fn handle_exception(&self) {
        unimplemented!()
    }

    fn tick(&mut self) {
        // let raw_inst = self.fetch();
        // let inst: Instruction = self.decode(raw_inst);
        // self.execute(inst);

        self.incr_clock();        
    }

    fn cycle(&mut self) {

    }
}