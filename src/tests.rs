
#[cfg(test)]
mod tests {
    use crate::{HashMap, solver::formule::formule};
    use std::{assert_eq, vec};

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
    fn test_variables() {
        let formule = formule!(
            1 | 2,
            -1,
            -2
        );

        let variables = formule.variables();

        assert_eq!(variables.len(), 2);
        assert!(variables.contains(&1));
        assert!(variables.contains(&2));
        assert!(!variables.contains(&-1));
        assert!(!variables.contains(&-2));
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

    #[test]
    fn test_find_unit_clauses() {
        let formule = formule!(
            1 | 2,
            -3,
            4,
            -1
        );

        let unit_literals: Vec<&Literal> = formule.find_unit_literals();

        assert_eq!(unit_literals.len(), 3);
        assert!(unit_literals.iter().any(|l| l.value() == -3));
        assert!(unit_literals.iter().any(|l| l.value() == 4));
        assert!(unit_literals.iter().any(|l| l.value() == -1));
        assert!(!unit_literals.iter().any(|l| l.value() == 2));
    }
}