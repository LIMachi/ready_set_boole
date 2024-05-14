pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    vec![vec![]]
}

#[test]
pub fn test_powerset() {
    dbg!(powerset(vec![1, 2, 3]));
}