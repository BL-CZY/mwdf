use crate::interpreter::structs::Token;

use std::boxed::Box;

pub struct CanvasNode {
    pub value: Token,
    pub parent: Option<Box<CanvasNode>>,
    pub children: Vec<Box<CanvasNode>>,
}

impl CanvasNode {
    pub fn new(value: Token, parent: Option<Box<CanvasNode>>, children: Vec<Box<CanvasNode>>) -> Self {
        Self { value, parent, children }
    }

    pub fn is_top_node(&self) -> bool {
        match self.parent {
            Some(..) => false,
            None => true,
        }
    }
}