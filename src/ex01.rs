use crate::ex00::adder;

///allowed: & | ^ << >> = == != < > <= >=
///tolerated: ++/+= only in loops
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut v = a;
    let mut m = b;
    let mut acc = 0;
    while m > 0 {
        if m & 1 == 1 {
            acc = adder(acc, v);
        }
        m >>= 1;
        v <<= 1;
    }
    acc
}

#[test]
pub fn test_multiplier() {
    dbg!(multiplier(0, 10));
    dbg!(multiplier(1, 10));
    dbg!(multiplier(2, 2));
    dbg!(multiplier(3, 5));
    dbg!(multiplier(7, 7));
    dbg!(multiplier(u32::MAX, 1));
    dbg!(multiplier(2, u32::MAX));
    dbg!(multiplier(u32::MAX, u32::MAX));
}