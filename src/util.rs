
/// Gets the bits from n in the interval from start to end. Position 0 refers to LSB.
pub fn get_bits(n: u32, start: u32, end: u32) -> u32 {
    let mask = (1 << (end - start + 1)) - 1;
    (n >> start) & mask
}

pub fn sign_extend(value: u32, bit_width: u32) -> i32 {
    let shift = 32 - bit_width;
    ((value << shift) as i32) >> shift
}