use ast_parser::parser;
use crate::token::types::*;


pub trait Declaration {}

pub trait Expression {}

pub trait Statement {}

trait Parser {
    type Output;
    fn parse(stream: &mut TokenStream) -> Result<Self::Output, String>;
}
#[derive(Default, Debug)]
#[parser("Token::Ident -> name Token::Colon Token::Ident -> type_")]
pub struct Argument {
    name: String,
    type_: String,
}

// impl Parser for Argument {
//     type Output = Argument;
//     fn parse(stream: &mut TokenStream) -> Result<Argument, String> {
//         let mut res = Argument::default();
//         let mut state = 0;
//         while !stream.is_end(){
//             let top = &stream.tokens[stream.now_at];
//             match state {
//                 0 => {
//                     if let TokenKind::Ident = top.kind{
//                         state = 1;
//                         stream.now_at += 1;
//                         res.name = top.lex.to_string();
//                         continue
//                     }
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 1 => {
//                     if let TokenKind::Colon = top.kind{
//                         state = 2;
//                         stream.now_at += 1;
//                         continue
//                     }
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 2 => {
//                     if let TokenKind::Ident = top.kind{
//                         state = 3;
//                         stream.now_at += 1;
//                         res.type_ = top.lex.to_string();
//                         continue
//                     }
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 3 => {
//                     break
//                 }
//                 _ => {
//                     return Err(String::from("unknown state"));
//                 }
//             }
//         }
//         return Ok(res)
//     }
// }


// impl Parser for Argument {
//     type Output = Argument;
//     fn parse(stream: &mut TokenStream) -> Result<Argument, String> {
//         let mut res = Argument::default();
//         let mut state = 0;
//         while !stream.is_end() {
//             match state {
//                 0 => {
//                     let top = &stream.tokens[stream.now_at];
//                     if let TokenKind::Ident = top.kind {
//                         stream.now_at += 1;
//                         state = 1;
//                         res.name = top.lex.to_string();
//                         continue
//                     }
//                     return Err(format!("Expected Ident, got {:?}", top.kind))
//                 }
//                 1 => {
//                     let top = &stream.tokens[stream.now_at];
//                     if let TokenKind::Colon = top.kind {
//                         stream.now_at += 1;
//                         state = 2;
//                         continue
//                     }
//                     return Err(format!("Expected Colon, got {:?}", top.kind))
//                 }
//                 2 => {
//                     let top = &stream.tokens[stream.now_at];
//                     if let TokenKind::Ident = top.kind {
//                         stream.now_at += 1;
//                         res.type_ = top.lex.to_string();
//                         break
//                     }
//                     return Err(format!("Expected Ident, got {:?}", top.kind))
//                 }
//                 _ => {
//                     return Err(String::from("unknown state"));
//                 }
//             }
//         }
//         now_state
//     }
// }

#[derive(Default, Debug)]
#[parser("Token::Keyword::Fn Token::Ident -> fn_name Token::LParen Argument -> .args ( (Token::Comma Argument -> .args) *) ? Token::RParen Block -> body")]
pub struct FnDeclaration {
    fn_name: String,
    args: Vec<Argument>,
    body: Block,
}

// impl Parser for FnDeclaration {
//     type Output = FnDeclaration;
//     fn parse(stream: &mut TokenStream) -> Result<FnDeclaration, String> {
//         let now = stream.now_at;
//         let mut res = FnDeclaration::default();
//         let mut state = 0;
//         while !stream.is_end(){
//             match state {
//                 0 => {
//                     {
//                         let top = &stream.tokens[stream.now_at];
//                         if let TokenKind::Keyword(Keyword::Fn) = top.kind {
//                             state = 1;
//                             stream.now_at += 1;
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 1 => {
//                     {
//                         let top = &stream.tokens[stream.now_at];
//                         if let TokenKind::Ident = top.kind {
//                             state = 2;
//                             stream.now_at += 1;
//                             res.fn_name = top.lex.to_string();
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 2 => {
//                     {
//                         let top = &stream.tokens[stream.now_at];
//                         if let TokenKind::LParen = top.kind {
//                             state = 3;
//                             stream.now_at += 1;
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 3 => {
//                     {
//                         if let Ok(item) = Argument::parse(stream) {
//                             state = 4;
//                             res.args.push(item);
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 4 => {
//                     {
//                         let top = &stream.tokens[stream.now_at];
//                         if let TokenKind::RParen = top.kind {
//                             state = 5;
//                             stream.now_at += 1;
//                             continue
//                         }
//                     }
//                     {
//                         let top = &stream.tokens[stream.now_at];
//                         if let TokenKind::Comma = top.kind {
//                             state = 7;
//                             stream.now_at += 1;
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 5 => {
//                     {
//                         if let Ok(item) = Block::parse(stream) {
//                             state = 6;
//                             res.body = item;
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 6 => {
//                     break
//                 }
//                 7 => {
//                     {
//                         if let Ok(item) = Argument::parse(stream) {
//                             state = 4;
//                             res.args.push(item);
//                             continue
//                         }
//                     }
//                     let top = &stream.tokens[stream.now_at];
//                     stream.now_at = now;
//                     return Err(format!("Unexpected Token, got {:?}", top.kind))
//                 }
//                 _ => {
//                     stream.now_at = now;
//                     return Err(String::from("unknown state"));
//                 }
//             }
//         }
//         return Ok(res)
//     }
// }

#[derive(Default, Debug)]
pub struct Block {
    // TODO
}

impl Parser for Block {
    type Output = Block;

    fn parse(stream: &mut TokenStream) -> Result<Self::Output, String> {
        return Ok(Block::default())
    }
}



// impl Parser for FnDeclaration {
//     type Output = FnDeclaration;
//
//     fn parse(stream: &mut TokenStream) -> Result<FnDeclaration, String> {
//         let mut state = 0;
//         let mut res = FnDeclaration::default();
//         loop {
//             if state == 0 {
//                 let top = &stream.tokens[stream.now_at];
//                 if let TokenKind::Keyword(Keyword::Fn) = top.kind {
//                     stream.now_at += 1;
//                     state = 1;
//                 }
//
//             }
//         }
//         Ok(FnDeclaration {
//             fn_name: "".to_string(),
//             args: vec![],
//             body: Block {},
//         })
//     }
// }

pub fn parse_fn(stream: &mut TokenStream) -> Result<(), String> {
    let f = FnDeclaration::parse(stream)?;
    println!("{:?}", f);
    Ok(())
}

pub fn parse_arg(stream: &mut TokenStream) -> Result<(), String> {
    let r = Argument::parse(stream)?;
    println!("{} {}", r.name, r.type_);
    Ok(())
}