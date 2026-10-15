use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::solver::{clause::Clause, literal::Literal};

#[derive(Clone)]
pub struct Formule {
    clauses: Vec<Clause>
}

macro_rules! formule {
    ($($($literal:literal)|+),+) => {
        $crate::solver::formule::Formule::new(vec![
            $(
                $crate::solver::clause::clause!($($literal)|+)
            ),+
        ])
    };
}

pub(crate) use formule;

impl Formule {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self { clauses }
    }

    pub fn evaluate(&self, map: &HashMap<i32, bool>) -> bool {
        self.clauses.iter().all(
            |clause|
            clause.evaluate(map)
        )
    }

    pub fn variables(&self) -> HashSet<i32> {
        self.clauses
            .iter()
            .flat_map(|clause| clause.variables())
            .collect()
    }

    pub fn clauses(&self) -> &[Clause] {
        &self.clauses
    }

    pub fn find_unit_literals(&self) -> Vec<&Literal> {
        self.clauses
            .iter()
            .filter(|clause| clause.literals().len() == 1)
            .filter_map(|clause| clause.literals().first())
            .collect()
    }

    pub fn reduce(&self, map: &HashMap<i32, bool>) -> Formule {
        
        Formule::new(
            self.clauses
                .iter()
                .filter_map(|clause| clause.reduce(map))
                .collect()
        )
    }

    pub fn as_empty_clause(&self) -> bool {
        self.clauses.iter()
            .any(|clause| clause.literals().is_empty())
    }

    pub fn propagate_units(&self) -> Formule {

        let unit_literals = self.find_unit_literals();

        if unit_literals.is_empty() {
            return self.clone();
        }

        let mut map = HashMap::new();

        for literal in unit_literals {
            map.insert(
                literal.abs_value(),
                literal.value() > 0,
            );
        }

        let reduced = self.reduce(&map);

        if reduced.as_empty_clause() {
            return reduced;
        }

        reduced.propagate_units()
    }

    pub fn solve(&self) -> Option<HashMap<i32, bool>> {

        let variables = self.variables();

        let n = variables.len();
        let nb_assignments = 1usize.checked_shl(n as u32)?;

        for counter in 0..nb_assignments {
            let mut solution: HashMap<i32, bool> = HashMap::new();
            
            for (i, variable) in variables.iter().enumerate() {
                let value = ((counter >> i) & 1) == 0;
                solution.insert(*variable, value);
            }

            if self.evaluate(&solution) {
                return Some(solution)
            }
        }

        None
    }
}

impl Display for Formule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, clause) in self.clauses.iter().enumerate() {
            if i > 0 {
                write!(f, " ∧ ")?;
            }

            write!(f, "({})", clause)?;
        }

        Ok(())
    }
}