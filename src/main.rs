use std::io::Write;

use crossterm_cursor::{cursor, TerminalCursor};
use types::Formula;

use crate::printer::PrintStyle;

mod parser;
mod printer;
mod solver;
mod solvers;
mod types;

fn main() {
    println!("Welcome to the SAT Solver!");
    let mut cursor = cursor();
    let mut solver = solvers::Dfs;
    let mut style = PrintStyle::Normal;
    loop {
        let input = read_line(&mut cursor);
        match input.trim() {
            "" => (),
            "exit" => break,
            "math" => {
                style = PrintStyle::Mathematical;
                println!("OK");
            }
            "normal" => {
                style = PrintStyle::Normal;
                println!("OK");
            }
            "prog" => {
                style = PrintStyle::Programmatic;
                println!("OK");
            }
            "dfs" => {
                solver = solvers::Dfs;
                println!("OK");
            }
            "help" => {
                println!("Commands:");
                println!("  dfs: Use depth-first search (DFS) brute-force solver");
                println!("  math: Use mathematical notation");
                println!("  normal: Use normal notation");
                println!("  prog: Use programmatic notation");
                println!("  help: Display this help message");
                println!("  exit: Exit the program");
            }
            expr => {
                let Some(formula) = parser::parse(expr) else {
                    continue;
                };
                update_line(&input, &formula, &mut cursor, &style);
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
                        style.print_solution(solution);
                    }
                } else {
                    println!("Unsatisfiable");
                }
            }
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

fn update_line(input: &str, formula: &Formula, cursor: &mut TerminalCursor, style: &PrintStyle) {
    if let Err(err) = cursor.restore_position() {
        eprintln!("Failed to restore cursor position: {}", err);
        return; // Skip the rest of the function
    }
    // Clear the current line with length of the input
    print!("{}", " ".repeat(input.len()));
    cursor.restore_position().unwrap();
    style.print_formula(formula);
}
