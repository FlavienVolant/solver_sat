use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::solver::literal::Literal;

#[derive(Clone)]
pub struct Clause {
    literals: Vec<Literal>
}

macro_rules! clause {

    ($($literal: literal)|+) => {
        $crate::solver::clause::Clause::new(vec![
            $(
                $crate::solver::literal::Literal::new($literal),
            )+
        ])
    };
}

pub(crate) use clause;

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn evaluate(&self, map: &HashMap<i32, bool>) -> bool {
        self.literals.iter().any(
            |literal| 
            literal.evaluate(*map.get(&literal.abs_value()).unwrap_or(&true))
        )
    }

    pub fn reduce(&self, map: &HashMap<i32, bool>) -> Option<Clause> {

        self.literals.iter()
            .try_fold(
                Vec::new(), 
                |mut reduced, literal|
                match map.get(&literal.abs_value()){
                    Some(boolean) if literal.evaluate(*boolean) => None,
                    Some(_) => Some(reduced),
                    None => {
                        reduced.push(literal.clone());
                        Some(reduced)
                    },
                })
            .map(Clause::new)
    }

    pub fn literals(&self) -> &[Literal] {
        &self.literals
    }

    pub fn variables(&self) -> HashSet<i32> {
        self.literals
            .iter()
            .map(Literal::abs_value)
            .collect()
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