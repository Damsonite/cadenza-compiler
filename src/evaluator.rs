use std::collections::HashMap;

use crate::parser::Expr;
use crate::converter::note_to_number;


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