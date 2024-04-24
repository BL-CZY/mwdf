pub mod canvas_tree;

use self::canvas_tree::CanvasNode;

use super::structs::{self, CanvasInterpretState, InterpreterError, Token};
use crate::view::elements::Element;

pub fn parse_canvas(tokens: &Vec<Token>, index: &mut u32) -> Result<Vec<Element>, InterpreterError> {
    if tokens[*index as usize].content.as_str() != "<canvas>" {
        return Err(InterpreterError::Syntax(tokens[*index as usize].row, tokens[*index as usize].col, format!("expect <canvas> here")));
    }
    //initialize the stack
    let mut stack: Vec<(&Token, Option<&CanvasNode>)> = vec![];
    //initialize the interpret state
    let mut interpret_state: CanvasInterpretState = CanvasInterpretState::None;
    //initialize the result vector
    let mut result: Vec<Element> = vec![];
    //initialize the graph
    let top_node: Option<CanvasNode> = Some(CanvasNode::new(&tokens[*index as usize], None, vec![]));
    //initialize the current serving node
    let mut current_node: Option<&CanvasNode> = top_node.as_ref();

    stack.push((&tokens[*index as usize], current_node));

    //start parsing
    for token in tokens.iter() {
        //at the beginning, check for the <canvas> node, which will be at the top of the graph
        if stack.len() == 0 {
            if token.content.as_str() != "<canvas>" {
                return Err(InterpreterError::Syntax(token.row, token.col, format!("expect <canvas> here")));
            } else {
                //initialize the top node
                stack.push((&token, top_node.as_ref()))
            }
        }

        //deal with the tags
        if structs::is_open_tag(token) {
            //if it's an open tag, push it to the stack

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
            CanvasInterpretState::None => {
                match token.content.as_str() {
                    _ => {},
                };
            },
            _ => {},
        };
    }
    Ok(vec![])
}