use std::rc::Rc;
use crate::reverse_polish_notation::{RPN, RPNError, RPNNode, RPNVar};

impl RPNNode {
    //swap the children of the node if possible
    //note: material condition is not swappable (asymmetric truth table)
    //ex: AB| -> BA|
    //ex: BA|C| -> ABC||
    pub fn rotate(&mut self) {
        match self {
            Self::Conjunction(r1, r2)
            | Self::Disjunction(r1, r2)
            | Self::ExclusiveDisjunction(r1, r2)
            | Self::LogicalEquivalence(r1, r2) => {
                let t = r1.clone();
                *r1 = r2.clone();
                *r2 = t;
            }
            _ => {}
        }
    }

    pub fn replace_exclusive_disjunctions(&mut self) -> bool {
        if let Self::ExclusiveDisjunction(r1, r2) = self {
            *self = Self::Negation(Rc::new(RPNNode::LogicalEquivalence(r1.clone(), r2.clone())));
            true
        } else {
            false
        }
    }

    pub fn children_count(&self) -> usize {
        match self {
            Self::False | Self::True | Self::Var(_) => 0,
            Self::Negation(_) => 1,
            _ => 2
        }
    }

    pub fn children(&self) -> (Option<Rc<Self>>, Option<Rc<Self>>) {
        match self {
            Self::False | Self::True | Self::Var(_) => (None, None),
            Self::Negation(r) => (Some(r.clone()), None),
            Self::Conjunction(r1, r2)
            | Self::Disjunction(r1, r2)
            | Self::ExclusiveDisjunction(r1, r2)
            | Self::MaterialCondition(r1, r2)
            | Self::LogicalEquivalence(r1, r2) => (Some(r1.clone()), Some(r2.clone()))
        }
    }

    pub fn is_conjunction(self: &Rc<Self>) -> bool {
        if let Self::Conjunction(_, _) = self.as_ref() {
            true
        } else {
            false
        }
    }

    pub fn is_disjunction(self: &Rc<Self>) -> bool {
        if let Self::Disjunction(_, _) = self.as_ref() {
            true
        } else {
            false
        }
    }
}

impl <T: RPNVar + Clone> RPN<T> {
    //similar to nnf, but also ensure there is only !&| symbols (no ^=>) and & are always at the end (resulting in a POS notation aka product of sums)
    pub fn cnf(&mut self) -> Result<(), RPNError> {
        self.rec(&mut RPNNode::replace_exclusive_disjunctions); //AB^ -> AB=!
        self.rec(&mut RPNNode::replace_material_conditions); //AB> -> A!B|
        self.rec(&mut RPNNode::replace_equivalences); //AB= -> AB&A!B!|
        self.rec(&mut RPNNode::apply_morgans_law);
        self.rec(&mut |n| {
            if let (Some(r1), Some(r2)) = n.children() {
                if r1.is_disjunction() && !r2.is_disjunction() {
                    n.rotate();
                    return true;
                }
            }
            false
        });
        self.rec(&mut |n| {
            if let (Some(r1), Some(r2)) = n.children() {
                if r1.is_conjunction() && !r2.is_conjunction() {
                    n.rotate();
                    return true;
                }
            }
            false
        });
        Ok(())
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut rpn = RPN::<bool>::parse(formula).unwrap();
    rpn.cnf().unwrap();
    rpn.as_string()
}

#[test]
pub fn test_nnf() {
    dbg!(conjunctive_normal_form("AB&!"));
    dbg!(conjunctive_normal_form("AB|!"));
    dbg!(conjunctive_normal_form("AB|C&"));
    dbg!(conjunctive_normal_form("AB|C|D|"));
    dbg!(conjunctive_normal_form("AB&C&D&"));
    dbg!(conjunctive_normal_form("AB&!C!|"));
    dbg!(conjunctive_normal_form("AB|!C!&"));

}