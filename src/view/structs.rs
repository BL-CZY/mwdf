pub struct ColorType {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorType {
    pub fn new_empty() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from(color: ColorType) -> Self {
        Self {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

pub struct PointType {
    pub x: u32,
    pub y: u32,
}

impl PointType {
    pub fn new_empty() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn from(pt: PointType) -> Self {
        Self {
            x: pt.x,
            y: pt.y,
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