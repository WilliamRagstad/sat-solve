use crate::types::{Clause, Formula, Literal, Solution, Variable};
use crossterm::{
    style::{Attribute, Color, SetAttribute, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrintStyle {
    #[default]
    Normal,
    Programmatic,
    Mathematical,
}

impl PrintStyle {
    fn neg_sign(&self) -> &'static str {
        match self {
            PrintStyle::Normal => "-",
            PrintStyle::Programmatic => "!",
            PrintStyle::Mathematical => "¬",
        }
    }

    fn bool_style(&self, value: bool) -> &'static str {
        match self {
            PrintStyle::Programmatic => {
                if value {
                    "1"
                } else {
                    "0"
                }
            }
            PrintStyle::Normal => {
                if value {
                    "T"
                } else {
                    "F"
                }
            }
            PrintStyle::Mathematical => {
                if value {
                    "⊤"
                } else {
                    "⊥"
                }
            }
        }
    }

    fn lit_style(&self, id: Literal) -> String {
        match self {
            PrintStyle::Mathematical => {
                format!(
                    "𝑋{}",
                    id.to_string()
                        .chars()
                        .map(|c| char::from_u32('₀' as u32 + c.to_digit(10).unwrap()).unwrap())
                        .collect::<String>()
                )
            }
            _ => format!("X{}", id),
        }
    }

    fn and_style(&self) -> &'static str {
        match self {
            PrintStyle::Mathematical => "∧",
            PrintStyle::Normal => "and",
            PrintStyle::Programmatic => "&",
        }
    }

    fn or_style(&self) -> &'static str {
        match self {
            PrintStyle::Mathematical => "∨",
            PrintStyle::Normal => "or",
            PrintStyle::Programmatic => "|",
        }
    }

    pub fn print_variable(&self, variable: &Variable) {
        let mut stdout = std::io::stdout();
        let (id, positive) = match variable {
            Variable::Positive(id) => {
                stdout.execute(SetForegroundColor(Color::Green)).unwrap();
                (id, true)
            }
            Variable::Negative(id) => {
                stdout.execute(SetForegroundColor(Color::Red)).unwrap();
                (id, false)
            }
        };
        if !positive {
            print!("{}", self.neg_sign());
        }
        print!("{}", self.lit_style(*id));
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
    }

    pub fn print_clause(&self, clause: &Clause) {
        let mut stdout = std::io::stdout();
        stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
        print!("(");
        for (i, variable) in clause.iter().enumerate() {
            self.print_variable(variable);
            if i < clause.len() - 1 {
                stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
                print!(" {} ", self.or_style());
            }
        }
        stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
        print!(")");
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
    }

    pub fn print_formula(&self, formula: &Formula) {
        let mut stdout = std::io::stdout();
        for (i, clause) in formula.iter().enumerate() {
            self.print_clause(clause);
            if i < formula.len() - 1 {
                stdout.execute(SetForegroundColor(Color::Yellow)).unwrap();
                print!(" {} ", self.and_style());
            }
        }
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
        println!();
    }

    pub fn print_solution(&self, solution: &Solution) {
        let mut stdout = std::io::stdout();
        let literals = solution.literals();
        for i in 0..literals.len() {
            let id = literals[i];
            stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
            stdout.execute(SetAttribute(Attribute::Bold)).unwrap();
            print!("{}", self.lit_style(id));
            stdout.execute(SetAttribute(Attribute::Reset)).unwrap();
            stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
            print!(" = ");
            stdout
                .execute(SetForegroundColor(if solution.get(id) {
                    Color::Green
                } else {
                    Color::Red
                }))
                .unwrap();
            print!("{}", self.bool_style(solution.get(id)));
            if i < literals.len() - 1 {
                stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                print!(", ");
            }
        }
        println!();
        stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit_style() {
        assert_eq!(PrintStyle::Normal.lit_style(1), "X1");
        assert_eq!(PrintStyle::Programmatic.lit_style(1), "X1");
        assert_eq!(PrintStyle::Mathematical.lit_style(1), "𝑋₁");
        assert_eq!(PrintStyle::Normal.lit_style(10), "X10");
        assert_eq!(PrintStyle::Programmatic.lit_style(10), "X10");
        assert_eq!(PrintStyle::Mathematical.lit_style(10), "𝑋₁₀");
        assert_eq!(PrintStyle::Normal.lit_style(100), "X100");
        assert_eq!(PrintStyle::Programmatic.lit_style(100), "X100");
        assert_eq!(PrintStyle::Mathematical.lit_style(100), "𝑋₁₀₀");
    }

    #[test]
    fn test_print_variable() {
        let variable = Variable::Positive(1);
        print!("Variable: ");
        PrintStyle::Normal.print_variable(&variable);
        PrintStyle::Programmatic.print_variable(&variable);
        PrintStyle::Mathematical.print_variable(&variable);
    }

    #[test]
    fn test_print_formula() {
        let formula: Formula = vec![
            vec![Variable::Positive(1), Variable::Negative(2)],
            vec![Variable::Positive(3)],
        ]
        .into();
        print!("Formula: ");
        PrintStyle::Normal.print_formula(&formula);
        PrintStyle::Programmatic.print_formula(&formula);
        PrintStyle::Mathematical.print_formula(&formula);
    }

    #[test]
    fn test_print_solution() {
        let solution = [(1, true), (2, false), (3, true)][..].into();
        print!("Solution: ");
        PrintStyle::Normal.print_solution(&solution);
        PrintStyle::Programmatic.print_solution(&solution);
        PrintStyle::Mathematical.print_solution(&solution);
    }
}
