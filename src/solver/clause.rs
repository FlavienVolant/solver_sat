use std::{collections::HashMap, fmt::Display};

use crate::solver::literal::Literal;

pub struct Clause {
    literals: Vec<Literal>
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn evaluate(&self, map: &HashMap<i32, bool>) -> bool {
        self.literals.iter().any(
            |literal| 
            literal.evaluate(*map.get(&literal.get_v()).unwrap_or(&true))
        )
    }

    pub fn literals(&self) -> Vec<i32> {
        self.literals.iter().map(
            |literal|
            literal.get_v()
        ).collect()
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, literal) in self.literals.iter().enumerate() {
            if i > 0 {
                write!(f, " ∨ ")?;
            }

            write!(f, "{}", literal)?;
        }

        Ok(())
    }
}