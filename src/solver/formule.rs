use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::solver::clause::Clause;

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

    pub fn literals(&self) -> HashSet<i32> {
        let mut literals = HashSet::new();

        for clause in &self.clauses{
            for literal in clause.literals() {
                literals.insert(literal);
            }
        }

        literals
    }

    pub fn solve(&self) -> Option<HashMap<i32, bool>> {

        let variables = self.literals();

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