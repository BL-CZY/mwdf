pub enum TokenParseState {
    None,
    Tag,
    Var,
    Str,
    Builtin,
    TwoCharToken,
    Property,
    Vector,
    Comment,
}

#[derive(PartialEq)]
pub enum MacroHashState {
    None,
    VarName,
    VarEqual,

    VarArgsStart,
    VarArgsDollar,
    VarArgsName,
    VarArgsNext,
    VarArgsEnd,

    VarContentStart,
    VarContent,
    VarContentEnd,
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

pub enum MacroListElement<'a> {
    Token(&'a Token),
    Arg(&'a str),
}

pub enum NumberParseState {
    None,
    Number,
    Next,
    Finish,
}

pub struct Token {
    pub content: String,
    pub row: u32,
    pub col: u32,
}

impl Token {
    pub fn new(content: String, row: u32, col: u32) -> Self {
        Self { content, row, col }
    }
}

pub fn is_open_tag(token: &Token) -> bool {
    if token.content.chars().count() < 2 {
        //if it's too short, false
        return false;
    }

    //this is a close tag
    if token.content.chars().nth(1).unwrap() == '/' {
        return false;
    }

    if token.content.chars().nth(0).unwrap() == '<' && token.content.chars().last().unwrap() == '>'
    {
        //if the first element is <, true
        true
    } else {
        false
    }
}

pub fn is_close_tag(token: &Token) -> bool {
    if token.content.chars().count() < 3 {
        false
    } else if token.content.chars().nth(0).unwrap() == '<'
        && token.content.chars().nth(1).unwrap() == '/'
    {
        true
    } else {
        false
    }
}

pub fn is_closing_tag_to(expected_closing_tag: &Token, opening_tag: &Token) -> bool {
    if !is_open_tag(opening_tag) || !is_close_tag(expected_closing_tag) {
        return false;
    } else if expected_closing_tag.content.chars().count()
        != opening_tag.content.chars().count() + 1
    {
        return false;
    } else if &expected_closing_tag.content[3..] == &opening_tag.content[2..] {
        return true;
    }

    false
}
