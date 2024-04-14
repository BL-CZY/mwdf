pub enum TokenParseState {
    None,
    Tag,
    Var,
    Str,
    Builtin,
    TwoCharToken,
    Mark,
    Property,
    Vector,
    Comment,
}

//so that i can compare it using ==
#[derive(PartialEq)]
pub enum TokenConvertState {
    Var,
    Canvas,
}

pub enum VarHashState {
    None,
    VarName,
    VarType,
    VarDefStrEqual,
    VarDefStrQuota,
    VarDefStrContent,
    VarDefFont,
    VarDefVec,
    VarDefExp,
}

pub enum CanvasInterpretState {
    None,
    Tag,
    Property,
}

pub enum InterpreterError {
    Syntax(u32, u32, String),
    NoClosingTag(u32, u32),
    MissingEntryTag(u32, u32),
    MultipleEntryTag(u32, u32),
    IllegalTagStart(u32, u32),
    BrokenSection(u32, u32),
    InvalidToken(u32, u32),
    InvalidFile,
    EmptyFile,
    DecodingError,
}

pub struct Token {
    pub content: String,
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new(content: String, row: u32, col: u32) -> Self {
        Self {content, row, col}
    }

    pub fn is_closing_tag(&mut self) -> bool {
        if self.content.chars().count() < 2 {
            return false;
        }

        if &self.content.chars().take(2).collect::<String>() == "</" {
            return true;
        }

        false
    }

    pub fn is_mathing_closing_tag_to(&mut self, tag: Token) -> bool {
        //bad length
        if tag.content.chars().count() <= 2 {
            return false;
        }

        //check if the rest are the same
        let str: &str = &tag.content[2..];
        if tag.content.chars().skip(2).collect::<String>() == self.content.chars().skip(1).collect::<String>() {
            //check if the tag comes after self
            if self.row < tag.row {
                return true;
            } else if self.row == tag.row && self.col < tag.col {
                return true;
            }
        }

        false
    }
}