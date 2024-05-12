pub mod base;
pub mod text;

use super::structs::{ColorType, NumberType, PivotType, PointType, TextAlignType};

use std::collections::HashMap;

pub enum ElementType {
    Canvas,
    Panel,
    Label,
}

//* naming convention:
//* ExampleText -> example-text
pub enum Property {
    //* transform related
    Width(NumberType),
    Height(NumberType),
    Position(PointType),
    PosRelToParent(bool),
    Pivot(PivotType),

    //* color related
    BackgroundColor(ColorType),
    TextColor(ColorType),

    //* text related
    Text(String),
    Font(String),
    TextAlign(TextAlignType),
}

pub struct Element {
    pub element_type: ElementType,
    pub properties: HashMap<String, Property>,
}

pub fn element_property_common() -> HashMap<String, Property> {
    HashMap::from([
        //* transform related
        (format!("width"), Property::Width(NumberType::Pixel(0))),
        (format!("height"), Property::Height(NumberType::Pixel(0))),
        (format!("position"), Property::Position(PointType::new())),
        (format!("pos-rel-to-parent"), Property::PosRelToParent(true)),
        (format!("pivot"), Property::Pivot(PivotType::TopLeft)),
        //* color related
        (
            format!("background-color"),
            Property::BackgroundColor(ColorType::new()),
        ),
    ])
}
