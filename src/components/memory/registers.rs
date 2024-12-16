
#[repr(u8)]
pub enum Register {
    X0,  // zero
    X1,  // ra
    X2,  // sp
    X3,  // gp
    X4,  // tp
    X5,  // t0
    X6,  // t1
    X7,  // t2
    X8,  // s0 / fp
    X9,  // s1
    X10, // a0
    X11, // a1
    X12, // a2
    X13, // a3
    X14, // a4
    X15, // a5
    X16, // a6
    X17, // a7
    X18, // s2
    X19, // s3
    X20, // s4
    X21, // s5
    X22, // s6
    X23, // s7
    X24, // s8
    X25, // s9
    X26, // s10
    X27, // s11
    X28, // t3
    X29, // t4
    X30, // t5
    X31, // t6

    F0,  // ft0
    F1,  // ft1
    F2,  // ft2
    F3,  // ft3
    F4,  // ft4
    F5,  // ft5
    F6,  // ft6
    F7,  // ft7
    F8,  // fs0
    F9,  // fs1
    F10, // fa0
    F11, // fa1
    F12, // fa2
    F13, // fa3
    F14, // fa4
    F15, // fa5
    F16, // fa6
    F17, // fa7
    F18, // fs2
    F19, // fs3
    F20, // fs4
    F21, // fs5
    F22, // fs6
    F23, // fs7
    F24, // fs8
    F25, // fs9
    F26, // fs10
    F27, // fs11
    F28, // ft8
    F29, // ft9
    F30, // ft10
    F31, // ft11

    PC,
    ICOUNT,
    MISA,

}

impl From<usize> for Register {
    fn from(val: usize) -> Register {
        assert!(val <= 31);

        use Register::*;
        match val {
             0 => X0,
             1 => X1,
             2 => X2,
             3 => X3,
             4 => X4,
             5 => X5,
             6 => X6,
             7 => X7,
             8 => X8,
             9 => X9,
            10 => X10,
            11 => X11,
            12 => X12,
            13 => X13,
            14 => X14,
            15 => X15,
            16 => X16,
            17 => X17,
            18 => X18,
            19 => X19,
            20 => X20,
            21 => X21,
            22 => X22,
            23 => X23,
            24 => X24,
            25 => X25,
            26 => X26,
            27 => X27,
            28 => X28,
            29 => X29,
            30 => X30,
            31 => X31,

            _ => unimplemented!(),
        }
    }
}

pub struct RegisterFile<T> {
    regs: Vec<T>
}

impl<T: Clone + Default> RegisterFile<T> {
    pub fn new() -> Self {
        RegisterFile { 
            regs: vec![T::default(); 31] 
        }
    }

    fn write_num(&mut self, num: u8, data: T) {
        assert!(num < 32);
        self.regs[num as usize] = data;
    }

    fn read_num(&self, num: u8) -> T {
        if num == 0 {
            T::default()
        } else {
            self.regs[num as usize].clone()
        }
    }

    pub fn read(&mut self, reg: Register) {
        self.read_num(reg as u8);
    }

    pub fn write(&mut self, reg: Register, data: T) {
        self.write_num(reg as u8, data);
    }
}