use std::rc::Rc;

#[derive(Debug)]
pub struct RPN<T: RPNVar + Clone> {
    pub var_names: Vec<char>,
    pub vars: Vec<T>,
    pub nodes: Vec<Rc<RPNNode>>,
}

#[derive(Debug)]
pub enum RPNError {
    InvalidCharacter(char),
    InvalidBackReference(char, usize),
    InvalidIndexForASTOfLength(usize, usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RPNNode {
    False, // 0
    True, // 1
    Var(usize), // A..=Z
    Negation(Rc<RPNNode>), // !
    Conjunction(Rc<RPNNode>, Rc<RPNNode>), // &
    Disjunction(Rc<RPNNode>, Rc<RPNNode>), // |
    ExclusiveDisjunction(Rc<RPNNode>, Rc<RPNNode>), // ^
    MaterialCondition(Rc<RPNNode>, Rc<RPNNode>), // >
    LogicalEquivalence(Rc<RPNNode>, Rc<RPNNode>), // =
}

pub trait RPNVar {
    fn from_bool(val: bool) -> Self;
    fn negation(&self) -> Self;
    fn conjunction(&self, other: &Self) -> Self;
    fn disjunction(&self, other: &Self) -> Self;
    fn exclusive_disjunction(&self, other: &Self) -> Self;
    fn material_condition(&self, other: &Self) -> Self;
    fn logical_equivalence(&self, other: &Self) -> Self;
}

impl RPNVar for bool {
    fn from_bool(val: bool) -> Self { val }

    fn negation(&self) -> Self { !self }

    fn conjunction(&self, other: &Self) -> Self { self & other }

    fn disjunction(&self, other: &Self) -> Self { self | other }

    fn exclusive_disjunction(&self, other: &Self) -> Self { self ^ other }

    fn material_condition(&self, other: &Self) -> Self { !(self & !other) }

    fn logical_equivalence(&self, other: &Self) -> Self { self == other }
}

impl RPNNode {
    pub fn solve<T: RPNVar + Clone>(&self, vars: &Vec<T>) -> Result<T, RPNError> {
        match self {
            Self::False => Ok(T::from_bool(false)),
            Self::True => Ok(T::from_bool(true)),
            Self::Var(i) => Ok(vars[*i].clone()),
            Self::Negation(r) => r.solve(vars).map(|v| v.negation()),
            Self::Conjunction(r1, r2) => r1.solve(vars).and_then(|v1| r2.solve(vars).map(|v2| (&v1).conjunction(&v2))),
            Self::Disjunction(r1, r2) => r1.solve(vars).and_then(|v1| r2.solve(vars).map(|v2| (&v1).disjunction(&v2))),
            Self::ExclusiveDisjunction(r1, r2) => r1.solve(vars).and_then(|v1| r2.solve(vars).map(|v2| (&v1).exclusive_disjunction(&v2))),
            Self::MaterialCondition(r1, r2) => r1.solve(vars).and_then(|v1| r2.solve(vars).map(|v2| (&v1).material_condition(&v2))),
            Self::LogicalEquivalence(r1, r2) => r1.solve(vars).and_then(|v1| r2.solve(vars).map(|v2| (&v1).logical_equivalence(&v2))),
        }
    }

    pub fn rec<F>(&mut self, run: &mut F) -> bool where F: FnMut(&mut Self) -> bool {
        match self {
            RPNNode::False => run(self),
            RPNNode::True => run(self),
            RPNNode::Var(_) => run(self),
            RPNNode::Negation(r) => Rc::get_mut(r).unwrap().rec(run) || run(self),
            RPNNode::Conjunction(r1, r2) => Rc::get_mut(r1).unwrap().rec(run) || Rc::get_mut(r2).unwrap().rec(run) || run(self),
            RPNNode::Disjunction(r1, r2) => Rc::get_mut(r1).unwrap().rec(run) || Rc::get_mut(r2).unwrap().rec(run) || run(self),
            RPNNode::ExclusiveDisjunction(r1, r2) => Rc::get_mut(r1).unwrap().rec(run) || Rc::get_mut(r2).unwrap().rec(run) || run(self),
            RPNNode::MaterialCondition(r1, r2) => Rc::get_mut(r1).unwrap().rec(run) || Rc::get_mut(r2).unwrap().rec(run) || run(self),
            RPNNode::LogicalEquivalence(r1, r2) => Rc::get_mut(r1).unwrap().rec(run) || Rc::get_mut(r2).unwrap().rec(run) || run(self),
        }
    }
}

impl <T: RPNVar + Clone> RPN<T> {
    pub fn parse(formula: &str) -> Result<Self, RPNError> {
        let mut out = Self {
            var_names: vec![],
            vars: vec![],
            nodes: vec![]
        };
        for (i, c) in formula.chars().enumerate() {
            let l = out.nodes.len();
            match c {
                '0' => out.nodes.push(Rc::new(RPNNode::False)),
                '1' => out.nodes.push(Rc::new(RPNNode::True)),
                'A' ..= 'Z' => {
                    out.nodes.push(Rc::new(RPNNode::Var(c as usize - 'A' as usize)));
                    if !out.var_names.contains(&c) {
                        out.var_names.push(c);
                        out.vars.push(T::from_bool(false));
                    }
                }
                '!' => if l > 0 { out.nodes[l - 1] = Rc::new(RPNNode::Negation(out.nodes[l - 1].clone())); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                '&' => if l > 1 { out.nodes[l - 2] = Rc::new(RPNNode::Conjunction(out.nodes[l - 2].clone(), out.nodes[l - 1].clone())); out.nodes.pop(); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                '|' => if l > 1 { out.nodes[l - 2] = Rc::new(RPNNode::Disjunction(out.nodes[l - 2].clone(), out.nodes[l - 1].clone())); out.nodes.pop(); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                '^' => if l > 1 { out.nodes[l - 2] = Rc::new(RPNNode::ExclusiveDisjunction(out.nodes[l - 2].clone(), out.nodes[l - 1].clone())); out.nodes.pop(); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                '>' => if l > 1 { out.nodes[l - 2] = Rc::new(RPNNode::MaterialCondition(out.nodes[l - 2].clone(), out.nodes[l - 1].clone())); out.nodes.pop(); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                '=' => if l > 1 { out.nodes[l - 2] = Rc::new(RPNNode::LogicalEquivalence(out.nodes[l - 2].clone(), out.nodes[l - 1].clone())); out.nodes.pop(); } else { return Err(RPNError::InvalidBackReference(c, i)); }
                _ => return Err(RPNError::InvalidCharacter(c))
            }
        }
        Ok(out)
    }

    pub fn evaluate(&self) -> Result<T, RPNError> {
        if self.nodes.len() == 0 {
            Ok(T::from_bool(false))
        } else {
            self.nodes[self.nodes.len() - 1].solve(&self.vars)
        }
    }

    pub fn rec<F>(&mut self, run: &mut F) -> &mut Self
        where F: FnMut(&mut RPNNode) -> bool
    {
        while Rc::get_mut(&mut self.nodes[0]).unwrap().rec(run) {}
        self
    }

    pub fn as_string(&mut self) -> String {
        let mut out = String::new();
        let ro = &mut out;
        let names = self.var_names.clone();
        self.rec(&mut move |n| {
            match n {
                RPNNode::False => *ro += "0",
                RPNNode::True => *ro += "1",
                RPNNode::Var(i) => *ro += names[*i].to_string().as_str(),
                RPNNode::Negation(_) => *ro += "!",
                RPNNode::Conjunction(_, _) => *ro += "&",
                RPNNode::Disjunction(_, _) => *ro += "|",
                RPNNode::ExclusiveDisjunction(_, _) => *ro += "^",
                RPNNode::MaterialCondition(_, _) => *ro += ">",
                RPNNode::LogicalEquivalence(_, _) => *ro += "=",
            }
            false
        });
        out
    }
}