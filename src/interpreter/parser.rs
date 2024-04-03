use std::fs::File;
use std::io::Read;
use crate::view::elements::Element;
enum TokenParseState {
    None,
    Tag,
}

//so that i can compare it using ==
#[derive(PartialEq)]
enum TokenConvertState {
    Var,
    Canvas,
    Property,
}

pub enum ParserError {
    Syntax(u32),
    NoClosingTag(u32),
    MissingEntryTag(u32),
    MultipleEntryTag(u32),
    BrokenSection,
    EmptyFile,
}

pub struct Parser {  }

impl Parser {
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
    fn split_file(&mut self, path: &str) -> Result<Vec<String>, ParserError> {

        //read the file
        let mut data: Vec<u8> = vec![];
        
        match self.read_file(path) {
            Ok(file_data) => data = file_data,

            Err(e) => println!("Error {}", e),
        }

        //initialize the state machine
        let mut tokens: Vec<String> = Vec::new();
        let mut parse_state: TokenParseState = TokenParseState::None;

        let mut index = 0;
        //start parsing
        for i in data.iter() {
            let temp: char = char::from(*i);
            match parse_state {
                //if none then proceed until see a "<"
                TokenParseState::None => {
                    match temp {
                        //if < then it's a tag
                        '<' => {
                            parse_state = TokenParseState::Tag;
                            tokens.push(String::from("<"));
                        },
                        
                        //if space or next line ignore it
                        ' ' | '\n' => {},
                        
                        //if otherwise it's an error
                        _ => {
                            return Err(ParserError::Syntax(index));
                        },
                    }
                },
                //just getting tag tokens
                TokenParseState::Tag => {
                    match temp {
                        //exit tag mode
                        '>' => {
                            tokens.last_mut().unwrap().push_str(&format!("{}", temp));
                            parse_state = TokenParseState::None;
                        }

                        //you can't have a tag in a tag
                        '<' => {
                            return Err(ParserError::Syntax(index));
                        }

                        //or whaever just append the tag
                        _ => {
                            tokens.last_mut().unwrap().push_str(&format!("{}", temp));
                        }
                    }
                },
            }
            index += 1;
        }

        for str in tokens.iter() {
            print!("{}", str);
        }

        Ok(tokens)
    }

    pub fn parse_file(&mut self, path: &str) -> Result<Vec<Element>, ParserError>{

        //init the result vector
        let mut parsed_vector: Vec<Element> = vec![];

        //get the split tokens vector
        let mut split_list: Vec<String> = vec![];
        match self.split_file(path) {
            Ok(result) => split_list = result,
            Err(e) => return Err(e),
        };

        //initialize the stack
        let mut stack: Vec<String> = vec![];
        
        let mut parse_state: TokenConvertState;

        //check for empty and incomplete stuffs
        match split_list.first() {
            Some(str) => {
                //convert the string to a string slice
                match str.as_str() {
                    "<var>" => parse_state = TokenConvertState::Var,
                    "<canvas>" => parse_state = TokenConvertState::Canvas,

                    _ => {
                        return  Err(ParserError::MissingEntryTag(0));
                    },
                }
            },
            None => {
                return Err(ParserError::EmptyFile)
            },
        }
        
        let mut index: u32 = 0;
        if parse_state == TokenConvertState::Var {
            //push to the stack
            //TODO incomplete
            for i in split_list.iter() {
                match i.as_str() {
                    //if it's the <var>, push to the stack
                    "<var>" => {
                        //if the stack is not empty, it's appeared for more than 1 times, error
                        if stack.len() != 0 {
                            return Err(ParserError::MultipleEntryTag(index));
                        }
                        stack.push(String::from("<var>"));
                    },
                    "</var>" => {
                        stack.pop();
                        break;
                    },
                    _ => { }
                }
                index += 1;
            }
            //if the stack is not cleared, there is something wrong
            if stack.len() != 0 {
                return Err(ParserError::BrokenSection);
            }
            
            //if the current index in the vector is not <canvas>, there is comething wrong
            if stack[index as usize].as_str() != "<canvas>" {
                return Err(ParserError::BrokenSection);
            }
        }

        //deal with the canvas
        parse_state = TokenConvertState::Canvas;
        let slice: &[String] = &split_list[index as usize..split_list.len()];

        for i in slice.iter() {
            match parse_state {
                //at this state it's processing tags
                TokenConvertState::Canvas => {
                    
                },

                TokenConvertState::Property => {

                },

                TokenConvertState::Var => {

                },
            }
        }

        Ok(parsed_vector)
    }

}