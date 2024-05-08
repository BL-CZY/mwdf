use crate::interpreter::structs::Token;

use super::canvas_tree::CanvasNode;

use std::{cell::RefCell, rc::Rc};

//this function takes in the current serving node, the current serving property name, and the tokens representing the property
//this function sets the property
pub fn set_property_value(node: Rc<RefCell<CanvasNode>>, property_name: &String, tokens: &[Token]) {

}