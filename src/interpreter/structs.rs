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

#[derive(PartialEq)]
pub enum VarHashState {
    None,
    VarName,
    VarTypeColon,
    VarType,

    VarDefStrEqual,
    VarDefStrQuota,
    VarDefStrContent,

    VarDefFontEqual,
    VarDefFontQuota,
    VarDefFontContent,

    VarDefVecEqual,
    VarDefVecParenth,
    VarDefVecContent,

    VarDefExpEqual,
    VarDefExpArgsBracket,
    VarDefExpArgsDollar,
    VarDefExpArgsName,
    VarDefExpArgsColon,
    VarDefExpArgsType,
    VarDefExpArgsNext,

    VarDefExpArrow,
    VarDefExpContentBracket,
    VarDefExpContent,

    VarSemiColon,
}

pub enum CanvasInterpretState {
    None,
    Tag,
    Property,
}

#[derive(PartialEq)]
pub enum InterpreterError {
    Syntax(u32, u32, String),
    InternalError(String),
    InvalidFile,
    EmptyFile,
    DecodingError,
}

pub enum VarListElement {
    Token(Token),
    ArgDescriptor(ArgDescriptor),
}

pub struct Token {
    pub content: String,
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new_empty() -> Self {
        Self {
            content: format!(""),
            row: 0,
            col: 0,
        }
    }

    pub fn new(content: String, row: u32, col: u32) -> Self {
        Self {content, row, col}
    }

    pub fn from(token: &Token) -> Self {
        Self {
            content: String::from(&token.content),
            row: token.row,
            col: token.col,
        }
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

pub struct ArgDescriptor {
    pub arg_num: u32,
}

impl ArgDescriptor {
    pub fn new() -> Self {
        Self {
            arg_num: 0,
        }
    }
}