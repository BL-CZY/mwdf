use std::collections::HashMap;

use super::{ element_property_common, Element, ElementType, Property };

pub fn new_canvas() -> Element {
    let mut temp_properties: HashMap<String, Property> = HashMap::from([]);

    temp_properties.extend(element_property_common());

    Element {
        element_type: ElementType::Canvas,
        properties: temp_properties,
    }
}

pub fn new_panel() -> Element {
    let mut temp_properties: HashMap<String, Property> = HashMap::from([]);

    temp_properties.extend(element_property_common());

    Element {
        element_type: ElementType::Panel,
        properties: temp_properties,
    }
}