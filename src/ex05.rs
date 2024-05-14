use crate::reverse_polish_notation::{RPN, RPNError, RPNNode};

impl RPN {
    pub fn nnf(&mut self) -> Result<(), RPNError> {
        self.rec(&mut RPNNode::replace_equivalences);
        self.rec(&mut RPNNode::replace_material_conditions);
        self.rec(&mut RPNNode::apply_morgans_law);
        self.rec(&mut RPNNode::replace_double_negation);
        Ok(())
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut rpn = RPN::parse(formula).unwrap();
    rpn.nnf().unwrap();
    rpn.as_string()
}

#[test]
pub fn test_nnf() {
    dbg!(negation_normal_form("AB&!"));
    dbg!(negation_normal_form("AB|!"));
    dbg!(negation_normal_form("AB>"));
    dbg!(negation_normal_form("AB="));
    dbg!(negation_normal_form("AB|C&!"));
}