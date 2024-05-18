use std::collections::HashMap;

use super::structs::{InterpreterError, MacroHashState, MacroListElement, Token};

#[macro_export]
macro_rules! check_single_token {
    ($token:expr, $hash_state:expr, $expected:expr, $state:expr) => {
        if $token.content.as_str() == $expected {
            $hash_state = $state;
        } else {
            return Err(InterpreterError::Syntax(
                $token.row,
                $token.col,
                format!(
                    "expect \"{}\" here, found \"{}\"",
                    $expected,
                    $token.content.as_str()
                ),
            ));
        }
    };
}

pub fn parse_macro<'a>(
    tokens: &'a Vec<Token>,
    index: &mut u32,
) -> Result<HashMap<&'a str, Vec<MacroListElement<'a>>>, InterpreterError> {
    //initialize the result hashmap
    let mut result: HashMap<&str, Vec<MacroListElement>> = HashMap::new();
    let mut hash_state: MacroHashState = MacroHashState::None;
    let mut current_var: &str = "";

    for token in tokens[1..].iter() {
        *index += 1;

        if token.content.as_str() == "</macro>" {
            *index += 1;
            break;
        }

        match hash_state {
            MacroHashState::None => {
                check_single_token!(token, hash_state, "$", MacroHashState::VarName);
            }

            MacroHashState::VarName => {
                result.insert(token.content.as_str(), vec![]);
                current_var = token.content.as_str();
                hash_state = MacroHashState::VarEqual;
            }

            MacroHashState::VarEqual => {
                check_single_token!(token, hash_state, "=", MacroHashState::VarArgsStart);
            }

            MacroHashState::VarArgsStart => {
                check_single_token!(token, hash_state, "{", MacroHashState::VarArgsDollar);
            }

            MacroHashState::VarArgsDollar => {
                check_single_token!(token, hash_state, "$", MacroHashState::VarArgsName);
            }

            MacroHashState::VarArgsName => {
                if let None = result.get_mut(&current_var) {
                    return Err(InterpreterError::InternalError(
                        token.row,
                        token.col,
                        format!("unknown var \"{}\" identified", token.content.as_str()),
                    ));
                }

                //push the arg name to the stack
                result
                    .get_mut(&current_var)
                    .unwrap()
                    .push(MacroListElement::Arg(token.content.as_str()));

                hash_state = MacroHashState::VarArgsNext;
            }

            MacroHashState::VarArgsNext => match token.content.as_str() {
                "," => hash_state = MacroHashState::VarArgsDollar,
                "}" => hash_state = MacroHashState::VarArgsEnd,
                _ => {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        format!(
                            "expect \",\" or \"}}\" here, found {}",
                            token.content.as_str()
                        ),
                    ))
                }
            },

            MacroHashState::VarArgsEnd => {
                check_single_token!(token, hash_state, "->", MacroHashState::VarContentStart);
            }

            MacroHashState::VarContentStart => {
                check_single_token!(token, hash_state, "{", MacroHashState::VarContent);
            }

            MacroHashState::VarContent => {
                if let None = result.get_mut(&current_var) {
                    return Err(InterpreterError::InternalError(
                        token.row,
                        token.col,
                        format!("unknown var \"{}\" identified", token.content.as_str()),
                    ));
                }

                if token.content.as_str() == "}" {
                    hash_state = MacroHashState::VarContentEnd;
                } else {
                    result
                        .get_mut(&current_var)
                        .unwrap()
                        .push(MacroListElement::Token(&token))
                }
            }

            MacroHashState::VarContentEnd => {
                check_single_token!(token, hash_state, ";", MacroHashState::None);
            }
        }
    }

    Ok(result)
}
