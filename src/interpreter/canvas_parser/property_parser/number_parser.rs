use crate::{
    interpreter::structs::{InterpreterError, NumberParseState, Token},
    view::structs::NumberType,
};

fn parse_single_number(token: &Token) -> Result<NumberType, InterpreterError> {
    let mut result = NumberType::Pixel(0);

    if token.content.is_empty() {
        return Err(InterpreterError::Syntax(
            token.row,
            token.col,
            format!("empty number"),
        ));
    }

    //check the last digit
    match token.content.chars().last().unwrap() {
        '%' => {
            //it's a percent
            //get rid of the last char
            let temp_slice = &token.content[..token.content.len() - 1];
            match temp_slice.parse::<f32>() {
                Ok(val) => {
                    result = NumberType::Percent(val / 100.0);
                }
                _ => {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        format!("failed to parse the value {} into a percent", token.content),
                    ));
                }
            }
        }
        'x' => {
            //if the length is too short or the second last is not p
            if token.content.len() < 3
                || token.content.chars().nth(token.content.len() - 2) != Some('p')
            {
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    format!("number not recognizable"),
                ));
            }

            let temp_slice = &token.content[..token.content.len() - 1];
            match temp_slice.parse::<u32>() {
                Ok(val) => {
                    result = NumberType::Pixel(val);
                }
                _ => {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        format!(
                            "failed to parse the value {} into a pixel value",
                            token.content
                        ),
                    ));
                }
            }
        }
        'm' => {
            //if the length is too short or the second last is not e
            if token.content.len() < 3
                || token.content.chars().nth(token.content.len() - 2) != Some('e')
            {
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    format!("number not recognizable"),
                ));
            }

            let temp_slice = &token.content[..token.content.len() - 1];
            match temp_slice.parse::<f32>() {
                Ok(val) => {
                    result = NumberType::Percent(val);
                }
                _ => {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        format!("failed to parse the value {} into an em", token.content),
                    ));
                }
            }
        }
        _ => {}
    }

    Ok(result)
}

pub fn parse_number_list(tokens: &[Token]) -> Result<Vec<NumberType>, InterpreterError> {
    //initialize
    let mut result: Vec<NumberType> = vec![];
    let mut parse_state: NumberParseState = NumberParseState::None;

    //start the loop
    for token in tokens {
        match parse_state {
            NumberParseState::None => {
                match token.content.as_str() {
                    "(" => {
                        //marks the start of a number vector
                        parse_state = NumberParseState::Number;
                    }

                    _ => {
                        //nothing else is expected
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            format!("expect \"(\" here"),
                        ));
                    }
                }
            }

            NumberParseState::Number => {
                match parse_single_number(token) {
                    Ok(val) => {
                        //if succeed, push it to result
                        result.push(val);
                        parse_state = NumberParseState::Next;
                    }

                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            NumberParseState::Next => {
                match token.content.as_str() {
                    "," => {
                        parse_state = NumberParseState::Number;
                    }

                    ")" => {
                        //to finish state
                        parse_state = NumberParseState::Finish;
                    }

                    _ => {
                        //error
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            format!("unexpect \",\" or \")\" here"),
                        ));
                    }
                }
            }

            NumberParseState::Finish => {
                //if it runs here, everything is an error
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    format!("more tokens than expected"),
                ));
            }
        }
    }

    Ok(result)
}
