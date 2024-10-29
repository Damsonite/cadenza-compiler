#[derive(Debug, PartialEq)]
pub enum Token {
    Note(String),     // Example: "c#", "d"
    Operator(char),   // Example: 'a', 'g'
}

// Split the input string into tokens
pub fn tokenize(input: &str) -> Vec<Token> {
    let operators = ["a", "g", "f", "e"]; // Plus, Minus, Times, Divide
    let mut tokens = Vec::new();

    for (i, word) in input.split_whitespace().enumerate() {
        if i == 0 && operators.contains(&word) {
            tokens.push(Token::Operator(word.chars().next().unwrap())); 
        } else {
            tokens.push(Token::Note(word.to_string()));
        }
    }

    tokens
}
