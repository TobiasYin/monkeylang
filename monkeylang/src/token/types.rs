#[derive(Debug, Clone, Default)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) pos: Position,
    pub(crate) lex: String,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, src: &str, start: usize, end: usize) -> Self {
        let lex = if end > src.len() || start >= src.len() {
            ""
        } else {
            &src[start..end]
        };
        Self {
            kind,
            pos: Position { start, end },
            lex: lex.to_string(),
        }
    }
}

pub struct TokenStream {
    pub(crate) tokens :Vec<Token>,
    pub(crate) now_at: usize,
}

impl TokenStream {
    pub fn is_end(&self) ->bool {
        self.tokens.len() <= self.now_at
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
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

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::Unknown("default".to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Float,
    Int,
}

#[derive(Debug, Clone)]
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