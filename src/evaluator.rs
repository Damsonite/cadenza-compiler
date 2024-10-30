use std::collections::HashMap;

use crate::parser::Expr;

pub struct SymbolTable {
    variables: HashMap<String, u32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
        }
    }

    pub fn declare(&mut self, identifier: String, value: u32) {
        self.variables.insert(identifier, value);
    }

/*     pub fn get(&self, identifier: &str) -> Option<&u32> {
        self.variables.get(identifier)
    } */
}

const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const OPERATORS: [&str; 4] = ["A", "G", "F", "E"];
const DATATYPES: [&str; 2] = ["E", "G"];


// Validate a note
pub fn is_valid_note(note: &str) -> bool {
    NOTES.iter().any(|&n| n == note)
}

// Validate an operator
pub fn is_valid_operator(operator: &str) -> bool {
    OPERATORS.iter().any(|&op| op == operator)
}

// Validate a datatype
pub fn is_valid_datatype(datatype: &str) -> bool {
    DATATYPES.iter().any(|&dt| dt == datatype)
}

// Convert notes to a number
fn note_to_number(note: &str) -> Result<u32, String> {
    NOTES.iter()
        .position(|&n| n == note)
        .map(|pos| pos as u32)
        .ok_or_else(|| format!("Invalid note: {}", note))
}

// Convert a number to a note sequence
pub fn number_to_note(mut number: u32) -> Result<String, String> {
    if number == 0 {
        return Ok(NOTES[0].to_string());
    } 
    
    let mut result = String::new();
    
    // Build the note representation by repeatedly dividing by 12
    while number > 0 {
        let index = (number % 12) as usize;
        result = format!("{}{}", NOTES[index], result);
        number /= 12;
    }
    
    Ok(result)
}


// Convert an expression to a string with numbers
pub fn expr_to_numbers(expr: Expr) -> Result<String, String> {
    match expr {
        Expr::Operation { operator, operands } => {
            let mut result = String::new();
            for (i, operand) in operands.iter().enumerate() {
                let number = note_to_number(operand)?;
                if i > 0 {
                    result.push_str(match operator.as_str() {
                        "A" => " + ",
                        "G" => " - ",
                        "F" => " * ",
                        "E" => " / ",
                        _ => return Err("Unknown operator".into()),
                    });
                }
                result.push_str(&number.to_string());
            }
            Ok(result)
        },
        Expr::Declaration { identifier, data_type, value } => {
            let number = note_to_number(&value)?;
            Ok(format!("{}: {} = {}", identifier, data_type, number))
        }
    }
}

// Evaluate an operation with a given operator and operands
fn evaluate_operation(operator: &str, operands: Vec<&str>) -> Result<u32, String> {
    let mut result = note_to_number(operands[0])?; 

    for operand in operands.iter().skip(1) {
        let value = note_to_number(operand)?;
        result = match operator {
            "A" => result + value,
            "G" => result - value,
            "F" => result * value,
            "E" => result / value,
            _ => return Err("Unknown operator".into()),
        };
    }

    Ok(result)
}

// Evaluate an expression
pub fn evaluate(expr: Expr, symbol_table: &mut SymbolTable) -> Result<u32, String> {
    match expr {
        Expr::Operation { operator, operands } => {
            let operand_refs: Vec<&str> = operands.iter().map(|s| s.as_str()).collect();
            evaluate_operation(&operator, operand_refs)
        },
        Expr::Declaration { identifier, data_type: _, value } => {
            let number = note_to_number(&value)?;
            symbol_table.declare(identifier, number);
            Ok(number)
        }
    }
 }