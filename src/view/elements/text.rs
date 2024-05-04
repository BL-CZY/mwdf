use std::collections::HashMap;

use super::{ element_property_common, Element, ElementType, Property };
use super::super::structs::{ ColorType, TextAlignType };

pub fn new_label() -> Element {
    let mut temp_properties: HashMap<String, Property> = HashMap::from([
        (format!("text"), Property::Text(format!("text"))),
        (format!("font"), Property::Text(format!("sans serif"))),
        (format!("font-color"), Property::TextColor(ColorType::new_empty())),
        (format!("text-align"), Property::TextAlign(TextAlignType::Left)),
    ]);

    temp_properties.extend(element_property_common());

    Element {
        element_type: ElementType::Label,
        properties: temp_properties,
    }
}