use std::collections::HashMap;

use super::structs::{Token, InterpreterError, VarHashState, VarListElement};

pub fn parse_var(tokens: &Vec<Token>, index: &mut u32) -> Result<HashMap<String, Vec<VarListElement>>, InterpreterError> {
    //initialize the stack
    let mut stack: Vec<&Token> = vec![];
    //initialize the hashmap
    let mut result: HashMap<String, Vec<VarListElement>> = HashMap::new();
    //initialize the has state
    let mut hash_state: VarHashState = VarHashState::None;

    let mut current_var: String = String::from("");
    'hashing: loop {
        let token: &Token = &tokens[*index as usize];
        if (token.content.as_str() == "</var>") || ((*index as usize) >= tokens.len() || token.content.as_str() == "<canvas>") {
            break 'hashing;
        }
        match hash_state {
            VarHashState::None => {
                match token.content.as_str() {
                    //var section starter
                    "<var>" => {},

                    //at this point only accepts $
                    "$" => {
                        //take the name
                        hash_state = VarHashState::VarName;
                    },
                    _ => {
                        return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"$\" here")));
                    },
                }
            },
            VarHashState::VarName => {
                match token.content.as_str() {
                    //type
                    ":" => {
                        hash_state = VarHashState::VarType;
                    },

                    //hash the name
                    _ => {
                        current_var = String::from(&token.content);
                        result.insert(String::from(&token.content), vec![]);
                    },
                };
            },
            VarHashState::VarType => {
                match token.content.as_str() {
                    "@str" => {
                        result.get_mut(&current_var).unwrap().push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefStrEqual;
                    },
                    "@ft" => {
                        result.get_mut(&current_var).unwrap().push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefFontEqual;
                    },
                    "@vec" => {
                        result.get_mut(&current_var).unwrap().push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefVecEqual;
                    },
                    "@exp" => {
                        result.get_mut(&current_var).unwrap().push(VarListElement::Token(Token::from(token)));
                        hash_state = VarHashState::VarDefExpEqual;
                    },
                    
                    _ => {
                        return Err(InterpreterError::Syntax(token.row, token.col, format!("{} is an undefined type", token.content)));
                    },
                };
            },
            VarHashState::VarDefStrEqual => {
                if token.content.as_str() == "=" {
                    hash_state = VarHashState::VarDefStrQuota;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"=\" here")));
                }
            },
            VarHashState::VarDefStrQuota => {
                if token.content.as_str() == "\"" {
                    hash_state = VarHashState::VarDefStrContent;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"\"\" here")));
                }
            },
            VarHashState::VarDefStrContent => {
                match token.content.as_str() {
                    "\"" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    },

                    _ => {
                        result.get_mut(&current_var).unwrap().push(Token::new(String::from(token.content.as_str()), token.row, token.col));
                    },
                };
            },
            VarHashState::VarDefFontEqual => {
                if token.content.as_str() == "=" {
                    hash_state = VarHashState::VarDefFontQuota;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"=\" here")));
                }
            },
            VarHashState::VarDefFontQuota => {
                if token.content.as_str() == "`" {
                    hash_state = VarHashState::VarDefFontContent;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"`\" here")));
                }
            },
            VarHashState::VarDefFontContent => {
                match token.content.as_str() {
                    "`" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    },

                    _ => {
                        result.get_mut(&current_var).unwrap().push(Token::new(String::from(token.content.as_str()), token.row, token.col));
                    },
                };
            },
            VarHashState::VarDefVecEqual => {
                if token.content.as_str() == "=" {
                    hash_state = VarHashState::VarDefVecParenth;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"=\" here")));
                }
            },
            VarHashState::VarDefVecParenth => {
                if token.content.as_str() == "(" {
                    hash_state = VarHashState::VarDefVecContent;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"(\" here")));
                }
            },
            VarHashState::VarDefVecContent => {
                match token.content.as_str() {
                    ")" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    },

                    "," => {},

                    _ => {
                        result.get_mut(&current_var).unwrap().push(Token::new(String::from(token.content.as_str()), token.row, token.col));
                    },
                };
            },
            VarHashState::VarDefExpEqual => {
                if token.content.as_str() == "=" {
                    hash_state = VarHashState::VarDefExpArgsBracket;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"=\" here")));
                }
            },
            VarHashState::VarDefExpArgsBracket => {
                if token.content.as_str() == "{" {
                    hash_state = VarHashState::VarDefExpArgsDollar;
                } else {
                    return Err(InterpreterError::Syntax(token.row, token.col, String::from("expect \"{\" here")));
                }
            },
            VarHashState::VarDefExpContent => {
                match token.content.as_str() {
                    "}" => {
                        //end of string section
                        hash_state = VarHashState::VarSemiColon;
                    },

                    _ => {
                        result.get_mut(&current_var).unwrap().push(Token::new(String::from(token.content.as_str()), token.row, token.col));
                    },
                };
            },
            VarHashState::VarSemiColon => {
                //it's not mandatory to have the semicolon
                //*(yet?
                hash_state = VarHashState::None;
            },
        };
        *index += 1;
    }

    //proceed
    *index += 1;

    Ok(result)
}