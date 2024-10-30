use crate::converter::{is_valid_operator, is_valid_datatype};

#[derive(Debug, PartialEq)]
pub enum Token {
    Declare,  
    Operator(char),
    Identifier(String),
    Type(char),
    Value(String),  
}

const DECLARATION_INDEX: usize = 0;
const IDENTIFIER_INDEX: usize = 1;
const DATATYPE_INDEX: usize = 2;
const VALUE_INDEX: usize = 3;

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

        // Check if the first token is a declaration
        if tokens[DECLARATION_INDEX] == Token::Declare {
            match  i {
                IDENTIFIER_INDEX => {
                    // Check if the second token is an identifier
                    tokens.push(Token::Identifier(token.to_string()));
                }
                DATATYPE_INDEX if is_valid_datatype(token) => {
                    // Check if the third token is a data type
                    tokens.push(Token::Type(token.chars().next().unwrap()));
                }
                VALUE_INDEX => {
                    // Check if the third token is a value
                    tokens.push(Token::Value(token.to_string()));
                }
                _ => {}
            }
        } else {
            tokens.push(Token::Value(token.to_string()));
        }
    }
    Ok(tokens)
}
