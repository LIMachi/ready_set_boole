use crate::result;
use crate::utils::ex;

///allowed: & | ^ << >> = == != < > <= >=
///tolerated: ++/+= only in loops
pub fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut q = b;
    loop {
        (x, q) = (x ^ q, x & q);
        if q == 0 {
            return x;
        }
        q <<= 1;
    }
}

pub fn ex00() {
    ex(0, "Adder");
    result!(
        adder(0, 1),
        adder(1, 1),
        adder(2, 2),
        adder(3, 5),
        adder(7, 7),
        adder(u32::MAX, 1),
        adder(2, u32::MAX),
        adder(u32::MAX, u32::MAX),
    );
}

#[test]
pub fn test_adder() {
    dbg!(adder(0, 1));
    dbg!(adder(1, 1));
    dbg!(adder(2, 2));
    dbg!(adder(3, 5));
    dbg!(adder(7, 7));
    dbg!(adder(u32::MAX, 1));
    dbg!(adder(2, u32::MAX));
    dbg!(adder(u32::MAX, u32::MAX));
}