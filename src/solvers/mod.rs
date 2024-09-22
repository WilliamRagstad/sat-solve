use crate::types::{Formula, Literal, Solution};

mod dfs;
pub use dfs::Dfs;

/// A SAT solver is a program that determines whether a given boolean formula is satisfiable.
/// - If the formula is satisfiable, the solver returns `Some(solution)`.
/// - If the formula is unsatisfiable, the solver returns `None`.
pub trait Solver {
    fn solve(
        &self,
        formula: &mut Formula,
        variables: &[Literal],
        solution: &mut Solution,
    ) -> Option<Solution>;
}
