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
#[parser("(Ast::Expression -> expr) ? Semi")]
pub struct Statement{
    // TODO
    expr: Expression
}


#[derive(Debug)]
#[parser()]
pub enum Expression{
    NumberExpression(NumberExpression),
    PlusExpression(PlusExpression),
    MinusExpression(MinusExpression),

    BadExpression
}

#[derive(Default, Debug)]
#[parser("Number::_->number")]
pub struct NumberExpression{
    number: Token
}


#[derive(Default, Debug)]
#[parser("Ast::Expression -> !left Plus Ast::Expression -> !right")]
pub struct PlusExpression {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Default, Debug)]
#[parser("Ast::Expression -> !left Minus Ast::Expression -> !right")]
pub struct MinusExpression {
    left: Box<Expression>,
    right: Box<Expression>,
}


pub fn parse_fn(stream: &mut TokenStream) -> Result<(), String> {
    let f = FnDeclaration::parse(stream)?;
    println!("{:?}", f);
    Ok(())
}

pub fn parse_demo(stream: &mut TokenStream) -> Result<(), String> {
    let f = Statement::parse(stream).unwrap();
    println!("{:?}", f);
    Ok(())
}