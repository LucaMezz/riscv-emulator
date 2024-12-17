use num_traits::PrimInt;


/// Gets the bits from n in the interval from start to end. Position 0 refers to LSB.
pub fn get_bits<T>(n: T, start: usize, end: usize) -> T
where
    T: Copy + PrimInt,
{
    let mask = (T::one() << end - start + 1) - T::one();
    (n >> start) & mask
}

pub fn sign_extend_32(value: u32, bit_width: u8) -> i32 {
    let shift = 32 - bit_width;
    ((value << shift) as i32) >> shift
}

pub fn sign_extend_64(value: u64, bit_width: u8) -> i64 {
    let shift = 64 - bit_width;
    ((value << shift) as i64) >> shift
}

pub fn unsigned_32(value: u64) -> u64 {
    get_bits(value, 0, 31)
}

#[cfg(test)]
mod test {
    use crate::util::unsigned_32;

    #[test]
    fn it_finds_unsigned_32_correctly() {
        let num: u64 = 0x82a7_28e5_c984_d1f4;
        assert_eq!(unsigned_32(num), 0xc984_d1f4);
    }
}