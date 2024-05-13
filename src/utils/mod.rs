use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::canvas_parser::canvas_tree::CanvasNode;
use crate::view::elements::{Element, ElementType, Property};
use crate::view::structs::{ColorType, NumberType, PivotType, PointType, TextAlignType};

pub fn print_number_type(target: &NumberType) {
    match target {
        NumberType::Pixel(val) => print!("{}px", val),
        NumberType::Em(val) => print!("{}em", val),
        NumberType::Percent(val) => print!("{}%", val),
        NumberType::Number(val) => print!("{}", val),
    }
}

pub fn print_point_type(target: &PointType) {
    print!("(");
    print_number_type(&target.x);
    print!(", ");
    print_number_type(&target.y);
    print!(")")
}

pub fn print_pivot_type(target: &PivotType) {
    match target {
        PivotType::BottomCenter => print!("bottom center"),
        PivotType::BottomLeft => print!("bottom left"),
        PivotType::BottomRight => print!("bottom right"),
        PivotType::LeftCenter => print!("left center"),
        PivotType::RightCenter => print!("right center"),
        PivotType::TopCenter => print!("top center"),
        PivotType::TopLeft => print!("top left"),
        PivotType::TopRight => print!("top right"),
        PivotType::Center => print!("center"),
    }
}
pub fn print_color_type(target: &ColorType) {
    print!("({}, {}, {}, {})", target.r, target.g, target.b, target.a);
}

pub fn print_text_align(target: &TextAlignType) {
    match target {
        TextAlignType::Left => print!("left"),
        TextAlignType::Right => print!("right"),
    }
}

pub fn print_element(element: &Element, depth: u32) {
    for _ in 0..(depth * 5) {
        print!(" ");
    }

    print!("├───");

    print!(" ");

    print!("element type: ");
    //print the element type
    match element.element_type {
        ElementType::Canvas => println!("canvas"),
        ElementType::Label => println!("label"),
        ElementType::Panel => println!("panel"),
    }

    for property in &element.properties {
        for _ in 0..(depth * 3 + 5) {
            print!(" ");
        }
        print!("│     ");

        print!("--{}: ", property.0);

        match property.1 {
            Property::Width(val) => print_number_type(val),
            Property::Height(val) => print_number_type(val),
            Property::Position(val) => print_point_type(val),
            Property::PosRelToParent(val) => print!("{}", val),
            Property::Pivot(val) => print_pivot_type(val),
            Property::BackgroundColor(val) => print_color_type(val),
            Property::TextColor(val) => print_color_type(val),
            Property::Text(val) => print!("{}", val),
            Property::Font(val) => print!("{}", val),
            Property::TextAlign(val) => print_text_align(val),
        }

        println!("");
    }

    println!("");
}

//recursively print the value in a tree
pub fn print_canvas_tree(node: Rc<RefCell<CanvasNode>>, current_depth: u32) {
    for sub_node in &node.borrow().children {
        print_element(&sub_node.borrow().value, current_depth);
        print_canvas_tree(Rc::clone(sub_node), current_depth + 1);
    }
}
