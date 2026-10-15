use std::{collections::HashMap, println};

use crate::solver::formule::formule;

mod solver;

#[cfg(test)]
mod tests;

fn main() {

    let formule = formule!(
        1,
        -1 | 2
    );

    println!("{}", formule);

    let map: HashMap<i32, bool> = formule.solve().unwrap();

    println!("{:?}", map);
}
