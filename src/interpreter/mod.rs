pub mod structs;
pub mod token_parser;
pub mod var_paser;

use std::collections::HashMap;
use crate::view::elements::Element;
use self::structs::{ InterpreterError, Token, VarListElement };

pub fn interpret_file(path: &str) -> Result<Vec<Element>, InterpreterError> {
    //initialize token lists
    let tokens: Vec<Token>;

    match token_parser::to_token_list(path) {
        Ok(result) => tokens = result,
        Err(e) => {
            match e {
                InterpreterError::InvalidFile => {
                    println!("File Reader: no such file exists in the given path");
                }

                InterpreterError::DecodingError => {
                    println!("File Reader: unrecognized format");
                }

                InterpreterError::EmptyFile => {
                    println!("Token Parser: empty file or no valid tokens");
                },
                
                _ => {
                    println!("unhandled error");
                },
            };
            return Err(e);
        },
    };

    //initialize the result hashmap
    let vars: HashMap<String, Vec<VarListElement>>;
    //initialize the index
    let mut index: u32 = 0;

    match var_paser::parse_var(&tokens, &mut index) {
        Ok(result) => vars = result,
        Err(e) => {
            match &e {
                InterpreterError::Syntax(row, col, msg) => {
                    println!("Var Parser: syntax error at line {}, character {}, message: {}", row, col, msg);
                },

                InterpreterError::InternalError(msg) => {
                    println!("INTERNAL ERROR: {}", msg);
                },

                _ => {
                    println!("unhandled error");
                },
            };

            return Err(e);
        },
    };

    for (key, val) in vars.iter() {
        for i in val.iter() {
            match i {
                VarListElement::Token(t) => {
                    print!("{} ", t.content.as_str());
                },

                VarListElement::ArgDescriptor(a) => {
                    print!("{} ", a.arg_num);
                },
            };
        }
        println!("");
    }

    Ok(vec![])
}