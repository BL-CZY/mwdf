use std::collections::HashMap;

use super::super::structs::{ColorType, TextAlignType};
use super::{element_property_common, Element, ElementType, Property};

pub fn new_label() -> Element {
    let mut temp_properties: HashMap<String, Property> = HashMap::from([
        (format!("text"), Property::Text(format!("text"))),
        (format!("font"), Property::Font(format!("sans serif"))),
        (format!("text-color"), Property::TextColor(ColorType::new())),
        (
            format!("text-align"),
            Property::TextAlign(TextAlignType::Left),
        ),
    ]);

    temp_properties.extend(element_property_common());

    Element {
        element_type: ElementType::Label,
        properties: temp_properties,
    }
}
