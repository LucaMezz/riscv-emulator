
pub fn get_bits(n: u32, start: u32, end: u32) -> u32 {
    let mask = (1 << (end - start + 1)) - 1;
    (n >> start) & mask
}