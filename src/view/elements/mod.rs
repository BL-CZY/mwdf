pub mod base;
pub mod text;

use super::structs::{ ColorType, PivotType, PointType, TextAlignType };

use std::collections::HashMap;

pub enum ElementType {
    Canvas,
    Panel,
}

pub enum Property {
    //* transform related
    Width(u32),
    Height(u32),
    Position(PointType),
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
    element_type: ElementType,
    properties: HashMap<String, Property>,
}

pub fn element_property_common() -> HashMap<String, Property> {
    HashMap::from([
        //* transform related
        (format!("--width"), Property::Width(0)),
        (format!("--height"), Property::Height(0)),
        (format!("--position"), Property::Position(PointType::new_empty())),
        (format!("--pivot"), Property::Pivot(PivotType::TopLeft)),

        //* color related
        (format!("--background-color"), Property::BackgroundColor(ColorType::new_empty())),
    ])
}

macro_rules! new_element {
    ($type: ident, $extra: expr) => {
        let mut temp_properties: HashMap<String, Property> = HashMap::from([
            $expr
        ]);
    
        temp_properties.extend(element_property_common());
    
        Element {
            element_type: $type,
            properties: temp_properties,
        }
    };
}