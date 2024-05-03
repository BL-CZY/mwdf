use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::Property;

pub struct Canvas {
    pub options: HashMap<String, Property>,
    pub background_color: Color,
}

impl Canvas {
    pub fn new() -> Self {
        Self { background_color: Color::RGB(0, 0, 0) }
    }
}

pub struct Panel {
    left_up: Point,
    right_bottom: Point,
    background_color: Color,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            left_up: Point::new(0, 0),
            right_bottom: Point::new(0, 0),
            background_color: Color::RGB(0, 0, 0),
        }
    }
}