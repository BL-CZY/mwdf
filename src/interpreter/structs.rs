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
    PropertyName,
    PropertyColon,
    PropertyValue,
}

#[derive(PartialEq)]
pub enum InterpreterError {
    Syntax(u32, u32, String),
    Property(u32, u32, String),
    InternalError(u32, u32, String),
    InvalidFile,
    EmptyFile,
    DecodingError,
}

pub enum VarListElement {
    Token(Token),
    ArgDescriptor(ArgDescriptor),
}

pub enum NumberParseState {
    None,
    FirstDigit,
    Digit,
    End,
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

    pub fn from(token: &Token) -> Self {
        Self {
            content: String::from(&token.content),
            row: token.row,
            col: token.col,
        }
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

pub fn is_open_tag(token: &Token) -> bool {
    if token.content.chars().count() < 2 {
        //if it's too short, false
        false
    } else if token.content.chars().nth(0).unwrap() == '<' {
        //if the first element is <, true
        true
    } else {
        false
    }
}

pub fn is_close_tag(token: &Token) -> bool {
    if token.content.chars().count() < 3 {
        false
    } else if token.content.chars().nth(0).unwrap() == '<' && token.content.chars().nth(1).unwrap() == '/' {
        true
    } else {
        false
    }
}

pub fn is_closing_tag_to(expected_closing_tag: &Token, opening_tag: &Token) -> bool {
    if !is_open_tag(opening_tag) || !is_close_tag(expected_closing_tag) {
        return false;
    } else if expected_closing_tag.content.chars().count() != opening_tag.content.chars().count() + 1 {
        return false;
    } else if &expected_closing_tag.content[3..] == &opening_tag.content[2..] {
        return true;
    }

    false
}