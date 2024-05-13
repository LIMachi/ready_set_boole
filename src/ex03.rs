pub struct AST(Vec<ASTNode>);

impl AST {
    pub fn parse(formula: &str) -> Result<Self, ASTError> {
        let mut out = Self(Vec::new());
        for (i, c) in formula.chars().enumerate() {
            match c {
                '0' => out.0.push(ASTNode::False),
                '1' => out.0.push(ASTNode::True),
                '!' => if i > 0 { out.0.push(ASTNode::Negation(i - 1)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '&' => if i > 1 { out.0.push(ASTNode::Conjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '|' => if i > 1 { out.0.push(ASTNode::Disjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '^' => if i > 1 { out.0.push(ASTNode::ExclusiveDisjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '>' => if i > 1 { out.0.push(ASTNode::MaterialCondition(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '=' => if i > 1 { out.0.push(ASTNode::LogicalEquivalence(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                _ => return Err(ASTError::InvalidCharacterAtPosition(c, i))
            }
        }
        Ok(out)
    }

    pub fn get(&self, index: usize) -> Result<bool, ASTError> {
        if index >= self.0.len() {
            Err(ASTError::InvalidIndexForASTOfLength(index, self.0.len()))
        } else {
            match self.0[index] {
                ASTNode::False => Ok(false),
                ASTNode::True => Ok(true),
                ASTNode::Negation(i) => self.get(i).map(|v| !v),
                ASTNode::Conjunction(i1, i2) => self.get(i1).and_then(|v1| self.get(i2).map(|v2| v1 && v2)),
                ASTNode::Disjunction(i1, i2) => self.get(i1).and_then(|v1| self.get(i2).map(|v2| v1 || v2)),
                ASTNode::ExclusiveDisjunction(i1, i2) => self.get(i1).and_then(|v1| self.get(i2).map(|v2| v1 ^ v2)),
                ASTNode::MaterialCondition(i1, i2) => self.get(i1).and_then(|v1| self.get(i2).map(|v2| !(v1 && !v2))),
                ASTNode::LogicalEquivalence(i1, i2) => self.get(i1).and_then(|v1| self.get(i2).map(|v2| v1 == v2)),
            }
        }
    }

    pub fn evaluate(&self) -> Result<bool, ASTError> {
        if self.0.len() == 0 {
            Ok(false)
        } else {
            self.get(self.0.len() - 1)
        }
    }
}

#[derive(Debug)]
pub enum ASTError {
    InvalidCharacterAtPosition(char, usize),
    InvalidBackReference(char, usize),
    InvalidIndexForASTOfLength(usize, usize),

}

#[derive(Clone)]
pub enum ASTNode {
    False,
    True,
    Negation(usize),
    Conjunction(usize, usize),
    Disjunction(usize, usize),
    ExclusiveDisjunction(usize, usize),
    MaterialCondition(usize, usize),
    LogicalEquivalence(usize, usize)
}

pub fn eval_formula(formula: &str) -> bool {
    AST::parse(formula).unwrap().evaluate().unwrap()
}

#[test]
pub fn test_eval_formula() {
    dbg!(eval_formula("10&"));
    dbg!(eval_formula("10|"));
    dbg!(eval_formula("11>"));
    dbg!(eval_formula("10="));
    dbg!(eval_formula("1011||="));
}