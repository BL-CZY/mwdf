use crate::view::elements::Element;

pub struct CanvasNode<'a> {
    pub value: Element,
    pub parent: Option<&'a CanvasNode<'a>>,
    pub children: Vec<CanvasNode<'a>>,
}

impl<'a> CanvasNode<'a> {
    pub fn new(value: Element, parent: Option<&'a CanvasNode>, children: Vec<CanvasNode<'a>>) -> Self {
        Self { value, parent, children }
    }
}