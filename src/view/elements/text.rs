use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Label {
    pub start: Point,
    pub content: String,
    pub text_color: Color,
    pub font: String,
}

impl Label {
    pub fn new() -> Self {

    }
}