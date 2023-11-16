#![allow(unused)]

use super::tok::*;


pub fn match_single_char<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
	($tt:expr) => {
	    Some(Token::new($tt, word))
	};
    };

    let tok = match word {
	";" => tok!(Semicolon),
	"=" => tok!(Equal),
	"(" => tok!(LeftParen),
	")" => tok!(RightParen),
	"{" => tok!(LeftBrace),
	"}" => tok!(RightBrace),
	"," => tok!(Comma),
        "." => tok!(Dot),
        "-" => tok!(Minus),
        "+" => tok!(Plus),
        "/" => tok!(Slash),
        "*" => tok!(Star),
        "%" => tok!(Percent),
        "!" => tok!(Bang),
        ":" => tok!(Colon),
        "<" => tok!(Less),
        ">" => tok!(Greater),

	_ => None
    };

    tok
}

pub fn match_keyword<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
	($tt:expr) => {
	    Some(Token::new($tt, word))
	};
    };
    
    let tok = match word {
        "fn" => tok!(Fn),
        "let" => tok!(Let),
        "if" => tok!(If),
        "else" => tok!(Else),
        "while" => tok!(While),
        "elif" => tok!(Elif),
        "return" => tok!(Return),
        "for" => tok!(For),
        "in" => tok!(In),
        "break" => tok!(Break),
        "continue" => tok!(Continue),
        "true" => tok!(True),
        "false" => tok!(False),

        _ => None
    };

    tok
}

pub fn match_two_char<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
        ($tt:expr) => {
            Some(Token::new($tt, word))
        };
    };

    let tok = match word {
        "==" => tok!(EqualEqual),
        "!=" => tok!(BangEqual),
        "<=" => tok!(LessEqual),
        ">=" => tok!(GreaterEqual),

        _ => None
    };

    tok
}

pub fn match_string_literal<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
        ($tt:expr) => {
            Some(Token::new($tt, word))
        };
    };

    
    let mut chars = word.chars();

    if word.starts_with("\"") {
	chars.next();
	while let Some(char) = chars.next() {
	    if char == '\"' {
		return tok!(String);
	    }
	}
    }
    if word.starts_with("\'") {
	while let Some(char) = chars.next() {
	    if char == '\'' {
		return tok!(String);
	    }
	}
    }

    None
}

pub fn match_int_literal<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
        ($tt:expr) => {
            Some(Token::new($tt, word))
        };
    };

    let mut chars = word.chars();
    let mut tok = None;
    while let Some(char) = chars.next() {
	if char.is_digit(10) {
	    tok = tok!(Number);
	} else {
	    return None;
	}
    }

    tok
}

pub fn match_identifier<'a>(word: &'a str) -> Option<Token<'a>> {
    macro_rules! tok {
        ($tt:expr) => {
            Some(Token::new($tt, word))
        };
    };

    let mut chars = word.chars().peekable();
    let mut tok: Option<Token<'a>> = None;
    if chars.peek().unwrap_or(&'❌').is_ascii_alphabetic() {
	while let Some(char) = chars.next() {	    
	    if char.is_ascii() && match_single_char(char.to_string().as_str()).is_none() {
		tok = tok!(Identifier);
	    } else {
		return None;
	    }
	}
    } else {
	return None;
    }

    tok
}
