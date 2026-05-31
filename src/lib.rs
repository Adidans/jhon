use chumsky::{input::ValueInput, prelude::*};
use logos::Logos;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    Error,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token(":")]
    Colon,
    #[token(",")]
    Comma,

    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("null")]
    Null,

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r#""([^"\\\x00-\x1F]|\\(["\\bnfrt/]|u[a-fA-F0-9]{4}))*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    String(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => write!(f, "<error>"),

            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),

            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),

            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),

            Token::Bool(b) => write!(f, "{}", b),
            Token::Null => write!(f, "null"),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Null,
    Bool(bool),
    String(String),
    Number(f64),
    Array(Vec<Expr>),
    Object(HashMap<String, Expr>),
}

pub fn parser<'tok, I>() -> impl Parser<'tok, I, Expr, extra::Err<Rich<'tok, Token>>>
where
    I: ValueInput<'tok, Token = Token, Span = SimpleSpan>,
{
    recursive(|expr| {
        let atom = select! {
            Token::Null => Expr::Null,
            Token::Bool(b) => Expr::Bool(b),
            Token::String(s) => Expr::String(s),
            Token::Number(num) => Expr::Number(num),
        };

        let array = expr
            .clone()
            .separated_by(just(Token::Comma))
            .collect()
            .delimited_by(just(Token::LBracket), just(Token::RBracket))
            .map(Expr::Array);

        let pair = select! {
            Token::String(s) => s
        }
        .then_ignore(just(Token::Colon))
        .then(expr.clone());

        let object = pair
            .separated_by(just(Token::Comma))
            .collect()
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(Expr::Object);

        atom.or(array).or(object)
    })
}
