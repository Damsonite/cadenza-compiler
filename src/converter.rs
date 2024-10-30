use crate::parser::Expr;

pub const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

// Convert notes to a number
pub fn note_to_number(note: &str) -> Result<u32, String> {
    NOTES.iter()
        .position(|&n| n == note)
        .map(|pos| pos as u32)
        .ok_or_else(|| format!("Invalid note: {}", note))
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
            let data_type = match data_type.as_str() {
                "E" => "int",
                _ => return Err(format!("Unknown data type: {}", data_type)),
            };

            let number = note_to_number(&value)?;
            Ok(format!("{}: {} = {}", identifier, data_type, number))
        }
    }
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