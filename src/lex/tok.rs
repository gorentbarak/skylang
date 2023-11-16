#![allow(unused)]
pub use TokenType::*;
use super::parse::*;

#[derive(Debug)]
pub struct Token<'a> {
    tt: TokenType,
    word: &'a str,
}

#[derive(Debug)]
pub enum TokenType {
    EOF,

    // SINGLE CHARACTER TOKENS
    Semicolon,	// ;
    Equal,	// =
    LeftParen,	// (
    RightParen, // )
    LeftBrace,	// {
    RightBrace, // }
    Comma,	// ,
    Dot,	// .
    Minus,	// -
    Plus,	// +
    Slash,	// /
    Star,	// *
    Percent,	// %
    Bang,	// !
    Colon,	// :
    Less,	// <
    Greater,	// >

    // KEYWORDS
    Fn,		// fn
    Let,	// let
    If,		// if
    Else,	// else
    While,	// while
    Elif,	// elif
    Return,	// return
    For,	// for
    In,		// in
    Break,	// break
    Continue,	// continue

    // TWO CHARACTER TOKENS
    EqualEqual,		// ==
    BangEqual,		// !=
    LessEqual,		// <=
    GreaterEqual,	// >=
    
    // LITERALS
    String,	// A string literal.
    Number,	// An integer.
    Identifier, // An identifier.
    True,	// true
    False,	// false
    Null,	// None

    // ERROR
    Error, // A syntax error.
}

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    current: usize,
    after: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Lexer {
            source: "",
            tokens: Vec::new(),
            current: 0,
            after: ""
        }
    }
}

impl<'a> std::iter::Iterator for Lexer<'a> {
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
	unimplemented!("Iterating over lexer is not implemented.");
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(value: &'a str) -> Self {
        Lexer {
            source: value,
            tokens: Vec::new(),
            current: 0,
            after: value 
        }
    }
}

impl<'a> From<&'a std::string::String> for Lexer<'a> {
    fn from(value: &'a std::string::String) -> Self {
	Lexer {
            source: value.as_str(),
            tokens: Vec::new(),
            current: 0,
            after: value.as_str()
	} 
    }
}

impl<'a> Token<'a> {
    pub fn new(tt: TokenType, word: &'a str) -> Self {
	Token {
	    tt,
	    word
	}
    }

    pub fn empty() -> Self {
	Token {
	    tt: EOF,
	    word: ""
	}
    }
}
