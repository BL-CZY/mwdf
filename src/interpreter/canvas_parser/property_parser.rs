use crate::{interpreter::structs::{InterpreterError, NumberParseState, Token}, view::{elements::Property, structs::NumberType}};
use super::canvas_tree::CanvasNode;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

fn parse_float(input: &str) -> Result<f32, InterpreterError> {
    let result: f32 = 0.00;
    Ok(result)
}

fn parse_single_number(token: Token) -> Result<NumberType, InterpreterError> {
    let digits: HashSet<char> = HashSet::from(
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
    );
    let mut result = NumberType::Pixel(0);

    if token.content.is_empty() {
        return Err(InterpreterError::Syntax(token.row, token.col, format!("empty number")));
    }

    //check the last digit
    match token.content.chars().last().unwrap() {
        '%' => {
            //it's a percent
        },
        'x' => {},
        'm' => {},
        _ => {},
    }
    
    Ok(result)
}

fn parse_number_list(tokens: &[Token]) -> Result<Vec<NumberType>, InterpreterError> {
    //initialize
    let result: Vec<NumberType> = vec![];
    let mut parse_state: NumberParseState = NumberParseState::None;

    //start the loop
    for token in tokens {
        match parse_state {
            NumberParseState::None => {
                match token.content.as_str() {
                    "(" => {
                        //marks the start of a number vector
                        parse_state = NumberParseState::FirstDigit;
                    },

                    _ => {
                        //nothing else is expected
                        return Err(InterpreterError::Syntax(token.row, token.col, format!("expect \"(\" here")));
                    },
                }
            },
            NumberParseState::FirstDigit => {},
            NumberParseState::Digit => {},
            NumberParseState::End => {},
        }
    }

    Ok(result)
}

//this function takes in the current serving node, the current serving property name, and the tokens representing the property
//this function sets the property
pub fn set_property_value(node: Rc<RefCell<CanvasNode>>, property_name: &String, tokens: &[Token]) -> Result<(), InterpreterError> {
    //check if the list is empty
    if tokens.is_empty() {
        return Err(InterpreterError::InternalError(0, 0, format!("empty tokens passed to the property parser")))
    }

    //check if the property name is presented again
    if let None = node.borrow_mut().value.properties.get_mut(property_name) {
        return Err(InterpreterError::InternalError(tokens[0].row, tokens[0].col, format!("this property name should have been checked for not existing in the property list")));
    }

    match node.borrow_mut().value.properties.get_mut(property_name).unwrap() {
        Property::Width(val) => {
            //expect a number type
            parse_number_list(tokens);
        },

        _ => {
            return Err(InterpreterError::InternalError(tokens[0].row, tokens[0].col, format!("property WIP or deprecated")));
        },
    }

    Ok(())
}