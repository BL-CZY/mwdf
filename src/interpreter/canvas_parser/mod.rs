pub mod canvas_tree;

use self::canvas_tree::CanvasNode;

use super::structs::{self, CanvasInterpretState, InterpreterError, Token};
use crate::view::elements::Element;

pub fn parse_canvas(tokens: &Vec<Token>, index: &mut u32) -> Result<Vec<Element>, InterpreterError> {
    //initialize the stack
    let mut stack: Vec<&Token> = vec![];
    //initialize the graph
    let mut graph: Vec<CanvasNode> = vec![];
    //initialize the interpret state
    let mut interpret_state: CanvasInterpretState = CanvasInterpretState::None;
    //initialize the result vector
    let mut result: Vec<Element> = vec![];

    //start parsing
    for token in tokens.iter() {
        if structs::is_open_tag(token) {
            //if it's an open tag, push it to the stack
            stack.push(&token);
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