pub fn map(x: u16, y: u16) -> f64 {
    (((x as u64) << 16) | y as u64) as f64 / u32::MAX as f64
}

#[test]
pub fn test_map() {
    dbg!(map(0, 0));
    dbg!(map(15, 15));
    dbg!(map(u16::MAX, u16::MAX));
}