use crate::reverse_polish_notation::{RPN, RPNNode, RPNVar};

impl RPNVar for Vec<i32> {
    fn from_bool(val: bool) -> Self {
        vec![]
    }

    fn negation(&self) -> Self { //the negation of a set should technically be all the values not in the set but valid (ex: ![1] should be all the values of i32 except 1, but the examples given in the document seem to implement the negation of a set as the empty set)
        vec![]
    }

    fn conjunction(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for t in self {
            if other.contains(t) {
                out.push(*t);
            }
        }
        out
    }

    fn disjunction(&self, other: &Self) -> Self {
        let mut out = self.clone();
        for t in other {
            if !out.contains(t) {
                out.push(*t);
            }
        }
        out
    }

    fn exclusive_disjunction(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for t in self {
            if !other.contains(t) {
                out.push(*t);
            }
        }
        for t in other {
            if !self.contains(t) {
                out.push(*t);
            }
        }
        out
    }

    fn material_condition(&self, other: &Self) -> Self {
        todo!()
    }

    fn logical_equivalence(&self, other: &Self) -> Self {
        let mut out = Self::new();
        for t in self {
            if other.contains(t) {
                out.push(*t);
            }
        }
        out
    }
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rpn = RPN::<Vec<i32>>::parse(formula).unwrap();
    rpn.rec(&mut RPNNode::replace_material_conditions); //since material condition is not currently implemented for Vec<i32>, replace them TODO: implement material condition for sets (aka Vec<i32>)
    rpn.vars = sets;
    rpn.evaluate().unwrap()
}

#[test]
pub fn test_eval_set() {
    dbg!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]])); //[0]
    dbg!(eval_set("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]])); //[0, 1, 2, 3, 4, 5]
    dbg!(eval_set("A!", vec![vec![0, 1, 2]])); //[]
}