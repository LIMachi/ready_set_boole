pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    vec![]
}

#[test]
pub fn test_eval_set() {
    dbg!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]])); //[0]
    dbg!(eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]])); //[0, 1, 2, 3, 4, 5, 6]
    dbg!(eval_set("A!", vec![vec![0, 1, 2]])); //[]
}