use std::collections::HashMap;

/// A literal is a identifier of a variable.
pub type Literal = u32;

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
    Positive(Literal),
    Negative(Literal),
}

/// A clause is a disjunction of variables.
///
/// ## Examples
/// ```plaintext
///  x1 OR x2 OR  x3
/// -x1 OR x2 OR -x3
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Clause(pub(crate) Vec<Variable>);

impl Clause {
    /// Get all literal variables in the clause.
    pub fn literals(&self) -> Vec<Literal> {
        let mut variables = Vec::new();
        for variable in &self.0 {
            match variable {
                Variable::Positive(id) | Variable::Negative(id) => {
                    variables.push(*id);
                }
            }
        }
        variables.sort();
        variables.dedup();
        variables
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Variable> {
        self.0.iter()
    }
}

/// A boolean propositional formula in conjunctive normal form (CNF).
///
/// ## Examples
/// ```plaintext
/// ( x1 OR -x2) AND x3
/// (-x1 OR  x2) AND (x1 OR -x2) AND (-x3 OR x1)
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Formula(pub(crate) Vec<Clause>);

impl Formula {
    /// Create a new empty formula.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Add a new clause to the formula.
    pub fn add(&mut self, clause: Clause) {
        self.0.push(clause);
    }

    /// Get all literal variables in the formula.
    pub fn literals(&self) -> Vec<Literal> {
        let mut variables = Vec::new();
        for clause in &self.0 {
            variables.extend(clause.literals());
        }
        variables.sort();
        variables.dedup();
        variables
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<Clause> {
        self.0.iter()
    }
}

impl From<Vec<Vec<Variable>>> for Formula {
    fn from(clauses: Vec<Vec<Variable>>) -> Self {
        let mut formula = Formula::new();
        for clause in clauses {
            formula.add(Clause(clause));
        }
        formula
    }
}

/// A solution is a mapping of variables to truth values.
///
/// ## Examples
/// ```plaintext
/// x1 = true, x2 = false, x3 = true
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Solution(pub(crate) HashMap<Literal, bool>);

impl Solution {
    /// Create a new empty solution.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Check if a variable is assigned a truth value.
    pub fn get(&self, id: Literal) -> bool {
        if let Some(value) = self.0.get(&id) {
            *value
        } else {
            panic!("Variable x{} is not assigned a value", id);
        }
    }

    /// Assign a truth value to a variable.
    pub fn set(&mut self, id: Literal, value: bool) {
        self.0.insert(id, value);
    }

    pub fn literals(&self) -> Vec<Literal> {
        let mut variables = self.0.keys().copied().collect::<Vec<_>>();
        variables.sort();
        variables
    }

    /// Set all variables to `false`.
    pub fn reset(&mut self) {
        self.0.iter_mut().for_each(|(_, value)| *value = false);
    }

    pub fn satisfy(&self, formula: &Formula) -> bool {
        for clause in &formula.0 {
            if !clause.0.iter().any(|variable| match variable {
                Variable::Positive(id) => self.get(*id),
                Variable::Negative(id) => !self.get(*id),
            }) {
                return false;
            }
        }
        true
    }

    /// When a new solution is found, Add a new clause that forbids that exact solution. \
    /// Done using **De Morgan's Laws**:
    /// ```plaintext
    /// -(x1 AND x2 ... AND xN)  =>  (-x1 OR -x2 OR ... OR -xN)
    /// ```
    pub fn negative_clause(&self) -> Clause {
        let mut clause = Vec::new();
        for (id, value) in &self.0 {
            // if !value {
            //     clause.push(Variable::Positive(*id));
            // }
            if *value {
                clause.push(Variable::Negative(*id));
            } else {
                clause.push(Variable::Positive(*id));
            }
        }
        Clause(clause)
    }
}

/// Create a new solution from a list of pairs of literals and truth values.
impl From<&[(Literal, bool)]> for Solution {
    fn from(pairs: &[(Literal, bool)]) -> Self {
        let mut solution = Solution::new();
        for (id, value) in pairs {
            solution.set(*id, *value);
        }
        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula_get_literals() {
        let formula = Formula(vec![
            Clause(vec![Variable::Positive(1), Variable::Negative(2)]),
            Clause(vec![Variable::Positive(2), Variable::Negative(3)]),
        ]);
        assert_eq!(formula.literals(), vec![1, 2, 3]);
    }

    #[test]
    fn test_solution_satisfy() {
        let formula = Formula(vec![
            Clause(vec![Variable::Positive(1), Variable::Negative(2)]),
            Clause(vec![Variable::Positive(2), Variable::Negative(3)]),
        ]);
        let solution: Solution = ([(1u32, true), (2u32, false), (3u32, true)][..]).into();
        assert!(solution.satisfy(&formula));
    }
}
