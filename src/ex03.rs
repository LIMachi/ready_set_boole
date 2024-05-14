use crate::reverse_polish_notation::RPN;

pub fn eval_formula(formula: &str) -> bool {
    RPN::parse(formula).unwrap().evaluate().unwrap()
}

#[test]
pub fn test_eval_formula() {
    dbg!(eval_formula("10&"));
    dbg!(eval_formula("10|"));
    dbg!(eval_formula("11>"));
    dbg!(eval_formula("10="));
    dbg!(eval_formula("1011||="));
}