use std::io::Write;

use crossterm_cursor::{cursor, TerminalCursor};
use types::Formula;

use crate::printer::PrintStyle;

mod parser;
mod printer;
mod solver;
mod solvers;
mod types;
mod utils;

fn main() {
    println!("Welcome to the SAT Solver!");
    let mut cursor = cursor();
    let solver = solvers::DFS;
    loop {
        let input = read_line(&mut cursor);
        let Some(formula) = parser::parse(&input) else {
            continue;
        };
        update_line(&input, &formula, &mut cursor);
        let solutions = solver::solve_all(&formula, &solver);
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
                PrintStyle::Programmatic.print_solution(solution);
            }
        } else {
            println!("Unsatisfiable");
        }
    }
}

fn read_line(cursor: &mut TerminalCursor) -> String {
    println!();
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    if let Err(err) = cursor.save_position() {
        eprintln!("Failed to save cursor position: {}", err);
    }
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn update_line(input: &str, formula: &Formula, cursor: &mut TerminalCursor) {
    if let Err(err) = cursor.restore_position() {
        eprintln!("Failed to restore cursor position: {}", err);
        return; // Skip the rest of the function
    }
    // Clear the current line with length of the input
    print!("{}", " ".repeat(input.len()));
    cursor.restore_position().unwrap();
    PrintStyle::Programmatic.print_formula(formula);
}
