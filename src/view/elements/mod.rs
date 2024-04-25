pub mod base;

use base::{ Panel, Canvas };

pub enum Element {
    Canvas(Canvas),
    Panel(Panel),
}