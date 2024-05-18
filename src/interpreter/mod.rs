pub mod canvas_parser;
pub mod macro_paser;
pub mod structs;
pub mod token_parser;

use self::structs::{InterpreterError, MacroListElement, Token};
use crate::utils;
use crate::view::elements::Element;

use std::collections::HashMap;
use std::rc::Rc;

pub fn interpret_file(path: &str) -> Result<Vec<Element>, InterpreterError> {
    //initialize token lists
    let tokens: Vec<Token>;

    match token_parser::to_token_list(path) {
        Ok(result) => tokens = result,
        Err(e) => {
            match &e {
                InterpreterError::InvalidFile => {
                    println!("File Reader: no such file exists in the given path");
                }

                InterpreterError::DecodingError => {
                    println!("File Reader: unrecognized format");
                }

                InterpreterError::EmptyFile => {
                    println!("Token Parser: empty file or no valid tokens");
                }

                InterpreterError::Syntax(row, col, msg) => {
                    println!(
                        "Token Parser: syntax error at row {} col {}, message: {}",
                        row, col, msg
                    );
                }

                _ => {
                    println!("unhandled error");
                }
            };
            return Err(e);
        }
    };

    //initialize the result hashmap
    let vars: HashMap<&str, Vec<MacroListElement>>;
    //initialize the index
    let mut index: u32 = 0;

    match macro_paser::parse_macro(&tokens, &mut index) {
        Ok(result) => vars = result,
        Err(e) => {
            match &e {
                InterpreterError::Syntax(row, col, msg) => {
                    println!(
                        "Var Parser: syntax error at line {}, character {}, message: {}",
                        row, col, msg
                    );
                }

                InterpreterError::InternalError(row, col, msg) => {
                    println!("INTERNAL ERROR at {}, {}: {}", row, col, msg);
                }

                _ => {
                    println!("unhandled error");
                }
            };

            return Err(e);
        }
    };

    for (.., val) in vars.iter() {
        for i in val.iter() {
            match i {
                MacroListElement::Token(t) => {
                    print!("{} ", t.content.as_str());
                }

                MacroListElement::Arg(a) => {
                    print!("{} ", a);
                }
            };
        }
        println!("");
    }

    match canvas_parser::parse_canvas(&tokens[index as usize..], &mut index) {
        Ok(result) => {
            utils::print_canvas_tree(Rc::clone(&result), 0);
        }
        Err(e) => match e {
            InterpreterError::Syntax(r, c, msg) => {
                println!(
                    "canvas parser: syntax error at line {} character {}, message: {}",
                    r, c, msg
                );
            }

            InterpreterError::Property(r, c, msg) => {
                println!(
                    "canvas parser: syntax error at line {} character {}, message: {}",
                    r, c, msg
                );
            }

            InterpreterError::InternalError(row, col, msg) => {
                println!("INTERNAL ERROR at {}, {}: {}", row, col, msg);
            }

            _ => {
                println!("unhandled error");
            }
        },
    };

    Ok(vec![])
}
