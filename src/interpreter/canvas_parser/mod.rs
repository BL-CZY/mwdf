pub mod canvas_tree;
pub mod property_parser;

use self::canvas_tree::CanvasNode;

use super::structs::{self, CanvasInterpretState, InterpreterError, Token};
use crate::view::elements::base::new_canvas;
use crate::view::elements::text::new_label;
use crate::{check_single_token, view::elements::base::new_panel};

use std::{cell::RefCell, rc::Rc};

macro_rules! new_node {
    ($new: expr, $parent: ident, $stack: ident, $token: ident) => {
        //create a node
        let temp_node: Rc<RefCell<CanvasNode>> = Rc::new(RefCell::new(CanvasNode::new(
            $new,
            Some(Rc::clone(&$parent)),
            vec![],
        )));
        //add the children to the current parent
        $parent.borrow_mut().children.push(Rc::clone(&temp_node));
        //switch the current parent to the created element
        $parent = Rc::clone(&temp_node);
        //push the node to the stack
        $stack.push((&$token, Rc::clone(&temp_node)));
    };
}

//this function takes in a piece of the token vector and the current index, and will return a tree representing the nodes
//the tree will be one canvas
pub fn parse_canvas(
    tokens: &[Token],
    index: &mut u32,
) -> Result<Rc<RefCell<CanvasNode>>, InterpreterError> {
    if tokens[0].content.as_str() != "<canvas>" {
        return Err(InterpreterError::Syntax(
            tokens[0].row,
            tokens[0].col,
            format!("expect <canvas> here"),
        ));
    }
    //initialize the stack
    let mut stack: Vec<(&Token, Rc<RefCell<CanvasNode>>)> = vec![];
    //initialize the interpret state
    let mut interpret_state: CanvasInterpretState = CanvasInterpretState::None;
    //initialize the result top node
    let result: Rc<RefCell<CanvasNode>> =
        Rc::new(RefCell::new(CanvasNode::new(new_canvas(), None, vec![])));
    //initialize the current parent node children list
    //this is a mutable reference to the children vector of the current parent node
    let mut current_parent_node: Rc<RefCell<CanvasNode>> = Rc::clone(&result);
    //push the result node to the stack
    stack.push((&tokens[0], Rc::clone(&result)));
    *index += 1;

    //initialize these for properties
    let mut current_property_name: String = format!("");
    //initialize the pair for fetching the segment of the tokens vector
    let mut property_value_token_range: (usize, usize) = (0, 0);
    //initialize the current index
    let mut current_index: usize = 0;
    //start parsing
    //start from the second element as the first is executed
    for token in tokens[1..].iter() {
        //increment the counter
        current_index += 1;

        //if it's this node, it's the end
        if token.content.as_str() == "</canvas>" {
            println!("");
            break;
        }
        //deal with the tags
        if structs::is_open_tag(token) {
            //if it's an open tag, push it to the stack
            //check for the tag types
            match token.content.as_str() {
                "<panel>" => {
                    new_node!(new_panel(), current_parent_node, stack, token);
                    continue;
                }
                "<label>" => {
                    new_node!(new_label(), current_parent_node, stack, token);
                    continue;
                }
                _ => {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        format!("{} is an unknown tag", token.content),
                    ));
                }
            };
        } else if structs::is_close_tag(token) {
            //if it's a close tag, match it to the last element on the stack
            //if the stack if empty, return an error
            if stack.len() == 0 {
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    format!("extra closing tag"),
                ));
            }

            if structs::is_closing_tag_to(token, stack.last().unwrap().0) {
                if stack.len() > 0 {
                    //pop the stack
                    stack.pop();

                    //if the node doesn't have a parent, there is an internal error
                    if let None = current_parent_node.borrow().parent {
                        return Err(InterpreterError::InternalError(
                            token.row,
                            token.col,
                            "try to backtrace the parent of the current node, but failed to find one"
                                .into(),
                        ));
                    }

                    //if the node has parent, change the current parent node to the parent
                    let temp = Rc::clone(&current_parent_node.borrow().parent.as_ref().unwrap());
                    current_parent_node = Rc::clone(&temp);

                    continue;
                } else {
                    return Err(InterpreterError::InternalError(
                        token.row,
                        token.col,
                        "for some reason it tries to pop the element while the stack is empty"
                            .into(),
                    ));
                }
            } else {
                //if doesn't match, throw an error
                return Err(InterpreterError::Syntax(
                    token.row,
                    token.col,
                    "mismatched tags".into(),
                ));
            }
        }

        //if it's not a tag, match the rest
        match interpret_state {
            CanvasInterpretState::None => {
                match token.content.as_str() {
                    "--" => {
                        //the start of a property
                        interpret_state = CanvasInterpretState::PropertyName;
                    }
                    _ => {
                        return Err(InterpreterError::Syntax(
                            token.row,
                            token.col,
                            format!("expect \"--\" here, found \"{}\"", token.content),
                        ));
                    }
                };
            }
            CanvasInterpretState::PropertyName => {
                //get the name
                current_property_name = String::from(&token.content);
                //check if the current serving element has this property or not
                if let None = current_parent_node
                    .borrow_mut()
                    .value
                    .properties
                    .get_mut(&current_property_name)
                {
                    return Err(InterpreterError::Property(
                        token.row,
                        token.col,
                        format!(
                            "property {} is not included in the current serving element",
                            &current_property_name
                        ),
                    ));
                } else {
                    interpret_state = CanvasInterpretState::PropertyColon;
                }
            }
            CanvasInterpretState::PropertyColon => {
                check_single_token!(
                    token,
                    interpret_state,
                    ":",
                    CanvasInterpretState::PropertyValue
                );

                //if this is the end of the tokens vector passed, it's an error
                if tokens.len() == current_index + 1 {
                    return Err(InterpreterError::Syntax(
                        token.row,
                        token.col,
                        "incomplete property declaration".into(),
                    ));
                }

                //now set the property range
                property_value_token_range = (current_index + 1, 0);
            }
            CanvasInterpretState::PropertyValue => {
                //append the property range until meeting the semicolon
                match token.content.as_str() {
                    ";" => {
                        interpret_state = CanvasInterpretState::None;
                        //pass this to the property parser

                        match property_parser::set_property_value(
                            Rc::clone(&current_parent_node),
                            &current_property_name,
                            &tokens[property_value_token_range.0..property_value_token_range.1 + 1],
                        ) {
                            Ok(..) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }

                    _ => {
                        //if it's not a semicolon, just append the pair
                        property_value_token_range = (property_value_token_range.0, current_index);
                    }
                }
            }
        };
    }
    Ok(result)
}
