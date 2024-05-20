use crate::result;
use crate::reverse_polish_notation::{RPN, RPNError, RPNNode, RPNVar};
use crate::utils::ex;

impl <T: RPNVar + Clone> RPN<T> {
    pub fn nnf(&mut self) -> Result<(), RPNError> {
        self.rec(&mut RPNNode::replace_equivalences);
        self.rec(&mut RPNNode::replace_material_conditions);
        self.rec(&mut RPNNode::apply_morgans_law);
        self.rec(&mut RPNNode::replace_double_negation);
        Ok(())
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut rpn = RPN::<bool>::parse(formula).unwrap();
    rpn.nnf().unwrap();
    rpn.as_string()
}

pub fn ex05() {
    ex(5, "Negation normal form");
    result!(
        negation_normal_form("AB&!"),
        negation_normal_form("AB|!"),
        negation_normal_form("AB>"),
        negation_normal_form("AB="),
        negation_normal_form("AB|C&!"),
    );
}

#[test]
pub fn test_nnf() {
    dbg!(negation_normal_form("AB&!"));
    dbg!(negation_normal_form("AB|!"));
    dbg!(negation_normal_form("AB>"));
    dbg!(negation_normal_form("AB="));
    dbg!(negation_normal_form("AB|C&!"));
}