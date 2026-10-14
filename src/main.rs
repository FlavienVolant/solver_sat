use std::{collections::HashMap, println};

use crate::solver::formule::formule;

mod solver;

#[cfg(test)]
mod tests;

fn main() {

    let formule = formule!(
        1 | 2 | 3,
        -1
    );

    let map: HashMap<i32, bool> = formule.solve().unwrap();

    println!("{} {:?}", formule, map);
}
