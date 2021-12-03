#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) kind: TokenKind,
    pub(crate) pos: Position,
    pub(crate) lex: &'a str,
}

impl<'a> Token<'a> {
    pub(crate) fn new(kind: TokenKind, src: &'a str, start: usize, end: usize) -> Self {
        let lex = if end > src.len() || start >= src.len() {
            ""
        } else {
            &src[start..end]
        };
        Self {
            kind,
            pos: Position { start, end },
            lex,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub(crate) enum TokenKind {
    // single character
    LParen,
    // '('
    RParen,
    // ')'
    LBrace,
    // '{'
    RBrace,
    // '}'
    LBracket,
    // '['
    RBracket,
    // ']'
    Comma,
    // ','
    Dot,
    // '.'
    Semi,
    // ';'
    Colon,
    // ':'
    Not,
    // '!'
    Xor,
    // '^'

    // one or two character
    Plus,
    // '+'
    PlusAssign,
    // '+='
    Minus,
    // '-'
    MinusAssign,
    // '-='
    Star,
    // '*'
    MulAssign,
    // '*='
    Div,
    // '/'
    DivAssign,
    // '/='
    Mod,
    // '%'
    ModAssign,
    // '%='
    BitAnd,
    // '&'
    And,
    // '&&'
    BitOr,
    // '|'
    Or,
    // '||'
    Less,
    // '<'
    LessEqual,
    // '<='
    Greater,
    // '>'
    GreaterEqual,
    // '>='
    Assign,
    // '='
    Equal, // '=='

    // complex identifiers
    Ident,
    String,
    // "string"
    Keyword(Keyword),
    Number(Number),
    Char,

    // eof
    Eof,

    // Bad Token
    Unknown(String),
}

#[derive(Debug)]
pub enum Number {
    Float,
    Int,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
    For,
    While,
    Fn,
    If,
    Else,
    Return,
    Break,
    Continue,
    True,
    False,
    Struct,
}