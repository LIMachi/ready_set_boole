use std::rc::Rc;
use crate::reverse_polish_notation::RPNNode;

impl RPNNode {

    pub fn deep_clone(&self) -> Rc<Self> {
        match self {
            Self::False | Self::True | Self::Var(_) => Rc::new(self.clone()),
            Self::Negation(r) => Rc::new(Self::Negation(r.deep_clone())),
            Self::Conjunction(r1, r2) => Rc::new(Self::Conjunction(r1.deep_clone(), r2.deep_clone())),
            Self::Disjunction(r1, r2) => Rc::new(Self::Disjunction(r1.deep_clone(), r2.deep_clone())),
            Self::ExclusiveDisjunction(r1, r2) => Rc::new(Self::ExclusiveDisjunction(r1.deep_clone(), r2.deep_clone())),
            Self::MaterialCondition(r1, r2) => Rc::new(Self::MaterialCondition(r1.deep_clone(), r2.deep_clone())),
            Self::LogicalEquivalence(r1, r2) => Rc::new(Self::LogicalEquivalence(r1.deep_clone(), r2.deep_clone())),
        }
    }

    pub fn replace_double_negation(&mut self) -> bool {
        let mut t = None;
        if let Self::Negation(n1) = self {
            if let Self::Negation(n2) = n1.as_ref() {
                t = Some((**n2).clone());
            }
        }
        if let Some(t) = t {
            *self = t;
            true
        } else {
            false
        }
    }

    pub fn replace_material_conditions(&mut self) -> bool {
        if let Self::MaterialCondition(r1, r2) = self {
            *self = Self::Disjunction(Rc::new(RPNNode::Negation(r1.clone())), r2.clone());
            true
        } else {
            false
        }
    }

    pub fn replace_equivalences(&mut self) -> bool {
        if let Self::LogicalEquivalence(r1, r2) = self {
            *self = Self::Disjunction(Rc::new(Self::Conjunction(r1.deep_clone(), r2.deep_clone())), Rc::new(Self::Conjunction(Rc::new(Self::Negation(r1.clone())), Rc::new(Self::Negation(r2.clone())))));
            true
        } else {
            false
        }
    }

    pub fn apply_morgans_law(&mut self) -> bool {
        let mut t = None;
        if let Self::Negation(n) = self {
            if let Self::Disjunction(r1, r2) = n.as_ref() {
                t = Some(Self::Conjunction(Rc::new(Self::Negation(r1.clone())), Rc::new(Self::Negation(r2.clone()))));
            }
            if let Self::Conjunction(r1, r2) = n.as_ref() {
                t = Some(Self::Disjunction(Rc::new(Self::Negation(r1.clone())), Rc::new(Self::Negation(r2.clone()))));
            }
        }
        if let Some(t) = t {
            *self = t;
            true
        } else {
            false
        }
    }
}