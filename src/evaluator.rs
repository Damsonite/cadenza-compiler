use std::collections::HashMap;

pub fn notes_to_value(notes: &str) -> i32 {
    let notes = notes.to_uppercase();

    let note_map: HashMap<&str, i32> = [
        ("C", 0),
        ("C#", 1),
        ("D", 2),
        ("D#", 3),
        ("E", 4),
        ("F", 5),
        ("F#", 6),
        ("G", 7),
        ("G#", 8),
        ("A", 9),
        ("A#", 10),
        ("B", 11),
    ].iter().cloned().collect();

    let mut result = 0;
    let mut i = 0;

    while i < notes.len() {
        let note = if i + 1 < notes.len() && &notes[i+1..i+2] == "#" {
            &notes[i..i+2]
        } else {
            &notes[i..i+1]
        };
        result = result * 12 + note_map[note];
        i += note.len();
        };

        result
    }