use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Token {
    Note(String),    
    Operator(char),   
}

// Split the input string into tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let operators: HashSet<&str> = ["A", "G", "F", "E"].iter().cloned().collect();
    let mut tokens = Vec::new();

    for (i, word) in input.to_uppercase().split_whitespace().enumerate() {
        if i == 0 && operators.contains(&word) {
            tokens.push(Token::Operator(word.chars().next().unwrap())); 
        } else {
            tokens.push(Token::Note(word.to_string()));
        }
    }

    Ok(tokens)
}
