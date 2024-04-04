pub mod panel;

use crate::view::elements::panel::Panel;

pub enum Element {
    Panel(Panel),
}