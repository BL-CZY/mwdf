use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Panel {
    left_up: Point,
    right_bottom: Point,
    color: Color,    
}