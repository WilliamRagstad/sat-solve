use std::io::Write;

use crossterm::{
    style::{Attribute, Color, SetAttribute, SetForegroundColor},
    ExecutableCommand,
};
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
    let mut stdout = std::io::stdout();
    let mut solver = solvers::Dfs;
    let mut style = PrintStyle::Normal;
    loop {
        let (input, start) = read_line(&mut cursor);
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
                println!("  dfs      Use depth-first search (DFS) brute-force solver (default)");
                println!("  math     Use mathematical notation");
                println!("  normal   Use normal notation");
                println!("  prog     Use programmatic notation");
                println!("  help     Display this help message");
                println!("  exit     Exit the program");
            }
            expr => {
                let Some(formula) = parser::parse(expr) else {
                    continue;
                };
                update_line(&input, start, &formula, &mut cursor, &style);
                let solutions = solver::solve_all(&formula, &solver);
                if !solutions.is_empty() {
                    stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                    stdout.execute(SetAttribute(Attribute::Italic)).unwrap();
                    print!("\n  Satisfiable");
                    if solutions.len() > 1 {
                        print!(" ({})", solutions.len());
                        println!(": ");
                    } else {
                        print!(": ");
                    }
                    stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
                    stdout.execute(SetAttribute(Attribute::Reset)).unwrap();
                    for solution in &solutions {
                        if solutions.len() > 1 {
                            print!("  ");
                        }
                        style.print_solution(solution);
                    }
                } else {
                    stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
                    stdout.execute(SetAttribute(Attribute::Italic)).unwrap();
                    println!("\n  Unsatisfiable");
                    stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
                    stdout.execute(SetAttribute(Attribute::Reset)).unwrap();
                }
            }
        }
    }
}

fn read_line(cursor: &mut TerminalCursor) -> (String, (u16, u16)) {
    println!();
    let mut input = String::new();
    print!("> ");
    std::io::stdout().flush().unwrap();
    let start = cursor.pos().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    (input, start)
}

fn update_line(
    input: &str,
    start: (u16, u16),
    formula: &Formula,
    cursor: &mut TerminalCursor,
    style: &PrintStyle,
) {
    cursor.goto(start.0, start.1 - 1).unwrap();
    // Clear the current line with length of the input
    print!("{}", " ".repeat(input.len()));
    std::io::stdout().flush().unwrap();
    cursor.goto(start.0, start.1 - 1).unwrap();
    style.print_formula(formula);
    std::io::stdout().flush().unwrap();
}
