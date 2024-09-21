use crate::types::{Formula, Variable};

const AND: &str = "and";
const OR: &str = "or";
const LIT: &str = "x";

/// Parse a string into a formula.
///
/// ## Examples
/// ```plaintext
/// (x1 OR x2) AND (-x2 OR x3) AND (x1 OR -x3)
/// ```
/// Into:
/// ```rust
/// vec![
///    vec![Variable::Positive(1), Variable::Positive(2)],
///    vec![Variable::Negative(2), Variable::Positive(3)],
///    vec![Variable::Positive(1), Variable::Negative(3)],
/// ]
/// ```
pub fn parse(input: &str) -> Option<Formula> {
    let mut formula = Formula::new();
    let input = input.to_lowercase();
    for clause in input.split(AND) {
        let mut variables = Vec::new();
        let clause = clause.trim().trim_start_matches('(').trim_end_matches(')');
        for variable in clause.split(OR) {
            let variable = variable.trim();
            if variable.starts_with('-') {
                variables.push(Variable::Negative(parse_literal(
                    variable.trim_start_matches("-"),
                )?));
            } else {
                variables.push(Variable::Positive(parse_literal(variable)?));
            }
        }
        formula.push(variables);
    }
    Some(formula)
}

fn parse_literal(literal: &str) -> Option<u32> {
    if literal.trim().is_empty() {
        eprintln!("Missing variable!");
        return None;
    }
    if let Some(num) = literal.trim().strip_prefix(LIT) {
        if let Ok(num) = num.parse() {
            Some(num)
        } else {
            eprintln!("Invalid variable: {}, expected a number", literal);
            None
        }
    } else {
        eprintln!("Invalid variable: {}, expected xN", literal);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "(x1 OR x2) AND (-x2 OR x3) AND (x1 OR -x3)";
        let expected = vec![
            vec![Variable::Positive(1), Variable::Positive(2)],
            vec![Variable::Negative(2), Variable::Positive(3)],
            vec![Variable::Positive(1), Variable::Negative(3)],
        ];
        assert_eq!(parse(input), Some(expected));
    }

    #[test]
    fn test_parse_literal() {
        assert_eq!(parse_literal("x1"), Some(1));
        assert_eq!(parse_literal("x2"), Some(2));
        assert_eq!(parse_literal("x3"), Some(3));
    }
}
