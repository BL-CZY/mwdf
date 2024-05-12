use std::collections::HashMap;

use crate::interpreter::structs::ArgDescriptor;

use super::structs::{InterpreterError, Token, VarHashState, VarListElement};

#[macro_export]
macro_rules! check_single_token {
    ($token:expr, $hash_state:expr, $expected:expr, $state:expr) => {
        if $token.content.as_str() == $expected {
            $hash_state = $state;
        } else {
            return Err(InterpreterError::Syntax(
                $token.row,
                $token.col,
                format!("expect \"{}\" here", $expected),
            ));
        }
    };
}

pub fn parse_var(
    tokens: &Vec<Token>,
    index: &mut u32,
) -> Result<HashMap<String, Vec<VarListElement>>, InterpreterError> {
    //initialize the hashmap
    let mut result: HashMap<String, Vec<VarListElement>> = HashMap::new();
    //initialize the has state
    let mut hash_state: VarHashState = VarHashState::None;

    let mut current_var: String = String::from("");
    'hashing: loop {
        let token: &Token = &tokens[*index as usize];
        if (token.content.as_str() == "</var>")
            || ((*index as usize) >= tokens.len() || token.content.as_str() == "<canvas>")
        {
            //check if it's the start of the var declaration cycle
            if hash_state != VarHashState::None {
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    format!("incomplete variable declaration"),
                ));
            }
            break 'hashing;
        }
        match hash_state {
            VarHashState::None => {
                match token.content.as_str() {
                    //var section starter
                    "<var>" => {}

                    //at this point only accepts $
                    "$" => {
                        //take the name
                        hash_state = VarHashState::VarName;
                    }
                    _ => {
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            String::from("expect \"$\" here"),
                        ));
                    }
                }
            }
            VarHashState::VarName => {
                current_var = String::from(&token.content);
                result.insert(String::from(&token.content), vec![]);
                hash_state = VarHashState::VarTypeColon;
            }
            VarHashState::VarTypeColon => {
                check_single_token!(token, hash_state, ":", VarHashState::VarType);
            }
            VarHashState::VarType => {
                match token.content.as_str() {
                    "@str" => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefStrEqual;
                    }
                    "@ft" => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefFontEqual;
                    }
                    "@vec" => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefVecEqual;
                    }
                    "@exp" => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefExpEqual;
                    }

                    _ => {
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            format!("{} is an undefined type", token.content),
                        ));
                    }
                };
            }
            VarHashState::VarDefStrEqual => {
                check_single_token!(token, hash_state, "=", VarHashState::VarDefStrQuota);
            }
            VarHashState::VarDefStrQuota => {
                check_single_token!(token, hash_state, "\"", VarHashState::VarDefStrContent);
            }
            VarHashState::VarDefStrContent => {
                match token.content.as_str() {
                    "\"" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    }

                    _ => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                    }
                };
            }
            VarHashState::VarDefFontEqual => {
                check_single_token!(token, hash_state, "=", VarHashState::VarDefFontQuota);
            }
            VarHashState::VarDefFontQuota => {
                check_single_token!(token, hash_state, "`", VarHashState::VarDefFontContent);
            }
            VarHashState::VarDefFontContent => {
                match token.content.as_str() {
                    "`" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    }

                    _ => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                    }
                };
            }
            VarHashState::VarDefVecEqual => {
                check_single_token!(token, hash_state, "=", VarHashState::VarDefVecParenth);
            }
            VarHashState::VarDefVecParenth => {
                if token.content.as_str() == "(" {
                    hash_state = VarHashState::VarDefVecContent;
                } else {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        String::from("expect \"(\" here"),
                    ));
                }
            }
            VarHashState::VarDefVecContent => {
                match token.content.as_str() {
                    ")" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    }

                    "," => {}

                    _ => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                    }
                };
            }
            VarHashState::VarDefExpEqual => {
                check_single_token!(token, hash_state, "=", VarHashState::VarDefExpArgsBracket);
            }
            VarHashState::VarDefExpArgsBracket => {
                //initialize the arg number counter
                check_single_token!(token, hash_state, "{", VarHashState::VarDefExpArgsDollar);
                result
                    .get_mut(&current_var)
                    .unwrap()
                    .push(VarListElement::ArgDescriptor(ArgDescriptor::new()));
            }
            VarHashState::VarDefExpArgsDollar => {
                check_single_token!(token, hash_state, "$", VarHashState::VarDefExpArgsName);

                //when there is a new variable, increment the arg number counter by one
                //it should be the 3rd element, as the first is the name, and the second is the type
                match result
                    .get_mut(&current_var)
                    .unwrap()
                    .iter_mut()
                    .nth(1)
                    .unwrap()
                {
                    VarListElement::ArgDescriptor(val) => {
                        val.arg_num += 1;
                    }
                    _ => {
                        return Err(InterpreterError::InternalError(
                            token.row,
                            token.col,
                            format!("bad var hashtable structure"),
                        ));
                    }
                };
            }
            VarHashState::VarDefExpArgsName => {
                result
                    .get_mut(&current_var)
                    .unwrap()
                    .push(VarListElement::Token(Token::from(token)));
                hash_state = VarHashState::VarDefExpArgsColon;
            }
            VarHashState::VarDefExpArgsColon => {
                check_single_token!(token, hash_state, ":", VarHashState::VarDefExpArgsType);
            }
            VarHashState::VarDefExpArgsType => {
                //push the type
                result
                    .get_mut(&current_var)
                    .unwrap()
                    .push(VarListElement::Token(Token::from(token)));
                hash_state = VarHashState::VarDefExpArgsNext;
            }
            VarHashState::VarDefExpArgsNext => {
                match token.content.as_str() {
                    "," => {
                        //expecting the next to be the dollar sign
                        hash_state = VarHashState::VarDefExpArgsDollar;
                    }
                    "}" => {
                        //expecting the next to be the -> token
                        hash_state = VarHashState::VarDefExpArrow;
                    }
                    _ => {
                        //there is a syntax error
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            String::from("expect \",\" or \"}\" here"),
                        ));
                    }
                };
            }
            VarHashState::VarDefExpArrow => {
                check_single_token!(
                    token,
                    hash_state,
                    "->",
                    VarHashState::VarDefExpContentBracket
                );
            }
            VarHashState::VarDefExpContentBracket => {
                check_single_token!(token, hash_state, "{", VarHashState::VarDefExpContent);
            }
            VarHashState::VarDefExpContent => {
                match token.content.as_str() {
                    "}" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    }

                    _ => {
                        result
                            .get_mut(&current_var)
                            .unwrap()
                            .push(VarListElement::Token(Token::from(token)));
                    }
                };
            }
            VarHashState::VarSemiColon => {
                //it's not mandatory to have the semicolon
                //*(yet?
                hash_state = VarHashState::None;
            }
        };
        *index += 1;
    }

    //proceed
    *index += 1;

    Ok(result)
}
