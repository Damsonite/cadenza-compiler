use crate::evaluator::{is_valid_datatype, is_valid_operator};

#[derive(Debug, PartialEq)]
pub enum Token {
    Declare,  
    Operator(char),
    Identifier(String),
    Type(char),
    Value(String),  
}

// Split the input string into tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    // Split the input string into tokens
    for (i, token) in input.to_uppercase().split_whitespace().enumerate() {
        if i == 0 {
            // Check if the first token is a declaration
            if token == "D" {
                tokens.push(Token::Declare);
            // Check if the first token is an operator
            } else if is_valid_operator(token) {
                tokens.push(Token::Operator(token.chars().next().unwrap()))
            }

            continue;
        }

        if tokens[0] == Token::Declare {
            if i == 1 {
                // Check if the second token is an identifier
                tokens.push(Token::Identifier(token.to_string()));
            } else if i == 2 && is_valid_datatype(token){
                tokens.push(Token::Type(token.chars().next().unwrap()));
            }

            continue;
        }

        tokens.push(Token::Value(token.to_string()));
    }
    Ok(tokens)
}
