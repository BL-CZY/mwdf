pub enum NumberType {
    Int(i32),
    Float(f32),
    Percent(f32),
}

pub struct ColorType {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorType {
    pub fn new() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

pub struct PointType {
    pub x: NumberType,
    pub y: NumberType,
}

impl PointType {
    pub fn new() -> Self {
        Self {
            x: NumberType::Int(0),
            y: NumberType::Int(0),
        }
    }
}

pub enum PivotType {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    LeftCenter,
    RightCenter,
    Center,
    Custom(u32, u32),
}

pub enum TextAlignType {
    Left,
    Right,
}