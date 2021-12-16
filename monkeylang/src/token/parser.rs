use crate::token::types::{Token, TokenKind, TokenStream};
use crate::token::types::Number;
use crate::token::types::Keyword;

struct Parser<'a> {
    raw: &'a str,
    source: Vec<char>,
    pos: usize,
    tokens: Vec<Token>,
}


impl<'a> Parser<'a> {
    pub(crate) fn parse_tokens(&mut self) {
        while !self.is_eof() {
            if self.match_space() {
                self.pos += 1;
                continue;
            }
            if self.match_single() {
                continue;
            }
            if self.match_one_or_two() {
                continue;
            }
            if self.match_complex() {
                continue;
            }
            self.add_unknown();
        }
        self.add_token_cur_end(TokenKind::Eof, self.pos);
    }

    fn is_eof(&self) -> bool {
        return self.pos >= self.source.len();
    }

    fn add_token(&mut self, kind: TokenKind, start: usize) {
        self.tokens.push(Token::new(kind, &self.raw, start, self.pos + 1));
    }

    fn add_token_cur_end(&mut self, kind: TokenKind, start: usize) {
        self.tokens.push(Token::new(kind, &self.raw, start, self.pos));
    }


    fn match_space(&self) -> bool {
        let now = self.source[self.pos];
        now == ' ' || now == '\t' || now == '\n' || now == '\r'
    }

    fn match_bound(&self) -> bool {
        if self.match_space() {
            return true;
        }
        let now = self.source[self.pos];
        now == '(' || now == ')' || now == '[' || now == ']' || now == '{' || now == '}' || now == ',' || now == '+' || now == '-'
            || now == '*' || now == '/' || now == '%' || now == '^' || now == '&' || now == '|' || now == '~' || now == '!' || now == '='
            || now == '<' || now == '>' || now == ':' || now == ';' || now == '.'
    }

    fn match_single(&mut self) -> bool {
        macro_rules! simple_token {
            ($($char:expr => $kind: expr),+) => {
                {
                    let now = self.source[self.pos];
                    match now {
                        $(
                        $char => {
                            self.add_token($kind, self.pos);
                            self.pos += 1;
                            true
                        }
                        )*
                        _ => {
                            false
                        }
                    }
                }
            }
        }

        simple_token! {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semi,
            ':' => TokenKind::Colon,
            '!' => TokenKind::Not,
            '^' => TokenKind::Xor
        }
    }

    fn match_one_or_two(&mut self) -> bool {
        macro_rules! one_two_token {
            ($($char1:expr, $char2: expr => $kind1: expr, $kind2: expr),+) => {
                {
                    let now = self.source[self.pos];
                    match now {
                        $(
                        $char1 => {
                            let next_pos = self.pos + 1;
                            if next_pos >= self.source.len(){
                                self.add_token($kind1, self.pos);
                                self.pos += 1;
                            } else {
                                let next = self.source[next_pos];
                                if next == $char2{
                                    self.add_token($kind2, next_pos);
                                    self.pos += 2;
                                } else {
                                    self.add_token($kind1, self.pos);
                                    self.pos += 1;
                                }
                            }
                            true
                        }
                        )*
                        _ => {
                            false
                        }
                    }
                }
            }
        }
        one_two_token! {
            '+', '=' => TokenKind::Plus, TokenKind::PlusAssign,
            '-', '=' => TokenKind::Minus, TokenKind::MinusAssign,
            '*', '=' => TokenKind::Star, TokenKind::MulAssign,
            '/', '=' => TokenKind::Div, TokenKind::DivAssign,
            '%', '=' => TokenKind::Mod, TokenKind::ModAssign,
            '&', '&' => TokenKind::BitAnd, TokenKind::And,
            '|', '|' => TokenKind::BitOr, TokenKind::Or,
            '<', '=' => TokenKind::Less, TokenKind::LessEqual,
            '>', '=' => TokenKind::Greater, TokenKind::GreaterEqual,
            '=', '=' => TokenKind::Assign, TokenKind::Equal
        }
    }

    fn add_unknown(&mut self) {
        self.add_token(TokenKind::Unknown(String::from("unknown character")), self.pos);
        self.pos += 1;
    }

    fn match_complex(&mut self) -> bool {
        let now = self.source[self.pos];
        match now {
            '\'' => self.match_char(),
            '"' => self.match_string(),
            '0'..='9' => self.match_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.match_identifier(),
            _ => false
        }
    }

