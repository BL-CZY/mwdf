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