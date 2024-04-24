pub mod canvas_tree;

use self::canvas_tree::CanvasNode;

use super::structs::{self, CanvasInterpretState, InterpreterError, Token};
use crate::view::elements::Element;

pub fn parse_canvas(tokens: &Vec<Token>, index: &mut u32) -> Result<Vec<Element>, InterpreterError> {
    //initialize the stack
    let mut stack: Vec<(&Token, &CanvasNode)> = vec![];
    //initialize the interpret state
    let mut interpret_state: CanvasInterpretState = CanvasInterpretState::None;
    //initialize the result vector
    let mut result: Vec<Element> = vec![];
    //initialize the graph
    let mut top_node: CanvasNode;
    //initialize the current serving node
    let mut current_node: CanvasNode;

    //start parsing
    for token in tokens.iter() {
        if structs::is_open_tag(token) {
            //if it's an open tag, push it to the stack
            //TODO push to the stack
        } else if structs::is_close_tag(token) {
            //if it's a close tag, match it to the last element on the stack
            //if the stack if empty, return an error
            if stack.len() == 0 {
                return Err(InterpreterError::Syntax(token.row, token.col, format!("mismatched tags")));
            }

            if structs::is_closing_tag_to(token, stack.last().unwrap().0) {
                //TODO deal with the stacks
            }
        }

        //if it's not a tag, match the rest
        match interpret_state {
            None => {
                match token.content.as_str() {

                };
            },
        };
    }
    Ok(vec![])
}