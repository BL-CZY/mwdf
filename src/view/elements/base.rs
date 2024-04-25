use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Canvas {
    background_color: Color,
}

pub struct Panel {
    left_up: Point,
    right_bottom: Point,
    background_color: Color,
}