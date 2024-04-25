use crate::view::elements::Element;

pub struct CanvasNode<'a> {
    pub value: &'a Element,
    pub parent: Option<&'a CanvasNode<'a>>,
    pub children: Vec<CanvasNode<'a>>,
}

impl<'a> CanvasNode<'a> {
    pub fn new(value: &'a Element, parent: Option<&'a CanvasNode>, children: Vec<CanvasNode<'a>>) -> Self {
        Self { value, parent, children }
    }

    pub fn is_top_node(&self) -> bool {
        match self.parent {
            Some(..) => false,
            None => true,
        }
    }
}