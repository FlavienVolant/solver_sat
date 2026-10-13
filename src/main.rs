use std::{collections::HashMap, println};

use crate::solver::{clause::Clause, formule::Formule, literal::Literal};

mod solver;

#[cfg(test)]
mod tests;

fn main() {

    let formule: Formule = Formule::new(
        vec!(
            Clause::new(
                vec!(
                    Literal::new(1),
                    Literal::new(2)
                )
            ),
            Clause::new(
                vec!(
                    Literal::new(-1)
                )
            )
        )
    );

    let map: HashMap<i32, bool> = formule.solve().unwrap();

    println!("{} {:?}", formule, map);
}
