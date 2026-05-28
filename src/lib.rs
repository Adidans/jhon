use chumsky::{input::ValueInput, prelude::*};
use logos::Logos;
use std::fmt;

#[derive(Debug, Clone, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    Error,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Error => write!(f, "<error>"),
        }
    }
}

pub fn parser<'tok, I>() -> impl Parser<'tok, I, (), extra::Err<Rich<'tok, Token>>>
where
    I: ValueInput<'tok, Token = Token, Span = SimpleSpan>,
{
    end()
}
