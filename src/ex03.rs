use crate::result;
use crate::reverse_polish_notation::RPN;
use crate::utils::ex;

pub fn eval_formula(formula: &str) -> bool {
    RPN::parse(formula).unwrap().evaluate().unwrap()
}

pub fn ex03() {
    ex(3, "Boolean evaluation");
    result!(
        eval_formula("10&"),
        eval_formula("10|"),
        eval_formula("11>"),
        eval_formula("10="),
        eval_formula("1011||="),
    );
}

#[test]
pub fn test_eval_formula() {
    dbg!(eval_formula("10&"));
    dbg!(eval_formula("10|"));
    dbg!(eval_formula("11>"));
    dbg!(eval_formula("10="));
    dbg!(eval_formula("1011||="));
}