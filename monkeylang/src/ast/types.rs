use ast_parser::parser;
use crate::token::types::*;


// pub trait Declaration {}
//
// pub trait Expression {}
//
// pub trait Statement {}

trait Parser {
    type Output;
    fn parse(stream: &mut TokenStream) -> Result<Self::Output, String>;
}

#[derive(Default, Debug)]
#[parser("Ident->name Colon Ident->typ")]
pub struct Argument {
    name: Token,
    typ: Token,
}

#[derive(Default, Debug)]
#[parser("Keyword::Fn Ident->fn_name LParen (Ast::Argument->.args (Comma Ast::Argument->.args)*)? RParen Ast::Block->body")]
pub struct FnDeclaration {
    fn_name: Token,
    args: Vec<Argument>,
    body: Block,
}


#[derive(Default, Debug)]
#[parser("LBrace (Ast::Statement -> .statements)* RBrace")]
pub struct Block {
    // TODO
    statements: Vec<Statement>
}

#[derive(Default, Debug)]
// #[parser("")]
pub struct Statement{
    // TODO
    expr: Expression
}


#[derive(Debug)]
// #[parser("Ast::PlusExpression | Ast::MinusExpression")]
pub enum Expression{
    PlusExpression(PlusExpression),
    MinusExpression(MinusExpression),

    BadExpression(String),
}


impl Default for Expression{
    fn default() -> Self {
        Expression::BadExpression("default".to_string())
    }
}

#[derive(Default, Debug)]
#[parser("Number::_ Plus Number::_")]
pub struct PlusExpression {
    left: Token,
    right: Token,
}

#[derive(Default, Debug)]
#[parser("Number::_ Plus Number::_")]
pub struct MinusExpression {
    left: Token,
    right: Token,
}

impl Parser for Statement {
    type Output = Statement;

    fn parse(_stream: &mut TokenStream) -> Result<Self::Output, String> {
        return Ok(Statement::default());
    }
}


pub fn parse_fn(stream: &mut TokenStream) -> Result<(), String> {
    let f = FnDeclaration::parse(stream)?;
    println!("{:?}", f);
    Ok(())
}
