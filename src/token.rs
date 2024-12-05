#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Symbol(String),
    Identifier(String),
    Number {
        literal: String,
        kind: NumberKind,
        base: NumberBase,
        suffix: Option<NumberSuffix>,
    },
    CharLiteral(String),
    StringLiteral(String),
    Comment(String),
    Error(String),
}

impl Token {
    pub fn raw(&self) -> String {
        match self {
            Token::Keyword(s)             => s.to_string(),
            Token::Symbol(s)              => s.to_string(),
            Token::Identifier(s)          => s.to_string(),
            Token::Number { literal, .. } => literal.to_string(),
            Token::CharLiteral(s)         => s.to_string(),
            Token::StringLiteral(s)       => s.to_string(),
            Token::Comment(s)             => s.to_string(),
            Token::Error(s)               => s.to_string(),
        }
    }

    pub fn kind(&self) -> String {
        match self {
            Token::Keyword(_)       => "Keyword".to_string(),
            Token::Symbol(_)        => "Symbol".to_string(),
            Token::Identifier(_)    => "Identifier".to_string(),
            Token::Number { .. }    => "Number".to_string(),
            Token::CharLiteral(_)   => "CharLiteral".to_string(),
            Token::StringLiteral(_) => "StringLiteral".to_string(),
            Token::Comment(_)       => "Comment".to_string(),
            Token::Error(_)         => "Error".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct PositionedToken {
    pub token: Token,
    pub row: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct TokenLine {
    pub line_number: usize,
    pub tokens: Vec<PositionedToken>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberKind {
    Integer,
    FloatingPoint,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberBase {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberSuffix {
    Integer(IntegerSuffix),
    FloatingPoint(FloatingPointSuffix),
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerSuffix {
    unsigned: bool,
    long: u8, // 0 for none, 1 for 'L', 2 for 'LL'
}

impl IntegerSuffix {
    pub fn new(unsigned: bool, long: u8) -> Self {
        IntegerSuffix { unsigned, long }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FloatingPointSuffix {
    Float,        // 'f' or 'F'
    LongDouble,   // 'l' or 'L'
}

pub const KEYWORDS: &[&str] = &[
    "if",
    "else",
    "while",
    "do",
    "main",
    "int",
    "float",
    "double", 
    "char",
    "unsigned",
    "long",
    "return",
    "const",
    "void",
    "switch",
    "case",
    "continue",
    "break",
    "enum",
    "struct",
    "static",
    "for",
    "typedef",
];

pub const SYMBOLS: &[&str] = &[
    "+",
    "-",
    "*",
    "/",
    "=",
    "<",
    ">",
    "!",
    "==",
    "!=",
    "&&",
    "||",
    "<=",
    ">=",
    "{",
    "}",
    "[",
    "]",
    "(",
    ")",
    ";",
    ",",
    ".",
    ":",
    "&",
    "|",
    "^",
    "~",
];