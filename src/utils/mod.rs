use colored::Colorize;
use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::canvas_parser::canvas_tree::CanvasNode;
use crate::view::elements::{Element, ElementType, Property};
use crate::view::structs::{ColorType, NumberType, PivotType, PointType, TextAlignType};

pub fn print_number_type(target: &NumberType) {
    match target {
        NumberType::Pixel(val) => print!("{}{}", val.to_string().yellow(), "px".yellow()),
        NumberType::Em(val) => print!("{}{}", val.to_string().yellow(), "em".yellow()),
        NumberType::Percent(val) => print!("{}", val.to_string().yellow()),
        NumberType::Number(val) => print!("{}", val.to_string().yellow()),
    }
}

pub fn print_point_type(target: &PointType) {
    print!("(");
    print_number_type(&target.x);
    print!(", ");
    print_number_type(&target.y);
    print!(")");
}

pub fn print_pivot_type(target: &PivotType) {
    match target {
        PivotType::BottomCenter => print!("{}", "bottom center".yellow()),
        PivotType::BottomLeft => print!("{}", "bottom left".yellow()),
        PivotType::BottomRight => print!("{}", "bottom right".yellow()),
        PivotType::LeftCenter => print!("{}", "left center".yellow()),
        PivotType::RightCenter => print!("{}", "right center".yellow()),
        PivotType::TopCenter => print!("{}", "top center".yellow()),
        PivotType::TopLeft => print!("{}", "top left".yellow()),
        PivotType::TopRight => print!("{}", "top right".yellow()),
        PivotType::Center => print!("{}", "center".yellow()),
    }
}
pub fn print_color_type(target: &ColorType) {
    print!(
        "({}, {}, {}, {})",
        target.r.to_string().yellow(),
        target.g.to_string().yellow(),
        target.b.to_string().yellow(),
        target.a.to_string().yellow()
    );
}

pub fn print_text_align(target: &TextAlignType) {
    match target {
        TextAlignType::Left => print!("{}", "left".yellow()),
        TextAlignType::Right => print!("{}", "right".yellow()),
    }
}

pub fn print_element(element: &Element, depth: u32) {
    for _ in 0..depth {
        print!("│   ");
    }

    print!("├── ");

    print!("{}", "Element type: ".blue());
    //print the element type
    match element.element_type {
        ElementType::Canvas => println!("{}", "canvas".green()),
        ElementType::Label => println!("{}", "label".green()),
        ElementType::Panel => println!("{}", "panel".green()),
    }

    for property in &element.properties {
        for _ in 0..depth + 1 {
            print!("│   ");
        }

        print!("│ {}{}: ", "--".cyan(), property.0.as_str().cyan());

        match property.1 {
            Property::Width(val) => print_number_type(val),
            Property::Height(val) => print_number_type(val),
            Property::Position(val) => print_point_type(val),
            Property::PosRelToParent(val) => print!("{}", val.to_string().as_str().yellow()),
            Property::Pivot(val) => print_pivot_type(val),
            Property::BackgroundColor(val) => print_color_type(val),
            Property::TextColor(val) => print_color_type(val),
            Property::Text(val) => print!("{}", val.to_string().as_str().yellow()),
            Property::Font(val) => print!("{}", val.to_string().as_str().yellow()),
            Property::TextAlign(val) => print_text_align(val),
        }

        println!("");
    }
    for _ in 0..depth + 2 {
        print!("│   ");
    }
    println!("");
}

//recursively print the value in a tree
pub fn print_canvas_tree(node: Rc<RefCell<CanvasNode>>, current_depth: u32) {
    print_element(&node.borrow().value, current_depth);
    for sub_node in &node.borrow().children {
        print_canvas_tree(Rc::clone(sub_node), current_depth + 1);
    }
}
