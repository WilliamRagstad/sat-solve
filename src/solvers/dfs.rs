use crate::types::{Formula, Literal, Solution};

use super::Solver;

pub fn brute_force(formula: &Formula, literals: &[Literal], solution: &mut Solution) -> bool {
    if literals.is_empty() {
        return solution.satisfy(formula);
    }
    let lit = literals[0];
    // First, set the literal to false
    solution.set(lit, false);
    if brute_force(formula, &literals[1..], solution) {
        return true;
    }
    // Then, set the literal to true
    solution.set(lit, true);
    if brute_force(formula, &literals[1..], solution) {
        return true;
    }
    false
}

/// A depth-first search (DFS) solver for the SAT problem. \
/// The solver uses brute force to find a solution.
pub struct Dfs;

impl Solver for Dfs {
    fn solve(
        &self,
        formula: &mut Formula,
        variables: &[Literal],
        solution: &mut Solution,
    ) -> Option<Solution> {
        if brute_force(formula, variables, solution) {
            Some(solution.clone())
        } else {
            None
        }
    }
}
