use crate::lexer::tokenize;
use crate::parser::parse;
use crate::converter::{expr_to_numbers, number_to_note};
use crate::evaluator::{SymbolTable, evaluate};

pub fn process(input: &str, symbol_table: &mut SymbolTable) {
    match tokenize(&input) {
        Ok(tokens) => {
            match parse(tokens) {
                Ok(expr) => {
                    match expr_to_numbers(expr.clone()) {
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