use super::string_parser;
use crate::{
    interpreter::structs::{InterpreterError, Token},
    view::structs::PivotType,
};

pub fn parse_pivot(tokens: &[Token]) -> Result<PivotType, InterpreterError> {
    let result: PivotType;

    match string_parser::parse_string(tokens) {
        Ok(val) => match val.as_str() {
            "top-left" => {
                result = PivotType::TopLeft;
            }
            "top-center" => {
                result = PivotType::TopCenter;
            }
            "top-right" => {
                result = PivotType::TopRight;
            }
            "bottom-left" => {
                result = PivotType::BottomLeft;
            }
            "bottom-right" => {
                result = PivotType::BottomRight;
            }
            "left-center" => {
                result = PivotType::LeftCenter;
            }
            "right-center" => {
                result = PivotType::RightCenter;
            }
            "center" => {
                result = PivotType::Center;
            }

            _ => {
                return Err(InterpreterError::Property(
                    tokens[0].row,
                    tokens[0].col,
                    format!("\"{}\" is not in the Pivot type", val.as_str()),
                ));
            }
        },

        Err(e) => return Err(e),
    }

    Ok(result)
}
