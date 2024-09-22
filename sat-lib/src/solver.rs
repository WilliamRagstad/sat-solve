use crate::{
    solvers::Solver,
    types::{Formula, Solution},
};

/// Find all solutions to a SAT problem using a given solver.
/// Uses an exhaustive search with restart backtracking.
///
/// ## Note
/// This is a naive implementation that may not be efficient for large formulas with many variables and possible solutions.
pub fn solve_all<S: Solver>(formula: &Formula, solver: &S) -> Vec<Solution> {
    let mut formula = formula.clone();
    let mut solutions = Vec::new();
    let variables = formula.literals();
    let mut solution = Solution::new();

    while let Some(solution) = {
        solution.reset();
        solver.solve(&mut formula, &variables, &mut solution)
    } {
        solutions.push(solution.clone());
        // Remove that exact solution from the formula
        formula.add(solution.negative_clause());
    }
    solutions
}

#[cfg(test)]
mod tests {
    use crate::{printer::PrintStyle, solvers, types::Variable};

    use super::*;

    fn solvers() -> Vec<impl Solver> {
        vec![solvers::Dfs]
    }

    #[test]
    fn test_solve_sat_1() {
        // (x1 OR -x2) AND x3
        let formula: Formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ]
        .into();
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
        // There are multiple possible solutions:
        // - x1 = true, x2 = false, x3 = true
        // - x1 = true, x2 = true, x3 = true
        // - x1 = false, x2 = false, x3 = true
        // - x1 = true, x2 = false, x3 = true
        let possible_solutions: [Solution; 3] = [
            [(1, false), (2, false), (3, true)][..].into(),
            [(1, true), (2, false), (3, true)][..].into(),
            [(1, true), (2, true), (3, true)][..].into(),
        ];
        // Assert that each possible solution satisfies the formula
        for possible_solution in &possible_solutions {
            assert!(possible_solution.satisfy(&formula));
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
        let formula: Formula =
            vec![vec![Variable::Positive(1)], vec![Variable::Negative(1)]].into();
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
        let formula: Formula = vec![
            vec![Variable::Positive(1), Variable::Positive(2)],
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Negative(1), Variable::Positive(2)],
        ]
        .into();
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
        // There is only one possible solution:
        // - x1 = true, x2 = true
        let possible_solutions: [Solution; 1] = [[(1, true), (2, true)][..].into()];
        // Assert that each possible solution satisfies the formula
        for possible_solution in &possible_solutions {
            assert!(possible_solution.satisfy(&formula));
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
        let formula: Formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ]
        .into();
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
        // There are multiple possible solutions:
        // - x1 = true, x2 = false, x3 = true
        // - x1 = true, x2 = true, x3 = true
        // - x1 = false, x2 = false, x3 = true
        // - x1 = true, x2 = false, x3 = true
        let possible_solutions: [Solution; 3] = [
            [(1, false), (2, false), (3, true)][..].into(),
            [(1, true), (2, false), (3, true)][..].into(),
            [(1, true), (2, true), (3, true)][..].into(),
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
