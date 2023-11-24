#![allow(unused)]
use logos::Logos;
use logos::Lexer;
use core::iter::Peekable;

pub use TokenType::*;

#[derive(Debug, Logos, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenType {
    // SINGLE CHARACTER TOKENS
    #[token(";")]
    Semicolon,	// ;
    #[token("=")]
    Equal,	// =
    #[token("(")]
    LeftParen,	// (
    #[token(")")]
    RightParen, // )
    #[token("{")]
    LeftBrace,	// {
    #[token("}")]
    RightBrace, // }
    #[token(",")]
    Comma,	// ,
    #[token(".")]
    Dot,	// .
    #[token("-")]
    Minus,	// -
    #[token("+")]
    Plus,	// +
    #[token("/")]
    Slash,	// /
    #[token("*")]
    Star,	// *
    #[token("%")]
    Percent,	// %
    #[token("!")]
    Bang,	// !
    #[token(":")]
    Colon,	// :
    #[token("<")]
    Less,	// <
    #[token(">")]
    Greater,	// >
    #[token("|")]
    Pipe,       // |

    // KEYWORDS
    #[token("fnaf")]
    Fnaf,	// fnaf
    #[token("let")]
    Let,	// let
    #[token("if")]
    If,		// if
    #[token("else")]
    Else,	// else
    #[token("while")]
    While,	// while
    #[token("elif")]
    Elif,	// elif
    #[token("return")]
    Return,	// return
    #[token("for")]
    For,	// for
    #[token("in")]
    In,		// in
    #[token("break")]
    Break,	// break
    #[token("continue")]
    Continue,	// continue

    // TWO CHARACTER TOKENS
    #[token("==")]
    EqualEqual,		// ==
    #[token("!=")]
    BangEqual,		// !=
    #[token("<=")]
    LessEqual,		// <=
    #[token(">=")]
    GreaterEqual,	// >=
    
    // LITERALS
    #[regex(r#"("[^"]*")|('[^']*')"#)]
    String,	// A string literal.
    #[regex("[0-9]+")]
    Number,	// An integer.
    #[regex(r#"[^[0-9]^"^-^[ \t\n\f]^\.^=^(^)^{^}.^,^;]+[^"^-^=^\..^[ \t\n\f]^(^)^{^}^,^;]*"#)]
    Identifier, // An identifier.
    #[token("true")]
    True,	// true
    #[token("false")]
    False,	// false
    #[token("none")]
    Null,	// none
}

pub fn lex_str(this: &str) -> Vec<(TokenType, &str)> {
    println!("\"{}\"", this);
    let mut buf = Vec::new();
    let mut lexer = TokenType::lexer(this);
    while let Some(Ok(token)) = lexer.next() {
	buf.push((token, lexer.slice()));
    }

    buf
}
