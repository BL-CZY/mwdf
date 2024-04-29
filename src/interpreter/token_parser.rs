use std::fs::File;
use std::io::Read;

use super::structs::{InterpreterError, TokenParseState, Token};

fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
   //read the file into a vector of ASCII characters
   let mut f: File = File::open(path)?;
   let mut data: Vec<u8> = vec![];
   f.read_to_end(&mut data)?;
   Ok(data)
}

pub fn to_token_list(path: &str) -> Result<Vec<Token>, InterpreterError> {
   //read the file
   let data: Vec<u8>;

   match read_file(path) {
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

                  '\"' | '`' => {
                        parse_state = TokenParseState::Str;
                        //abt strings
                        tokens.push(Token::new(String::from(*character), row, col));
                        tokens.push(Token::new(String::from(""), row, col));
                  },

                  '@' => {
                        parse_state = TokenParseState::Builtin;
                        tokens.push(Token::new(String::from("@"), row, col));
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
                        return Err(InterpreterError::Syntax(row, col, String::from("unknown token start")));
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
                        return Err(InterpreterError::Syntax(row, col, String::from("recursive tag declaration")));
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
                  '\"' | '`' => {
                        parse_state = TokenParseState::None;
                        tokens.push(Token::new(String::from(*character), row, col));
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

                  ' ' | '\n' => {},

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
                        return Err(InterpreterError::Syntax(row, col, String::from("unknown token end")));
                  },
               };
            },
            TokenParseState::Mark => {
               match character {
                  ':' | '|' => {
                        parse_state = TokenParseState::None;
                        tokens.push(Token::new(String::from(*character), row, col));
                  },
                  ' ' | '\n' => {},
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
                        tokens.push(Token::new(format!(""), row, col));
                  },
                  //another pair of )
                  ')' => {
                        parse_state = TokenParseState::None;
                        tokens.push(Token::new(String::from(")"), row, col));
                  },
                  
                  ' ' | '\n' => {},

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
      print!("{} ", token.content);
   };
   println!("");

   if tokens.len() == 0 {
      return Err(InterpreterError::EmptyFile);
   }

   Ok(tokens)
}