use crate::reverse_polish_notation::{RPN, RPNError};
use crate::show;
use crate::utils::ex;

impl RPN<bool> {
    pub fn extract_truth_table(&mut self) -> Result<Vec<(Vec<bool>, bool)>, RPNError> {
        let l = self.var_names.len();
        let mut out = Vec::new();
        for i in 0..1 << l {
            for v in 0..l {
                self.vars[v] = ((i >> (l - 1 - v)) & 1) == 1;
            }
            out.push((self.vars.clone(), self.evaluate()?));
        }
        Ok(out)
    }

    pub fn print_truth_table(&mut self) -> Result<(), RPNError> {
        for c in &self.var_names {
            print!("| {c} ");
        }
        println!("| = |");
        let l = self.var_names.len();
        for _ in 0..l {
            print!("|---");
        }
        println!("|---|");
        for (v, r) in self.extract_truth_table()?.iter() {
            for s in v {
                print!("| {} ", if *s { 1 } else { 0 });
            }
            println!("| {} |", if *r { 1 } else { 0 });
        }
        Ok(())
    }
}

pub fn print_truth_table(formula: &str) {
    RPN::parse(formula).unwrap().print_truth_table().unwrap()
}

pub fn ex04() {
    ex(4, "Truth table");
    show!(print_truth_table("AB&C|"));
}

#[test]
pub fn test_print_truth_table() {
    dbg!(print_truth_table("AB&C|"));
}