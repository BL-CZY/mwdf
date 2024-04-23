use crate::interpreter::structs::Token;

use std::rc::Rc;

pub struct CanvasNode {
    pub value: Token,
    pub parent: Option<Rc<CanvasNode>>,
    pub children: Vec<Rc<CanvasNode>>,
}

impl CanvasNode {
    pub fn new(value: Token, parent: Option<Rc<CanvasNode>>, children: Vec<Rc<CanvasNode>>) -> Self {
        Self { value, parent, children }
    }

    pub fn is_top_node(&self) -> bool {
        match self.parent {
            Some(..) => false,
            None => true,
        }
    }
}