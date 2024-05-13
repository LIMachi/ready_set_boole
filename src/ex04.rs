pub struct AST {
    used: Vec<char>,
    vars: Vec<bool>,
    nodes: Vec<ASTNode>,
}

impl AST {
    pub fn parse(formula: &str) -> Result<Self, ASTError> {
        let mut out = Self {
            used: vec![],
            vars: vec![],
            nodes: vec![],
        };
        for (i, c) in formula.chars().enumerate() {
            match c {
                v @ 'A' ..= 'Z' => {
                    let u = v as usize - 'A' as usize;
                    out.nodes.push(ASTNode::Var(u));
                    if !out.used.contains(&v) {
                        out.used.push(v);
                        out.vars.push(false);
                    }
                },
                '!' => if i > 0 { out.nodes.push(ASTNode::Negation(i - 1)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '&' => if i > 1 { out.nodes.push(ASTNode::Conjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '|' => if i > 1 { out.nodes.push(ASTNode::Disjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '^' => if i > 1 { out.nodes.push(ASTNode::ExclusiveDisjunction(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '>' => if i > 1 { out.nodes.push(ASTNode::MaterialCondition(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                '=' => if i > 1 { out.nodes.push(ASTNode::LogicalEquivalence(i - 1, i - 2)); } else { return Err(ASTError::InvalidBackReference(c, i)); },
                _ => return Err(ASTError::InvalidCharacterAtPosition(c, i))
            }
        }
        Ok(out)
    }

    pub fn get(&self, index: usize) -> Result<bool, ASTError> {
        if index >= self.nodes.len() {
            Err(ASTError::InvalidIndexForASTOfLength(index, self.nodes.len()))
        } else {
            match self.nodes[index] {
                ASTNode::Var(i) => Ok(self.vars[i]),
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
        if self.nodes.len() == 0 {
            Ok(false)
        } else {
            self.get(self.nodes.len() - 1)
        }
    }

    pub fn print_truth_table(&mut self) -> Result<(), ASTError> {
        for c in &self.used {
            print!("| {c} ");
        }
        println!("| = |");
        let l = self.used.len();
        for _ in 0..l {
            print!("|---");
        }
        println!("|---|");
        for i in 0..1 << l {
            for v in 0..l {
                self.vars[v] = ((i >> (l - 1 - v)) & 1) == 1;
                print!("| {} ", if self.vars[v] { 1 } else { 0 });
            }
            println!("| {} |", if self.evaluate()? { 1 } else { 0 });
        }
        Ok(())
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
    Var(usize),
    Negation(usize),
    Conjunction(usize, usize),
    Disjunction(usize, usize),
    ExclusiveDisjunction(usize, usize),
    MaterialCondition(usize, usize),
    LogicalEquivalence(usize, usize)
}

pub fn print_truth_table(formula: &str) {
    AST::parse(formula).unwrap().print_truth_table().unwrap()
}

#[test]
pub fn test_eval_formula() {
    dbg!(print_truth_table("AB&C|"));
}