    fn match_char(&mut self) -> bool {
        let start = self.pos;
        self.pos += 1;
        let mut len = 0;
        let mut escape = false;
        let mut is_match = false;
        let mut is_end_line = false;
        while !self.is_eof() {
            let now = self.source[self.pos];
            if now == '\n' {
                is_end_line = true;
                self.pos += 1;
                break;
            }
            if now == '\'' && !escape {
                is_match = true;
                self.pos += 1;
                break;
            }
            if now == '\\' && !escape {
                escape = true;
                self.pos += 1;
                continue;
            }
            if escape {
                escape = false;
            }
            len += 1;
            self.pos += 1;
        }
        if is_match {
            if len != 1 {
                self.add_token_cur_end(TokenKind::Unknown(String::from("char only support one character, consider use string ")), start);
            } else {
                self.add_token_cur_end(TokenKind::Char, start);
            }
        } else {
            if is_end_line {
                self.add_token_cur_end(TokenKind::Unknown(String::from("end line before char end")), start);
            } else {
                self.add_token_cur_end(TokenKind::Unknown(String::from("eof char end")), start);
            }
        }
        true
    }

    fn match_string(&mut self) -> bool {
        let start = self.pos;
        self.pos += 1;
        let mut escape = false;
        let mut is_match = false;
        let mut is_end_line = false;
        while !self.is_eof() {
            let now = self.source[self.pos];
            if now == '\n' {
                is_end_line = true;
                self.pos += 1;
                break;
            }
            if now == '\"' && !escape {
                is_match = true;
                self.pos += 1;
                break;
            }
            if now == '\\' && !escape {
                escape = true;
                self.pos += 1;
                continue;
            }
            if escape {
                // TODO check escape is allow
                escape = false;
            }
            self.pos += 1;
        }
        if is_match {
            self.add_token_cur_end(TokenKind::String, start);
        } else {
            if is_end_line {
                self.add_token_cur_end(TokenKind::Unknown(String::from("end line before string end")), start);
            } else {
                self.add_token_cur_end(TokenKind::Unknown(String::from("eof string end")), start);
            }
        }
        true
    }

    // for easy to implement, just allow int and float numbers, hex, octal, binary, exp number not supported for now
    fn match_number(&mut self) -> bool {
        let start = self.pos;
        let mut is_float = self.source[self.pos] == '.';
        let mut multi_dot = false;
        let mut unknown_char = false;
        self.pos += 1;
        while !self.is_eof() {
            let now = self.source[self.pos];
            match now {
                '0'..='9' => {
                    self.pos += 1;
                }
                _ => {
                    if now == '.' {
                        if !is_float {
                            is_float = true;
                            self.pos += 1;
                            continue;
                        } else {
                            multi_dot = true;
                        }
                    } else if self.match_bound() {
                        break;
                    } else {
                        unknown_char = true;
                    }
                }
            }
        }
        if multi_dot {
            self.add_token_cur_end(TokenKind::Unknown(String::from("multi dot in expr")), start);
        } else if unknown_char {
            self.add_token_cur_end(TokenKind::Unknown(String::from("unknown character in expr")), start);
        } else {
            if is_float {
                self.add_token_cur_end(TokenKind::Number(Number::Float), start);
            } else {
                self.add_token_cur_end(TokenKind::Number(Number::Int), start);
            }
        }
        true
    }


    fn match_identifier(&mut self) -> bool {
        let start = self.pos;
        let mut unknown_char = false;
        self.pos += 1;
        while !self.is_eof() {
            let now = self.source[self.pos];
            match now {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.pos += 1;
                    continue;
                }
                _ => {
                    if self.match_bound() {
                        break;
                    }
                    unknown_char = true;
                }
            }
        }
        if unknown_char {
            self.add_token_cur_end(TokenKind::Unknown(String::from("unknown character in ident")), start);
        } else {
            if !self.match_keyword(start) {
                self.add_token_cur_end(TokenKind::Ident, start);
            }
        }
        true
    }
    fn match_keyword(&mut self, start: usize) -> bool {
        let ident = &self.raw[start..self.pos];

        macro_rules! match_keyword {
            ($($name:ident),*) => {
                    $(
                    if &stringify!($name).to_lowercase()[..] == ident {
                        self.add_token_cur_end(TokenKind::Keyword(Keyword::$name), start);
                        return true;
                    }
                    )*
            }
        }
        match_keyword! {
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
                Struct
        }
        false
    }
}

pub fn parse(src: &str) -> TokenStream {
    let mut parser = Parser {
        source: src.chars().collect(),
        raw: src,
        pos: 0,
        tokens: Vec::new(),
    };
    parser.parse_tokens();
    TokenStream{tokens: parser.tokens, now_at: 0 }
}