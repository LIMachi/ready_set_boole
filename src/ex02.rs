use crate::result;
use crate::utils::ex;

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

pub fn from_gray(g: u32) -> u32 {
    let mut o = g ^ (g >> 16);
    o ^= o >> 8;
    o ^= o >> 4;
    o ^= o >> 2;
    o ^ o >> 1
}

pub fn ex02() {
    ex(2, "Gray code");
    result!(
        gray_code(0),
        gray_code(1),
        gray_code(2),
        gray_code(3),
        gray_code(4),
        gray_code(5),
        gray_code(6),
        gray_code(7),
        gray_code(8),
        gray_code(15),
        gray_code(255),
        from_gray(gray_code(15)),
        from_gray(gray_code(7)),
        from_gray(gray_code(255)),
    );
}

#[test]
pub fn test_gray_code() {
    dbg!(gray_code(0));
    dbg!(gray_code(1));
    dbg!(gray_code(2));
    dbg!(gray_code(3));
    dbg!(gray_code(4));
    dbg!(gray_code(5));
    dbg!(gray_code(6));
    dbg!(gray_code(7));
    dbg!(gray_code(8));
    dbg!(gray_code(15));
    dbg!(gray_code(255));
    dbg!(from_gray(gray_code(15)));
    dbg!(from_gray(gray_code(7)));
    dbg!(from_gray(gray_code(255)));
}