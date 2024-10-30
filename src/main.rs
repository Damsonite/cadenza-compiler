use std::process::Command;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod lexer;
mod parser;
mod evaluator;

use crate::evaluator::{SymbolTable, evaluate, number_to_note};

fn process_input(input: &str, symbol_table: &mut SymbolTable) {
    match lexer::tokenize(&input) {
        Ok(tokens) => {
            match parser::parse(tokens) {
                Ok(expr) => {
                    match evaluator::expr_to_numbers(expr.clone()) {
                        Ok(numbers) => {
                            println!("Input: {}", input);
                            println!("Expression: {}", numbers)},
                        Err(e) => eprintln!("Conversion error: {}", e),
                    };

                    match evaluate(expr, symbol_table) {
                        Ok(result) => {
                            println!("Result: {}", result);
                            println!("Output: {}\n", number_to_note(result).unwrap());
                        },
                        Err(e) => eprintln!("Evaluation error: {}", e),
                    }
                }
                Err(e) => eprintln!("Parse error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexical error: {}", e),
    }
}

fn main() {
    Command::new("clear").status().expect("Failed to clear console");

    let mut symbol_table = evaluator::SymbolTable::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 { // FILE MODE
        let file_path = &args[1];
        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let input = line.expect("Failed to read line").trim().to_string();
            if input.is_empty() {
                eprintln!("Input is empty or invalid");
                continue;
            }

            process_input(&input, &mut symbol_table);
        }
    } else { // INTERACTIVE MODE
        loop {
            let mut input = String::new();
            println!("Enter an expression or type 'exit' to quit:");
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            if input == "exit" {
                break;
            }

            if input.is_empty() {
                eprintln!("Input is empty or invalid");
                continue;
            }

            process_input(&input, &mut symbol_table);
        }
    }
}