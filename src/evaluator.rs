use crate::parser::Expr;

const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
const OPERATORS: [&str; 4] = ["A", "G", "F", "E"];

// Validate a note
pub fn is_valid_note(note: &str) -> bool {
    NOTES.iter().any(|&n| n == note)
}

// Validate an operator
pub fn is_valid_operator(operator: &str) -> bool {
    OPERATORS.iter().any(|&op| op == operator)
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
pub fn evaluate(expr: Expr) -> Result<u32, String> {
    match expr {
        Expr::Operation { operator, operands } => {
            let operand_refs: Vec<&str> = operands.iter().map(|s| s.as_str()).collect();
            evaluate_operation(&operator, operand_refs)
            }
        }
 }