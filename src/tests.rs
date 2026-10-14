
#[cfg(test)]
mod tests {
    use crate::{HashMap, solver::formule::formule};
    use std::vec;

    use crate::solver::{clause::Clause, formule::Formule, literal::Literal};

    #[test]
    fn test_evaluate() {
        // (A v B) ^ -A
        let formule = formule!(
            1 | 2,
            -1
        );

        let mut succed = HashMap::new();
        succed.insert(1, false);
        succed.insert(2, true);


        let mut failed = HashMap::new();
        failed.insert(1, false);
        failed.insert(2, false);

        assert!(formule.evaluate(&succed));
        assert!(!formule.evaluate(&failed));
    }

    #[test]
    fn test_evaluate_when_no_value_is_specified_for_a_literal_default_value_is_true() {
        let a = formule!(1);

        let not_a = Formule::new(
            vec!(
                Clause::new(
                    vec!(
                        Literal::new(-1)
                    )
                )
            )
        );

        assert!(a.evaluate(&HashMap::new()));
        assert!(!not_a.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_literals() {
        let formule = formule!(
            1 | 2,
            -1,
            -2
        );

        let literals = formule.literals();

        assert!(literals.len() == 2);
        assert!(literals.contains(&1));
        assert!(literals.contains(&2));
        assert!(!literals.contains(&-1));
        assert!(!literals.contains(&-2));
    }

    #[test]
    fn test_solve() {
        let solvable = formule!(
            1 | 2,
            -1 | 3,
            -2 | 3
        );

        let unsolvable = formule!(
            1,
            -1
        );

        assert!(solvable.solve().is_some());
        assert!(unsolvable.solve().is_none());
    }

}