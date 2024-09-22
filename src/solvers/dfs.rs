use crate::{
    types::{Formula, Literal, Solution},
    utils::satisfy_formula,
};

use super::Solver;

pub fn brute_force(formula: &Formula, variables: &[Literal], solution: &mut Solution) -> bool {
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

/// A depth-first search (DFS) solver for the SAT problem. \
/// The solver uses brute force to find a solution.
pub struct DFS;

impl Solver for DFS {
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
