pub mod canvas_tree;

use self::canvas_tree::CanvasNode;

use super::structs::{self, CanvasInterpretState, InterpreterError, Token};
use crate::view::elements::{base::{Canvas, Panel}, Element};

use std::{cell::RefCell, rc::Rc};

macro_rules! new_node {
    ($element: literal, $current_parent: expr) => {
        //create an element
        let temp_ele: $element = $element::new();
        let temp_node: Rc<RefCell<CanvasNode>> = Rc::new(RefCell::new(CanvasNode::new(
            Element::Panel(temp_ele),
            Some(Rc::clone(&$current_parent)), 
            vec![])));
        $current_parent.borrow_mut().children.push(Rc::clone(&temp_node));
        $current_parent = Rc::clone(&temp_node);
    };
}

//this function takes in the tokens and the current index, and will return a tree representing the nodes
pub fn parse_canvas<'a>(tokens: &Vec<Token>, index: &mut u32) -> Result<Rc<RefCell<CanvasNode>>, InterpreterError> {
    if tokens[*index as usize].content.as_str() != "<canvas>" {
        return Err(InterpreterError::Syntax(tokens[*index as usize].row, tokens[*index as usize].col, format!("expect <canvas> here")));
    }
    //initialize the stack
    let mut stack: Vec<(&Token, Rc<RefCell<CanvasNode>>)> = vec![];
    //initialize the interpret state
    let mut interpret_state: CanvasInterpretState = CanvasInterpretState::None;
    //initialize the result top node
    let mut result: Rc<RefCell<CanvasNode>> = Rc::new(RefCell::new(CanvasNode::new(Element::Canvas(Canvas::new()), None, vec![])));
    //initialize the current parent node children list
    //this is a mutable reference to the children vector of the current parent node
    let mut current_parent_node: Rc<RefCell<CanvasNode>> = Rc::clone(&result);
    //push the result node to the stack
    stack.push((&tokens[*index as usize], Rc::clone(&result)));
    *index += 1;

    //start parsing
    for token in tokens.iter() {
        //if it's the last element, stop
        if (*index as usize) >= tokens.len() {
            break;
        }

        //deal with the tags
        if structs::is_open_tag(token) {
            //if it's an open tag, push it to the stack
            //check for the tag types
            match token.content.as_str() {
                "<panel>" => {
                    //create an element
                    let temp_ele: Panel = Panel::new();
                    let temp_node: Rc<RefCell<CanvasNode>> = Rc::new(RefCell::new(CanvasNode::new(
                        Element::Panel(temp_ele),
                        Some(Rc::clone(&current_parent_node)), 
                        vec![])));
                    current_parent_node.borrow_mut().children.push(Rc::clone(&temp_node));
                    current_parent_node = Rc::clone(&temp_node);
                },
                "<label>" => {},
                _ => {
                    return Err(InterpreterError::Syntax(token.row, token.col, format!("unknown tag")));
                },
            };
        } else if structs::is_close_tag(token) {
            //if it's a close tag, match it to the last element on the stack
            //if the stack if empty, return an error
            if stack.len() == 0 {
                return Err(InterpreterError::Syntax(token.row, token.col, format!("extra closing tag")));
            }

            if structs::is_closing_tag_to(token, stack.last().unwrap().0) {
                if stack.len() > 0 {
                    stack.pop();
                } else {
                    return Err(InterpreterError::InternalError(format!("for some reason it tries to pop the element while the stack is empty")))
                }
            } else {
                //if doesn't match, throw an error
                return Err(InterpreterError::Syntax(token.row, token.col, format!("mismatched tags")));
            }
        }

        //if it's not a tag, match the rest
        match interpret_state {
            CanvasInterpretState::None => {
                match token.content.as_str() {
                    _ => {},
                };
            },
            _ => {},
        };
    }
    Ok(result)
}