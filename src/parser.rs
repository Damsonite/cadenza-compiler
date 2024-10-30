use crate::lexer::Token;
use crate::evaluator::{is_valid_operator, is_valid_note};

#[derive(Debug, Clone)]
pub enum Expr {
    Operation {
        operator: String,
        operands: Vec<String>,
    },
}

// Parse a list of tokens into an expression
pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = tokens.into_iter();

    // Verify that the expression starts with an operator
    let operator = match tokens.next() {
        Some(Token::Operator(op)) if is_valid_operator(&op.to_string()) => op,
        Some(Token::Operator(op)) => {
            return Err(format!("Semantic error: '{}' is not a valid operator.", op));
        }
        _ => return Err("Syntax error: Expression must start with an operator.".into()),
    };

    // Parse the operands
    let mut operands = Vec::new();
    for token in tokens {
        match token {
            Token::Note(note) => {
                if !is_valid_note(&note) {
                    return Err(format!("Semantic error: Note '{}' is out of range.", note));
                }
                operands.push(note)
            },
            _ => return Err("Syntax error: Expected a note as an operand.".into()),
        }
    }

    // Verify that the expression has at least one operand
    if operands.is_empty() {
        return Err("Syntax error: Operator must be followed by at least one note.".into());
    }

    Ok(Expr::Operation { operator: operator.to_string(), operands })
}