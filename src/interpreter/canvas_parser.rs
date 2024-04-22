use super::structs::{self, InterpreterError, Token};
use crate::view::elements::Element;

pub fn parse_canvas(tokens: &Vec<Token>, index: &mut u32) -> Result<Vec<Element>, InterpreterError> {
    Ok(vec![])
}