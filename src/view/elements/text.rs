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
        Self {
            start: Point::new(0, 0),
            content: String::from(""),
            text_color: Color::RGB(0, 0, 0),
            font: String::from(""),
        }
    }
}