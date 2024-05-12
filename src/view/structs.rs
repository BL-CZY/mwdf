pub enum NumberType {
    Pixel(u32),
    Em(f32),
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
            x: NumberType::Pixel(0),
            y: NumberType::Pixel(0),
        }
    }

    pub fn from(x: NumberType, y: NumberType) -> Self {
        Self { x, y }
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
}

pub enum TextAlignType {
    Left,
    Right,
}
