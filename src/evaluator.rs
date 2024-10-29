const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

// Convert a string of notes to a value
pub fn notes_to_value(notes: &str) -> i32 {
    let notes = notes.to_uppercase();
    let mut result = 0;
    
    let mut i = 0;
    while i < notes.len() {
        let note = if i + 1 < notes.len() && &notes[i+1..i+2] == "#" {
            &notes[i..i+2]
        } else {
            &notes[i..i+1]
        };
        
        result = result * 12 + NOTES.iter().position(|&n| n == note).unwrap() as i32;
        i += note.len();
        };

        result
    }