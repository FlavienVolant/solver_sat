
#[cfg(test)]
mod tests {
    use crate::{HashMap, solver::{clause::clause, formule::formule}};
    use std::{assert_eq, collections::HashSet, vec};

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
    fn test_find_unit_literals() {
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
    
    #[test]
    fn test_reduce() {
        let formule = formule!(
            1 | 2,
            -1 | 3,
            2 | -3
        );

        let mut map = HashMap::new();
        map.insert(1, true);

        let reduced = formule.reduce(&map);

        assert_eq!(reduced.variables().len(), 2);
        assert!(reduced.find_unit_literals().iter().any(|l| l.value() == 3));
        assert_eq!(reduced.find_unit_literals().len(), 1);
        assert_eq!(reduced.variables(), HashSet::from([2, 3]));
    }

    #[test]
    fn test_reduce_clause_totology_return_none() {
        let clause = clause!(
            -1 | 1
        );

        let mut map = HashMap::new();
        map.insert(1, false);

        let reduced = clause.reduce(&map);

        assert!(reduced.is_none());
    }

    #[test]
    fn test_reduce_formule_impossible_as_empty_clause() {
        let formule = formule!(
            1,
            -1
        );

        let mut map = HashMap::new();
        map.insert(1, false);

        assert!(formule.reduce(&map).as_empty_clause());
    }

    #[test]
    fn test_reduce_formule_fully_reduced_is_empty() {
        let formule = formule!(
            1 | 2,
            -2 | 4
        );

        let mut map = HashMap::new();
        map.insert(1, true);
        map.insert(2, false);

        assert!(formule.reduce(&map).clauses().is_empty());
    }

    #[test]
    fn test_reduce_keeps_unassigned_literals() {
        let formule = formule!(
            1 | 2
        );

        let mut map = HashMap::new();
        map.insert(1, false);

        let reduced = formule.reduce(&map);

        assert_eq!(reduced.variables(), HashSet::from([2]));
        assert_eq!(reduced.find_unit_literals().len(), 1);
        assert_eq!(reduced.find_unit_literals()[0].value(), 2);
    }

    #[test]
    fn test_propagate_units_without_unit_literals() {
        let formule = formule!(
            1 | 2,
            -1 | 3,
            2 | -3
        );

        let mut map = HashMap::new();
        let propagated = formule.propagate_units(&mut map);

        assert_eq!(propagated.to_string(), formule.to_string());
        assert!(map.is_empty());
    }

    #[test]
    fn test_propagate_units_chain() {
        let formule = formule!(
            1,
            -1 | 2,
            -2 | 3
        );

        let mut map = HashMap::new();
        let propagated = formule.propagate_units(&mut map);

        assert!(propagated.clauses().is_empty());
        assert_eq!(map.len(), 3);
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert!(map.contains_key(&3));
    }

    #[test]
    fn test_propagate_units_detects_indirect_contradiction() {
        let formule = formule!(
            1,
            -1 | 2,
            -2
        );

        let propagated = formule.propagate_units(&mut HashMap::new());

        assert!(propagated.as_empty_clause());
    }

    #[test]
    fn test_solve_more() {
        let a = formule!(1);
        let a_option_solution = a.solve();
        assert!(a_option_solution.is_some());
        let a_solution = a_option_solution.unwrap();
        assert!(a_solution.contains_key(&1));
        assert_eq!(a_solution.get(&1).unwrap(), &true);

        let b = formule!(1, -1);
        let b_option_solution = b.solve();
        assert!(b_option_solution.is_none());
        
        let c = formule!(1 | 2, -1 | 2, 1 | -2);
        let c_option_solution = c.solve();
        assert!(c_option_solution.is_some());
        let c_solution = c_option_solution.unwrap();
        assert!(c_solution.contains_key(&1));
        assert!(c_solution.contains_key(&2));
        assert_eq!(c_solution.get(&1).unwrap(), &true);
        assert_eq!(c_solution.get(&2).unwrap(), &true);

        let d = formule!(1 | 2, 1 | -2, -1 | 2, -1 | -2);
        let d_option_solution = d.solve();
        assert!(d_option_solution.is_none());
    }
}