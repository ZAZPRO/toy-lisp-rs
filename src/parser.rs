use crate::object::*;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::lexer::*;

#[derive(Debug)]
pub struct ParserError {
    err: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParserError {}

pub fn parse(lexer_tokens: &mut Vec<LexerToken>) -> Result<Object, ParserError> {
    parse_list(lexer_tokens)
}

fn parse_list(lexer_tokens: &mut Vec<LexerToken>) -> Result<Object, ParserError> {
    let token = lexer_tokens.pop();
    if token != Some(LexerToken::ParenthesesOpen) {
        return Err(ParserError {
            err: format!("Expected Parentheses Open '(', but found {:?}\n", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !lexer_tokens.is_empty() {
        let token = lexer_tokens.pop();
        if token == None {
            return Err(ParserError {
                err: "Insufficient tokens\n".to_string(),
            });
        }

        let t = token.unwrap();

        match t {
            LexerToken::Integer(n) => list.push(Object::Integer(n)),
            LexerToken::Float(n) => list.push(Object::Float(n)),
            LexerToken::Bool(b) => list.push(Object::Bool(b)),
            LexerToken::OpAdd => list.push(Object::Operator(Op::Add)),
            LexerToken::OpSub => list.push(Object::Operator(Op::Sub)),
            LexerToken::OpMul => list.push(Object::Operator(Op::Mul)),
            LexerToken::OpDiv => list.push(Object::Operator(Op::Div)),
            LexerToken::OpEqual => list.push(Object::Operator(Op::Eq)),
            LexerToken::OpNotEqual => list.push(Object::Operator(Op::NotEq)),
            LexerToken::OpGreater => list.push(Object::Operator(Op::Greater)),
            LexerToken::OpSmaller => list.push(Object::Operator(Op::Smaller)),
            LexerToken::ParenthesesOpen => {
                lexer_tokens.push(LexerToken::ParenthesesOpen);
                let sub_list = parse_list(lexer_tokens)?;
                list.push(sub_list);
            }
            LexerToken::ParenthesesClose => {
                return Ok(Object::List(list));
            }
            LexerToken::Keyword(s) => list.push(Object::Keyword(s.to_string())),
            LexerToken::Name(s) => list.push(Object::Name(s.to_string())),
            LexerToken::If => list.push(Object::Condition),
            LexerToken::Error => (),
        }
    }

    Ok(Object::List(list))
}
