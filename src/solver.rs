use std::collections::HashMap;

/// A variable is a symbol that can be assigned a truth value.
/// It can be either positive or negative.
///
/// ## Examples
/// ```plaintext
/// x1
/// -x2
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Variable {
    Positive(u32),
    Negative(u32),
}

/// A clause is a disjunction of variables.
///
/// ## Examples
/// ```plaintext
///  x1 OR x2 OR  x3
/// -x1 OR x2 OR -x3
/// ```
pub type Clause = Vec<Variable>;

/// A boolean propositional formula in conjunctive normal form (CNF).
///
/// ## Examples
/// ```plaintext
/// ( x1 OR -x2) AND x3
/// (-x1 OR  x2) AND (x1 OR -x2) AND (-x3 OR x1)
/// ```
pub type Formula = Vec<Clause>;

/// A solution is a mapping of variables to truth values.
///
/// ## Examples
/// ```plaintext
/// x1 = true, x2 = false, x3 = true
/// ```
pub type Solution = HashMap<u32, bool>;

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

pub fn get_variables(formula: &Formula) -> Vec<u32> {
    let mut variables = Vec::new();
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

pub fn solve_all(formula: &Formula) -> Vec<Solution> {
    let mut formula = formula.clone();
    let mut solutions = Vec::new();
    while let Some(solution) = solve(&formula) {
        solutions.push(solution.clone());
        // Remove the solution from the formula by adding a new clause that forbids it
        // -(x1 AND x2 ... AND xN) = (-x1 OR -x2 OR ... OR -xN)
        let mut clause = Vec::new();
        for (id, value) in &solution {
            if *value {
                clause.push(Variable::Negative(*id));
            } else {
                clause.push(Variable::Positive(*id));
            }
        }
        formula.push(clause);
    }
    solutions
}

/// A SAT solver is a program that determines whether a given boolean formula is satisfiable.
/// - If the formula is satisfiable, the solver returns `Some(solution)`.
/// - If the formula is unsatisfiable, the solver returns `None`.
pub fn solve(formula: &Formula) -> Option<Solution> {
    let variables = get_variables(formula);
    let mut solution = Solution::new();
    for variable in &variables {
        solution.insert(*variable, false); // Initialize all variables to `false`
    }
    // Go through all permutations of the variables
    if solvers::brute_force(formula, &variables, &mut solution) {
        Some(solution)
    } else {
        None
    }
}

mod solvers {
    use super::*;

    pub fn brute_force(formula: &Formula, variables: &[u32], solution: &mut Solution) -> bool {
        if variables.is_empty() {
            return satisfy_formula(formula, solution);
        }
        let variable = variables[0];
        let mut remaining_variables = Vec::from(variables);
        remaining_variables.remove(0);
        solution.insert(variable, false);
        if brute_force(formula, &remaining_variables, solution) {
            return true;
        }
        solution.insert(variable, true);
        if brute_force(formula, &remaining_variables, solution) {
            return true;
        }
        solution.remove(&variable);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_variable() {
        let solution = HashMap::from([(1, true), (2, false)]);
        assert!(eval_variable(&Variable::Positive(1), &solution));
        assert!(!eval_variable(&Variable::Negative(1), &solution));
        assert!(!eval_variable(&Variable::Positive(2), &solution));
        assert!(eval_variable(&Variable::Negative(2), &solution));
    }

    #[test]
    fn test_satisfy_clause() {
        let solution = HashMap::from([(1, true), (2, false)]);
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
        let solution = HashMap::from([(1, true), (2, false)]);
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
    fn test_solve_sat_1() {
        // (x1 OR -x2) AND x3
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        print!("Formula: ");
        print_formula(&formula);
        // There are multiple possible solutions:
        // - x1 = true, x2 = false, x3 = true
        // - x1 = true, x2 = true, x3 = true
        // - x1 = false, x2 = false, x3 = true
        // - x1 = true, x2 = false, x3 = true
        let possible_solutions = [
            Solution::from([(1, false), (2, false), (3, true)]),
            Solution::from([(1, true), (2, false), (3, true)]),
            Solution::from([(1, true), (2, true), (3, true)]),
        ];
        // Assert that each possible solution satisfies the formula
        for possible_solution in &possible_solutions {
            assert!(satisfy_formula(&formula, possible_solution));
        }
        // Find a solution using the solver
        let solution = solve(&formula);
        // Assert that the solution is one of the possible solutions
        assert!(solution.is_some());
        let solution = solution.unwrap();
        print!("Solution: ");
        print_solution(&solution);
        assert!(possible_solutions.contains(&solution));
    }

    #[test]
    fn test_solve_unsat_1() {
        // (x1 OR x2) AND (-x1 OR -x2)
        let formula = vec![vec![Variable::Positive(1)], vec![Variable::Negative(1)]];
        print!("Formula: ");
        print_formula(&formula);
        // There is no solution that satisfies the formula
        let solution = solve(&formula);
        assert_eq!(solution, None);
        println!("Solution: Unsatisfiable");
    }

    #[test]
    fn test_solve_sat_2() {
        // (x1 OR x2) AND (x1 OR -x2) AND (-x1 OR x2)
        let formula = vec![
            vec![Variable::Positive(1), Variable::Positive(2)],
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Negative(1), Variable::Positive(2)],
        ];
        print!("Formula: ");
        print_formula(&formula);
        // There is only one possible solution:
        // - x1 = true, x2 = true
        let possible_solutions = [Solution::from([(1, true), (2, true)])];
        // Assert that each possible solution satisfies the formula
        for possible_solution in &possible_solutions {
            assert!(satisfy_formula(&formula, possible_solution));
        }
        // Find a solution using the solver
        let solution = solve(&formula);
        // Assert that the solution is one of the possible solutions
        assert!(solution.is_some());
        let solution = solution.unwrap();
        print!("Solution: ");
        print_solution(&solution);
        assert!(possible_solutions.contains(&solution));
    }

    #[test]
    fn test_solve_all_sat_1() {
        // (x1 OR -x2) AND x3
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        print!("Formula: ");
        print_formula(&formula);
        // There are multiple possible solutions:
        // - x1 = true, x2 = false, x3 = true
        // - x1 = true, x2 = true, x3 = true
        // - x1 = false, x2 = false, x3 = true
        // - x1 = true, x2 = false, x3 = true
        let possible_solutions = [
            Solution::from([(1, false), (2, false), (3, true)]),
            Solution::from([(1, true), (2, false), (3, true)]),
            Solution::from([(1, true), (2, true), (3, true)]),
        ];
        // Find all solutions using the solver
        let solutions = solve_all(&formula);
        // Assert that the solutions are the same as the possible solutions
        assert_eq!(solutions.len(), possible_solutions.len());
        println!("Solutions:");
        for solution in &solutions {
            assert!(possible_solutions.contains(solution));
            print_solution(solution);
        }
    }
}
