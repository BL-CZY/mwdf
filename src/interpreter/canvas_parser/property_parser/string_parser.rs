use crate::interpreter::structs::{InterpreterError, Token};

pub fn parse_string(tokens: &[Token]) -> Result<String, InterpreterError> {
    let result: String;

    if tokens.is_empty() {
        return Err(InterpreterError::InternalError(
            0,
            0,
            format!("empty token list passed to string parser"),
        ));
    }

    //a string's structure must follow: " + content + "
    if tokens.len() != 3 {
        return Err(InterpreterError::Syntax(
            tokens[0].row,
            tokens[1].col,
            format!("bad string declaration"),
        ));
    }

    if tokens[0].content.as_str() != "\"" || tokens[2].content.as_str() != "\"" {
        return Err(InterpreterError::Syntax(
            tokens[0].row,
            tokens[0].col,
            format!("bad string declaration"),
        ));
    }

    //just get the second token's content out
    result = String::from(tokens[1].content.as_str());

    Ok(result)
}

pub fn parse_bool(tokens: &[Token]) -> Result<bool, InterpreterError> {
    match parse_string(tokens) {
        Ok(val) => match val.as_str() {
            "true" => return Ok(true),
            "false" => return Ok(false),
            _ => {
                return Err(InterpreterError::Property(
                    tokens[0].row,
                    tokens[1].col,
                    format!("content is not a bool value"),
                ))
            }
        },

        Err(e) => {
            return Err(e);
        }
    }
}
