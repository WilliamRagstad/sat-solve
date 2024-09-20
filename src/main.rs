use std::io::Write;

use crate::solver::{print_formula, print_solution};

mod parser;
mod solver;

fn main() {
    println!("Welcome to the SAT Solver!");
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let Some(formula) = parser::parse(&input) else {
            println!();
            continue;
        };
        print!("Formula: ");
        print_formula(&formula);
        let solutions = solver::solve_all(&formula);
        if !solutions.is_empty() {
            if solutions.len() == 1 {
                print!("Satisfiable: ");
            } else {
                println!("Satisfiable ({} solutions): ", solutions.len());
            }
            for solution in &solutions {
                if solutions.len() > 1 {
                    print!("  ");
                }
                print_solution(solution);
            }
        } else {
            println!("Unsatisfiable");
        }
        println!();
    }
}
