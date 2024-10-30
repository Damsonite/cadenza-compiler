use crate::converter::NOTES;

const OPERATORS: [&str; 4] = ["A", "G", "F", "E"];
const DATATYPES: [&str; 1] = ["E"];

// Validate a note
fn is_valid_note(note: &str) -> bool {
    NOTES.iter().any(|&n| n == note)
}

// Validate a sequence of notes
pub fn is_valid_notes(notes: &str) -> bool {
    notes.split_whitespace().all(|note| is_valid_note(note))
}

// Validate an operator
pub fn is_valid_operator(operator: &str) -> bool {
    OPERATORS.iter().any(|&op| op == operator)
}

// Validate a datatype
pub fn is_valid_datatype(datatype: &str) -> bool {
    DATATYPES.iter().any(|&dt| dt == datatype)
}