//it's just here to avoid precision lost
pub struct FloatType {
    pub left: u32,
    pub right: u32,
}

impl FloatType {
    pub fn get_float() -> f64 {
        //TODO combine the left and right part without precision lost
        0.00
    }
}

pub enum NumberType {
    Pixel(u32),
    Em(FloatType),
    Percent(FloatType),
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
            x: NumberType::Pixel(0),
            y: NumberType::Pixel(0),
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