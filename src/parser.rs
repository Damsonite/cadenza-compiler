use crate::lexer::Token::{self, Value, Operator};
use crate::evaluator::{is_valid_operator, is_valid_note};

#[derive(Debug, Clone)]
pub enum Expr {
    Declaration {
        identifier: String,
        data_type: String,
        value: String,
    },
    Operation {
        operator: String,
        operands: Vec<String>,
    },
}

// Parse a list of tokens into an expression
pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut tokens = tokens.into_iter();

/*     // Verify that the expression starts with a declaration
    if let Some(Token::Declare) = tokens.next() {

        let identifier = match tokens.next() {
            Some(Token::Identifier(id)) => id,
            _ => return Err("Syntax error: Expected an identifier after 'D'.".into()),
        };

        let data_type = match tokens.next() {
            Some(Token::Type(t)) if is_valid_note(&t.to_string()) => t.to_string(),
            Some(Token::Type(t)) => {
                return Err(format!("Semantic error: '{}' is not a valid data type.", t));
            }
            _ => return Err("Syntax error: Expected a data type after the identifier.".into()),
        };

        let value = match tokens.next() {
            Some(Value(note)) => note,
            _ => return Err("Syntax error: Expected a note as the value.".into()),
        };

        return Ok(Expr::Declaration { identifier, data_type, value });
    }  */

    // Verify that the expression starts with an operator
    let operator = match tokens.next() {
        Some(Operator(op)) if is_valid_operator(&op.to_string()) => op,
        Some(Operator(op)) => {
            return Err(format!("Semantic error: '{}' is not a valid operator.", op));
        }
        _ => {
            return Err("Syntax error: Expression must start with an operator.".into())
        },
    };

    // Parse the operands
    let mut operands = Vec::new();
    for token in tokens.by_ref() {
        match token {
            Value(note) => {
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