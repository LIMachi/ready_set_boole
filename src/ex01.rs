use crate::ex00::adder;
use crate::result;
use crate::utils::ex;

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

pub fn ex01() {
    ex(1, "Multiplier");
    result!(
        multiplier(0, 10),
        multiplier(1, 10),
        multiplier(2, 2),
        multiplier(3, 5),
        multiplier(7, 7),
        multiplier(u32::MAX, 1),
        multiplier(2, u32::MAX),
        multiplier(u32::MAX, u32::MAX),
    );
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