use crate::ex10::map;
use crate::dresult;
use crate::utils::ex;

pub fn reverse_map(n: f64) -> (u16, u16) {
    let t = (n * (u32::MAX as f64)) as u64;
    (((t >> 16) & 0xFFFF) as u16, (t & 0xFFFF) as u16)
}

pub fn ex11() {
    ex(11, "Inverse function");
    dresult!(
        reverse_map(0.),
        reverse_map(1.),
        reverse_map(map(15, 15)),
    );
}

#[test]
pub fn test_reverse_map() {
    dbg!(reverse_map(0.));
    dbg!(reverse_map(1.));
    dbg!(reverse_map(map(15, 15)));
}