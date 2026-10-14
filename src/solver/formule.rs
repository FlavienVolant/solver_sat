use std::{collections::{HashMap, HashSet}, fmt::Display};

use crate::solver::{clause::Clause, literal::Literal};

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

    pub fn find_unit_literals(&self) -> Vec<&Literal> {
        self.clauses
            .iter()
            .filter(|clause| clause.literals().len() == 1)
            .filter_map(|clause| clause.literals().first())
            .collect()
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