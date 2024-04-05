pub mod structs;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::view::elements::Element;
use crate::interpreter::structs::{ TokenParseState, TokenConvertState, InterpreterError, CanvasInterpretState, Token };

pub struct Interpreter {  }

impl Interpreter {
    pub fn new() -> Self {
        Self {  }
    }

    fn read_file(&mut self, path: &str) -> std::io::Result<Vec<u8>> {
        //read the file into a vector of ASCII characters
        let mut f: File = File::open(path)?;
        let mut data: Vec<u8> = vec![];
        f.read_to_end(&mut data)?;

        Ok(data)
    }

    //this generates a vector of tokens
    pub fn to_token_list(&mut self, path: &str) -> Result<Vec<Token>, InterpreterError> {

        //read the file
        let mut data: Vec<u8> = vec![];
        
        match self.read_file(path) {
            Ok(file_data) => data = file_data,

            Err( .. ) => {
                return Err(InterpreterError::InvalidFile);
            }
        }

        //initialize the state machine
        let mut tokens: Vec<Token> = Vec::new();
        let mut parse_state: TokenParseState = TokenParseState::None;

        //initialize
        let mut row: u32 = 1;
        let mut col: u32 = 1;

        //convert the read data to chars
        let char_data: Vec<char>;
        match String::from_utf8(data) {
            Ok(result) => char_data = result.chars().collect(),
            Err(..) => return Err(InterpreterError::DecodingError),
        }

        //start parsing
        for character in char_data.iter() {
            match parse_state {
                //if none then proceed until see a "<"
                TokenParseState::None => {
                    match character {
                        //if < then it's a tag
                        '<' => {
                            parse_state = TokenParseState::Tag;
                            tokens.push(Token::new(String::from("<"), row, col));
                        },

                        '$' => {
                            parse_state = TokenParseState::Var;
                            //it's going to be abt variables
                            tokens.push(Token::new(String::from("$"), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        '\"' => {
                            parse_state = TokenParseState::Str;
                            //abt strings
                            tokens.push(Token::new(String::from("\""), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        '@' => {
                            parse_state = TokenParseState::Builtin;
                            tokens.push(Token::new(String::from("@"), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        //single char tokens
                        ';' | ':' | '+' | ',' | '|' | '{' | '}' => {
                            tokens.push(Token::new(String::from(*character), row, col));
                        },

                        //double char tokens
                        '-' => {
                            parse_state = TokenParseState::TwoCharToken;
                            tokens.push(Token::new(String::from(*character), row, col));
                        },

                        //marks
                        '*' => {
                            parse_state = TokenParseState::Mark;
                            tokens.push(Token::new(String::from("*"), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        //property
                        '^' => {
                            parse_state = TokenParseState::Property;
                            tokens.push(Token::new(String::from("^"), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        //vector
                        '(' => {
                            parse_state = TokenParseState::Vector;
                            tokens.push(Token::new(String::from(*character), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        //comments
                        '#' => {
                            parse_state = TokenParseState::Comment;
                        },
                        
                        //if space or next line ignore it
                        ' ' | '\n' => {  },
                        
                        //if otherwise it's an error
                        _ => {
                            return Err(InterpreterError::Syntax(row, col));
                        },
                    };
                },
                //just getting tag tokens
                TokenParseState::Tag => {
                    match character {
                        //exit tag mode
                        '>' => {
                            tokens.last_mut().unwrap().content.push(*character);
                            parse_state = TokenParseState::None;
                        }

                        //you can't have a tag in a tag
                        '<' => {
                            return Err(InterpreterError::Syntax(row, col));
                        }

                        //or whaever just append the tag
                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        }
                    };
                },
                TokenParseState::Var => {
                    match character {
                        //these would mark the end of a variable name
                        ':' | '=' | '+' => {
                            tokens.push(Token::new(String::from(*character), row, col));
                            parse_state = TokenParseState::None;
                        },

                        //ignore these
                        ' ' | '\n' => {  },

                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::Str => {
                    match character {
                        '\"' => {
                            parse_state = TokenParseState::None;
                            tokens.push(Token::new(String::from("\""), row, col));
                        },

                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::Builtin => {
                    match character {
                        ':' | ',' | ';' | '}' | '=' => {
                            tokens.push(Token::new(String::from(*character), row, col));
                            parse_state = TokenParseState::None;
                        },

                        '(' => {
                            parse_state = TokenParseState::Vector;
                            tokens.push(Token::new(String::from("("), row, col));
                            tokens.push(Token::new(String::from(""), row, col));
                        },

                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::TwoCharToken => {
                    match character {
                        '>' => {
                            parse_state = TokenParseState::None;
                            tokens.last_mut().unwrap().content.push(*character);
                        },

                        _ => {
                            return Err(InterpreterError::Syntax(row, col));
                        },
                    };
                },
                TokenParseState::Mark => {
                    match character {
                        ':' | '|' => {
                            parse_state = TokenParseState::None;
                            tokens.push(Token::new(String::from(*character), row, col));
                        },
                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::Property => {
                    match character {
                        '|' | ':' => {
                            parse_state = TokenParseState::None;
                            tokens.push(Token::new(String::from(*character), row, col));
                        },
                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::Vector => {
                    match character {
                        //new element in a vector
                        ',' => {
                            tokens.push(Token::new(String::from(*character), row, col));
                        },
                        //another pair of )
                        ')' => {
                            parse_state = TokenParseState::None;
                            tokens.push(Token::new(String::from(")"), row, col));
                        },
                        _ => {
                            tokens.last_mut().unwrap().content.push(*character);
                        },
                    };
                },
                TokenParseState::Comment => {
                    match character {
                        '#' => {
                            parse_state = TokenParseState::None;
                        },

                        _ => {},
                    };
                },
            };
            
            //keep track of rows and cols
            match character {
                '\n' => {
                    col = 0;
                    row += 1;
                },
                _ => {
                    col += 1;
                },
            };
        };

        for token in tokens.iter() {
            print!("{}, ", token.content);
        };

        Ok(tokens)
    }

    fn parse_var(&mut self, tokens: &Vec<Token>, index: &mut u32) -> Result<HashMap<&str, Vec<Token>>, InterpreterError> {
        let mut stack: Vec<&Token> = vec![];
        let result: HashMap<&str, Vec<Token>> = HashMap::new();
        //push to the stack
        //TODO incomplete
        for i in tokens.iter() {
            match i.content.as_str() {
                //if it's the <var>, push to the stack
                "<var>" => {
                    //if the stack is not empty, it's appeared for more than 1 times, error
                    if stack.len() != 0 {
                        return Err(InterpreterError::MultipleEntryTag(i.row, i.col));
                    }
                    stack.push(i);
                },
                "</var>" => {
                    if stack.len() != 1 {
                        return Err(InterpreterError::NoClosingTag(i.row, i.col));
                    }
                    stack.pop();
                    break;
                },
                _ => { }
            }
            *index += 1;
        }

        //if there is nothing else, it's an error
        if *index as usize == tokens.len() - 1 {
            return Err(InterpreterError::MissingEntryTag(tokens[*index as usize].row, tokens[*index as usize].col));
        }

        //proceed
        *index += 1;

        Ok(result)
    }

    //at this point i just pass the ownership, as they are no longer needed
    fn parse_canvas(&mut self, tokens: &Vec<Token>, vars: &HashMap<&str, Vec<Token>>, index: &mut u32) -> Result<Vec<Element>, InterpreterError> {
        //initialize the result vector
        let mut result: Vec<Element> = vec![];
        //initialize the state machine
        let mut state: CanvasInterpretState = CanvasInterpretState::None;
        //initialize the stack
        let mut stack: Vec<&Token> = vec![];

        while (*index as usize) < tokens.len() {
            let token: &Token = &tokens[*index as usize];
            match state {
                CanvasInterpretState::None => {
                    //if not starting with <canvas>
                    if token.content.as_str() != "<canvas>" {
                        return Err(InterpreterError::MissingEntryTag(token.row, token.col));
                    }

                    //it it, so push it to the stack and start interpreting this canvas
                    state = CanvasInterpretState::Tag;
                    stack.push(token);
                    *index += 1;
                },
                CanvasInterpretState::Tag => {
                    if token.content.as_str().chars().next().unwrap() == '$' {
                        //TODO variables
                        continue;
                    }
                },
                CanvasInterpretState::Property => {

                },
            }
        }
        Ok(result)
    }

    pub fn parse_file(&mut self, path: &str) -> Result<Vec<Element>, InterpreterError>{

        //init the result vector
        let mut parsed_vector: Vec<Element> = vec![];

        //get the split tokens vector
        let mut split_list: Vec<Token> = vec![];
        match self.to_token_list(path) {
            Ok(result) => split_list = result,
            Err(e) => return Err(e),
        };
        
        //initialize the variable hash table
        let mut vars: HashMap<&str, Vec<Token>> = HashMap::new();
        //initialize the parse state
        let mut parse_state: TokenConvertState;

        //check for empty and incomplete stuffs
        match split_list.first() {
            Some(token) => {
                //convert the string to a string slice
                match token.content.as_str() {
                    "<var>" => parse_state = TokenConvertState::Var,
                    "<canvas>" => parse_state = TokenConvertState::Canvas,

                    _ => {
                        return Err(InterpreterError::MissingEntryTag(token.row, token.col));
                    },
                }
            },
            None => {
                return Err(InterpreterError::EmptyFile)
            },
        }
        
        let mut index: u32 = 0;
        if parse_state == TokenConvertState::Var {
            match self.parse_var(&split_list, &mut index) {
                Ok(result) => vars = result,
                Err(e) => return Err(e),
            }
            parse_state = TokenConvertState::Canvas;
        }

        Ok(parsed_vector)
    }

}
