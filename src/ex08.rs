use crate::dresult;
use crate::utils::ex;

//note: the technique used is limited to a maximum of 63 initial terms (since I use binary properties to generate my unique sets)
//a recursive method should be preferred for bigger sets (but 63 initial terms is already bigger than most computer's memory so...)
//on my machine I overflow before 42 initial terms (32G of memory)
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = Vec::with_capacity(1<<set.len());
    for i in 0..(1usize << set.len()) {
        let mut t = Vec::new();
        for m in 0..64 {
            if i & (1 << m) != 0 {
                t.push(set[m]);
            }
        }
        out.push(t);
    }
    out
}

pub fn ex08() {
    ex(8, "Powerset");
    dresult!(
        powerset(vec![]),
        powerset(vec![1]),
        powerset(vec![1, 2, 3]),
        powerset(vec![1, 2, 3, 4, 5, 6]),
    );
}

#[test]
pub fn test_powerset() {
    dbg!(powerset(vec![]));
    dbg!(powerset(vec![1]));
    dbg!(powerset(vec![1, 2, 3]));
    dbg!(powerset(vec![1, 2, 3, 4, 5, 6]));
    // dbg!(powerset(vec![0; 25]).iter().flatten().count()); //419430400
}