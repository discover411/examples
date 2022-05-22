
use std::fmt::{ Display, Formatter, Result as FmtResult };

// ------------------------------------------------------------------------------------------------
// Token type
// ------------------------------------------------------------------------------------------------

/*
Token grammar:

LParen:  '('
RParen:  ')'
Id:      IdStart IdCont*
IdStart: <alphabetic> | '_'
IdCont:  IdStart | Digit
IntLit:  Digit+
Token:   LParen | RParen | Id | IntLit

Whitespace: ' ' | '\t' | '\n'
Program:    (Whitespace? Token)* Whitespace? Eof
*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
	// Eof represents the end of the input. There will be one Eof token at the
	// end of the lexer output.
	Eof,
	LParen,
	RParen,
	Id(String),
	IntLit(i64),
}

#[derive(Debug, Clone)]
pub struct Token {
	pub loc:  usize, // the codepoint index in the source code.
	pub kind: TokenKind,
}

impl Token {
	// Self is a "magical type" that can be used in impl blocks, and refers to the type
	// that the impl is attached to. Here it means "Token". It doesn't save much typing
	// here, but if it were a big generic type with arguments, it would!
	pub fn new(loc: usize, kind: TokenKind) -> Self {
		return Token { loc, kind };
	}
}

// ------------------------------------------------------------------------------------------------
// LexError type
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum LexError {
	InvalidChar(char),
	IntOutOfRange,
}

// Display is a "trait," something like a Java interface. Display is Rust's equivalent to Java's
// toString() method. This way, a LexError can be printed out.
impl Display for LexError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			LexError::InvalidChar(c) => write!(f, "invalid character '{}'", c.escape_debug()),
			LexError::IntOutOfRange  => write!(f, "integer out of range"),
		}
	}
}

// std::error::Error is another standard library trait, and says that LexError can be used to
// report errors, basically. Empty {} because there are no required methods to implement.
impl std::error::Error for LexError {}

// ------------------------------------------------------------------------------------------------
// The lexer algorithm
// ------------------------------------------------------------------------------------------------

// These two functions implement the IdStart and IdCont rules in the lexer grammar.

// IdStart: <alphabetic> | '_'
fn is_ident_start(c: char) -> bool {
	return c.is_alphabetic() || c == '_';
}

// IdCont:  IdStart | Digit
fn is_ident_cont (c: char) -> bool {
	return is_ident_start(c) || c.is_ascii_digit();
}

// Result<R, E> is how functions return errors in Rust. R is the return type if it succeeds;
// E is the error type if it fails. So this function returns a Vec<Token> on success and
// a LexError on failure.
pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
	// this turns source from a UTF-8 string into a vector of codepoints, so we can
	// index it by codepoint index in O(1) time, at the cost of taking up more space.
	// (Rust lets you re-declare variables of the same name. It's really a new variable.)

	// The syntax ".collect::<Vec<_>>()" is called the "turbofish" and is required here to
	// say *what* to collect the chars into. I'm saying "I want a Vec of whatever." otherwise
	// the compiler won't have enough information and won't know what data structure you want!
	let source = source.chars().collect::<Vec<_>>();

	// our position in the source. since it's mut, we can move this forward through the source.
	let mut pos = 0;

	// the vec of tokens to be returned. since it's mut, we can push things into it.
	let mut ret = vec![];

	// let's go! this loop implements the Program rule.
	while pos < source.len() {
		match source[pos] {
			// Whitespace: ' ' | '\t' | '\n'
			' ' | '\t' | '\n' => {
				// ignore it and move on.
				pos += 1;
			}

			// LParen
			'(' => {
				ret.push(Token::new(pos, TokenKind::LParen));
				pos += 1;
			}

			// RParen
			')' => {
				ret.push(Token::new(pos, TokenKind::RParen));
				pos += 1;
			}

			// This case is the "default", but it assigns the thing we matched on (source[pos])
			// into this new variable "c", so "c" in the code below refers to the current character.
			c => {
				// Id: IdStart IdCont*
				// this check implements the "IdStart" part of the grammar rule.
				if is_ident_start(c) {
					let start = pos;

					// a mut String variable is like a StringBuilder/StringBuffer in Java.
					let mut s = String::new();

					// this loop implements the "IdCont*" part of the grammar rule.
					while pos < source.len() && is_ident_cont(source[pos]) {
						// we can push characters into the string, similar to how Vecs work.
						s.push(source[pos]);
						pos += 1;
					}

					ret.push(Token::new(start, TokenKind::Id(s)));

				// IntLit: Digit+
				} else if c.is_ascii_digit() {
					let start = pos;

					let mut num = String::new();

					// The c.is_ascii_digit() above and this loop together implement the
					// "Digit+" part of the grammar rule.
					while pos < source.len() && source[pos].is_ascii_digit() {
						num.push(source[pos]);
						pos += 1;
					}

					// this rule makes things like "123abc" invalid. this is actually
					// a lookahead because we're just checking the next character without
					// making it part of this token.
					if pos < source.len() && source[pos].is_alphabetic() {
						// If we encounter an error condition, we return an Err() value
						// containing the error. This is sort of like throwing an exception
						// in Java.
						return Err(LexError::InvalidChar(source[pos]));
					}

					// some rules, like "can't exceed the capacity of a 64-bit integer," can't
					// be encoded in the grammar rules and have to be checked manually.
					// this function returns Ok() if it succeeded and Err() if it failed.
					match i64::from_str_radix(&num, 10) {
						// this match arm declares the "value" variable, and it contains
						// the actual integer that was returned by i64::from_str_radix.
						Ok(value) => {
							ret.push(Token::new(start, TokenKind::IntLit(value)));
						}

						Err(..) => {
							return Err(LexError::IntOutOfRange)
						}
					}
				} else {
					// any other character is no good.
					return Err(LexError::InvalidChar(c));
				}
			}
		}
	}

	// If we get down here, we successfully lexed the entire string,
	// so let's put an Eof token at the end of the output.
	ret.push(Token::new(pos, TokenKind::Eof));

	// we indicate success by returning an Ok() value containing the thing we want to return.
	return Ok(ret);
}