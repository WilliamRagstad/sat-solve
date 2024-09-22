use crate::{
    solvers::Solver,
    types::{Formula, Solution, Variable},
    utils::get_variables,
};

pub fn solve_all<S: Solver>(formula: &Formula, solver: &S) -> Vec<Solution> {
    let mut formula = formula.clone();
    let mut solutions = Vec::new();
    let variables = get_variables(&formula);
    let mut solution = Solution::new();

    while let Some(solution) = {
        // Initialize all variables to `false`
        for variable in &variables {
            solution.insert(*variable, false);
        }
        // Find a solution using the solver
        solver.solve(&mut formula, &variables, &mut solution)
    } {
        // Add the solution to the list of solutions
        solutions.push(solution.clone());
        // Remove that exact solution from the formula
        remove_solution(&mut formula, &solution);
    }
    solutions
}

/// When a solution is found, remove it from the formula by adding a new clause that forbids it. \
/// Done using **De Morgan's Laws**:
/// ```plaintext
/// -(x1 AND x2 ... AND xN)  =>  (-x1 OR -x2 OR ... OR -xN)
/// ```
fn remove_solution(formula: &mut Formula, solution: &Solution) {
    let mut clause = Vec::new();
    for (id, value) in solution {
        if *value {
            clause.push(Variable::Negative(*id));
        } else {
            clause.push(Variable::Positive(*id));
        }
    }
    formula.push(clause);
}

#[cfg(test)]
mod tests {
    use crate::{printer::PrintStyle, solvers, utils::satisfy_formula};

    use super::*;

    fn solvers() -> Vec<impl Solver> {
        vec![solvers::DFS]
    }

    #[test]
    fn test_solve_sat_1() {
        // (x1 OR -x2) AND x3
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
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

        for solver in solvers() {
            // Find a solution using the solver
            let solutions = solve_all(&formula, &solver);
            // Assert that the solutions are the same as the possible solutions
            assert_eq!(solutions.len(), possible_solutions.len());
            println!("Solutions:");
            for solution in &solutions {
                assert!(possible_solutions.contains(solution));
                PrintStyle::Normal.print_solution(solution);
            }
        }
    }

    #[test]
    fn test_solve_unsat_1() {
        // (x1 OR x2) AND (-x1 OR -x2)
        let formula = vec![vec![Variable::Positive(1)], vec![Variable::Negative(1)]];
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
        for solver in solvers() {
            // There is no solution that satisfies the formula
            let solutions = solve_all(&formula, &solver);
            assert!(solutions.is_empty());
            println!("Solution: Unsatisfiable");
        }
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
        PrintStyle::Normal.print_formula(&formula);
        // There is only one possible solution:
        // - x1 = true, x2 = true
        let possible_solutions = [Solution::from([(1, true), (2, true)])];
        // Assert that each possible solution satisfies the formula
        for possible_solution in &possible_solutions {
            assert!(satisfy_formula(&formula, possible_solution));
        }
        for solver in solvers() {
            // Find a solution using the solver
            let solutions = solve_all(&formula, &solver);
            // Assert that the solutions are the same as the possible solutions
            assert_eq!(solutions.len(), possible_solutions.len());
            println!("Solutions:");
            for solution in &solutions {
                assert!(possible_solutions.contains(solution));
                PrintStyle::Normal.print_solution(solution);
            }
        }
    }

    #[test]
    fn test_solve_all_sat_1() {
        // (x1 OR -x2) AND x3
        let formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ];
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
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
        for solver in solvers() {
            // Find all solutions using the solver
            let solutions = solve_all(&formula, &solver);
            // Assert that the solutions are the same as the possible solutions
            assert_eq!(solutions.len(), possible_solutions.len());
            println!("Solutions:");
            for solution in &solutions {
                assert!(possible_solutions.contains(solution));
                PrintStyle::Normal.print_solution(solution);
            }
        }
    }
}
