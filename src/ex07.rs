use crate::reverse_polish_notation::{RPN, RPNError};

impl RPN<bool> {
    //find if any combination of var return true (similar to truth table)
    pub fn sat(&mut self) -> Result<bool, RPNError> {
        self.extract_truth_table().map(|v| v.iter().fold(false, |a, (_, r)| a || *r))
    }
}

pub fn sat(formula: &str) -> bool {
    RPN::parse(formula).unwrap().sat().unwrap()
}

#[test]
pub fn test_sat() {
    dbg!(sat("AB|")); //true
    dbg!(sat("AB&")); //true
    dbg!(sat("AA!&")); //false
    dbg!(sat("AA^")); //false
}