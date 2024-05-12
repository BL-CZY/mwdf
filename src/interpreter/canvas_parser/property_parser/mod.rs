pub mod enum_parser;
pub mod number_parser;
pub mod string_parser;

use super::canvas_tree::CanvasNode;
use crate::{
    interpreter::structs::{InterpreterError, Token},
    view::{
        elements::Property,
        structs::{ColorType, NumberType, PivotType, PointType, TextAlignType},
    },
};

use std::{cell::RefCell, rc::Rc};

fn set_number(target: &mut NumberType, tokens: &[Token]) -> Result<(), InterpreterError> {
    //expect a number type
    let mut temp_vec: Vec<NumberType>;
    match number_parser::parse_number_list(tokens) {
        Ok(result) => {
            temp_vec = result;
        }

        Err(e) => {
            return Err(e);
        }
    }

    //cannot be empty
    if temp_vec.is_empty() {
        return Err(InterpreterError::Property(
            tokens[0].row,
            tokens[0].col,
            format!("empty vector not expected"),
        ));
    }

    //dereference it to set the value
    *target = temp_vec.remove(0);
    Ok(())
}

fn set_point(target: &mut PointType, tokens: &[Token]) -> Result<(), InterpreterError> {
    //expect a number type
    let mut temp_vec: Vec<NumberType>;
    match number_parser::parse_number_list(tokens) {
        Ok(result) => {
            temp_vec = result;
        }

        Err(e) => {
            return Err(e);
        }
    }

    //cannot be less than 2
    if temp_vec.len() < 2 {
        return Err(InterpreterError::Property(
            tokens[0].row,
            tokens[0].col,
            format!("vector too short for this property"),
        ));
    }

    //dereference it to set the value
    *target = PointType::from(temp_vec.remove(0), temp_vec.remove(0));
    Ok(())
}

fn set_bool(target: &mut bool, tokens: &[Token]) -> Result<(), InterpreterError> {
    match string_parser::parse_bool(tokens) {
        Ok(result) => *target = result,

        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

fn set_string(target: &mut String, tokens: &[Token]) -> Result<(), InterpreterError> {
    match string_parser::parse_string(tokens) {
        Ok(result) => *target = result,

        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

fn set_pivot(target: &mut PivotType, tokens: &[Token]) -> Result<(), InterpreterError> {
    match enum_parser::parse_pivot(tokens) {
        Ok(result) => *target = result,

        Err(e) => return Err(e),
    }

    Ok(())
}

fn set_text_align(target: &mut TextAlignType, tokens: &[Token]) -> Result<(), InterpreterError> {
    match enum_parser::parse_text_align(tokens) {
        Ok(result) => *target = result,

        Err(e) => return Err(e),
    }

    Ok(())
}

fn set_color(target: &mut ColorType, tokens: &[Token]) -> Result<(), InterpreterError> {
    //expect a number type
    let mut temp_vec: Vec<NumberType>;
    match number_parser::parse_number_list(tokens) {
        Ok(result) => {
            temp_vec = result;
        }

        Err(e) => {
            return Err(e);
        }
    }

    //cannot be less than 4, it's guaranteed to not be empty
    if temp_vec.len() < 4 {
        return Err(InterpreterError::Property(
            tokens[0].row,
            tokens[0].col,
            format!("vector too short for this property"),
        ));
    }

    let mut result_vec: Vec<u8> = vec![];

    //check the first for elements to be either a percent or a number with value < 256
    for _ in 0..4 {
        //get the first element
        match temp_vec.remove(0) {
            //if it's a number, check if it's smaller than 256
            NumberType::Number(val) => {
                if val >= 256 {
                    return Err(InterpreterError::Property(
                        tokens[0].row,
                        tokens[1].col,
                        format!(
                            "number assigned to color is too large, should be smaller than 256"
                        ),
                    ));
                }

                //push this value in the result vector as u8
                result_vec.push(val as u8);
            }

            NumberType::Percent(val) => {
                if val >= 100.0 {
                    return Err(InterpreterError::Property(
                        tokens[0].row,
                        tokens[1].col,
                        format!(
                            "percent assigned to color is too large, should be smaller than or equal to 100%"
                        ),
                    ));
                }

                //push the percent * 255 to the result vector as u8
                result_vec.push((255.0 * val).floor() as u8);
            }

            //if it's not a number or percent, it's an error
            _ => {
                return Err(InterpreterError::Property(
                    tokens[0].row,
                    tokens[1].col,
                    format!(
                        "value assigned to color is not allowed, should be a percent or a number"
                    ),
                ));
            }
        }
    }

    //set the value
    *target = ColorType::from(
        result_vec.remove(0),
        result_vec.remove(0),
        result_vec.remove(0),
        result_vec.remove(0),
    );

    Ok(())
}

//this function takes in the current serving node, the current serving property name, and the tokens representing the property
//this function sets the property
pub fn set_property_value(
    node: Rc<RefCell<CanvasNode>>,
    property_name: &String,
    tokens: &[Token],
) -> Result<(), InterpreterError> {
    //check if the list is empty
    if tokens.is_empty() {
        return Err(InterpreterError::InternalError(
            0,
            0,
            format!("empty tokens passed to the property parser"),
        ));
    }

    //check if the property name is presented again
    if let None = node.borrow_mut().value.properties.get_mut(property_name) {
        return Err(InterpreterError::InternalError(
            tokens[0].row,
            tokens[0].col,
            format!(
                "this property name should have been checked for not existing in the property list"
            ),
        ));
    }

    match node
        .borrow_mut()
        .value
        .properties
        .get_mut(property_name)
        .unwrap()
    {
        //treat it as a reference in pattern matching
        Property::Width(ref mut val) => {
            //if the function returns an error, forward it
            if let Err(e) = set_number(val, tokens) {
                return Err(e);
            }
        }

        Property::Height(ref mut val) => {
            if let Err(e) = set_number(val, tokens) {
                return Err(e);
            }
        }

        Property::Position(ref mut val) => {
            if let Err(e) = set_point(val, tokens) {
                return Err(e);
            }
        }

        Property::PosRelToParent(ref mut val) => {
            if let Err(e) = set_bool(val, tokens) {
                return Err(e);
            }
        }

        Property::Pivot(ref mut val) => {
            if let Err(e) = set_pivot(val, tokens) {
                return Err(e);
            }
        }

        Property::BackgroundColor(ref mut val) => {
            if let Err(e) = set_color(val, tokens) {
                return Err(e);
            }
        }

        Property::TextColor(ref mut val) => {
            if let Err(e) = set_color(val, tokens) {
                return Err(e);
            }
        }

        Property::Text(ref mut val) => {
            if let Err(e) = set_string(val, tokens) {
                return Err(e);
            }
        }

        Property::Font(ref mut val) => {
            if let Err(e) = set_string(val, tokens) {
                return Err(e);
            }
        }

        Property::TextAlign(ref mut val) => {
            if let Err(e) = set_text_align(val, tokens) {
                return Err(e);
            }
        }
    }

    Ok(())
}
