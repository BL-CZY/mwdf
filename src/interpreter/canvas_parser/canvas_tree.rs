use crate::view::elements::Element;

use std::{cell::RefCell, rc::Rc};

pub struct CanvasNode {
    pub value: Element,
    pub parent: Option<Rc<RefCell<CanvasNode>>>,
    pub children: Vec<Rc<RefCell<CanvasNode>>>,
}

impl CanvasNode {
    pub fn new(
        value: Element,
        parent: Option<Rc<RefCell<CanvasNode>>>,
        children: Vec<Rc<RefCell<CanvasNode>>>,
    ) -> Self {
        Self {
            value,
            parent,
            children,
        }
    }
}
