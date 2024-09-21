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

/// A list of variable identifier literals.
pub type Variables = Vec<Literal>;

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
pub type Solution = HashMap<Literal, bool>;
