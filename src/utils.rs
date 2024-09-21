use crate::types::{Clause, Formula, Solution, Variable, Variables};

pub fn print_formula(formula: &Formula) {
    for i in 0..formula.len() {
        let clause = &formula[i];
        print!("(");
        for j in 0..clause.len() {
            let variable = &clause[j];
            match variable {
                Variable::Positive(id) => {
                    print!("x{}", id);
                }
                Variable::Negative(id) => {
                    print!("-x{}", id);
                }
            }
            if j < clause.len() - 1 {
                print!(" OR ");
            }
        }
        print!(")");
        if i < formula.len() - 1 {
            print!(" AND ");
        }
    }
    println!();
}

pub fn print_solution(solution: &Solution) {
    let mut variables = solution.keys().collect::<Vec<_>>();
    variables.sort();
    for i in 0..variables.len() {
        let id = variables[i];
        let value = solution.get(id).unwrap();
        print!("x{} = {}", id, if *value { "T" } else { "F" });
        if i < variables.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

pub fn eval_variable(variable: &Variable, solution: &Solution) -> bool {
    match variable {
        Variable::Positive(id) => *solution.get(id).unwrap_or(&false),
        Variable::Negative(id) => !solution.get(id).unwrap_or(&false),
    }
}

/// A clause is satisfied if at least one of its variables is true.
pub fn satisfy_clause(clause: &Clause, solution: &Solution) -> bool {
    for variable in clause {
        if eval_variable(variable, solution) {
            return true;
        }
    }
    false
}

/// A formula is satisfied if all of its clauses are satisfied.
pub fn satisfy_formula(formula: &Formula, solution: &Solution) -> bool {
    for clause in formula {
        if !satisfy_clause(clause, solution) {
            return false;
        }
    }
    true
}

pub fn get_variables(formula: &Formula) -> Variables {
    let mut variables = Variables::new();
    for clause in formula {
        for variable in clause {
            match variable {
                Variable::Positive(id) | Variable::Negative(id) => {
                    variables.push(*id);
                }
            }
        }
    }
    variables.sort();
    variables.dedup();
    variables
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_formula() {
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        print!("Formula: ");
        print_formula(&formula);
    }

    #[test]
    fn test_print_solution() {
        let solution = Solution::from([(1, true), (2, false), (3, true)]);
        print!("Solution: ");
        print_solution(&solution);
    }

    #[test]
    fn test_eval_variable() {
        let solution = Solution::from([(1, true), (2, false)]);
        assert!(eval_variable(&Variable::Positive(1), &solution));
        assert!(!eval_variable(&Variable::Negative(1), &solution));
        assert!(!eval_variable(&Variable::Positive(2), &solution));
        assert!(eval_variable(&Variable::Negative(2), &solution));
    }

    #[test]
    fn test_satisfy_clause() {
        let solution = Solution::from([(1, true), (2, false)]);
        assert!(satisfy_clause(&vec![Variable::Positive(1)], &solution));
        assert!(!satisfy_clause(&vec![Variable::Positive(2)], &solution));
        assert!(satisfy_clause(&vec![Variable::Negative(2)], &solution));
        assert!(!satisfy_clause(&vec![Variable::Negative(1)], &solution));
        assert!(satisfy_clause(
            &vec![Variable::Positive(1), Variable::Negative(2)],
            &solution
        ));
        assert!(!satisfy_clause(
            &vec![Variable::Positive(2), Variable::Negative(1)],
            &solution
        ));
    }

    #[test]
    fn test_satisfy_formula() {
        let solution = Solution::from([(1, true), (2, false)]);
        assert!(satisfy_formula(
            &vec![vec![Variable::Positive(1)], vec![Variable::Negative(2)],],
            &solution
        ));
        assert!(!satisfy_formula(
            &vec![vec![Variable::Positive(1)], vec![Variable::Positive(2)],],
            &solution
        ));
        assert!(satisfy_formula(
            &vec![vec![Variable::Positive(1)], vec![Variable::Negative(2)]],
            &solution
        ));
        assert!(!satisfy_formula(
            &vec![vec![Variable::Positive(2)], vec![Variable::Positive(1)]],
            &solution
        ));
    }

    #[test]
    fn test_get_variables() {
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        let variables = get_variables(&formula);
        assert_eq!(variables, vec![1, 2, 3]);
    }
}
