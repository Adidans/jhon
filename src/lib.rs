use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
}
