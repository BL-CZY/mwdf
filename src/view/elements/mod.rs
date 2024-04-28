pub mod base;
pub mod text;

use base::{ Panel, Canvas };

pub enum Element {
    Canvas(Canvas),
    Panel(Panel),
}