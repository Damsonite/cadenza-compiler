use std::process::Command;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod processor;
mod lexer;
mod parser;
mod validator;
mod converter;
mod evaluator;

fn main() {
    Command::new("clear").status().expect("Failed to clear console");

    let args: Vec<String> = env::args().collect();
    let mut symbol_table = evaluator::SymbolTable::new();

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

            processor::process(&input, &mut symbol_table);
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

            processor::process(&input, &mut symbol_table);
        }
    }
}