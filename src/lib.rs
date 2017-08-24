#[macro_use]
extern crate synom;

pub mod parser;
pub mod ast;

use std::ops::Deref;
use synom::IResult;
use synom::space::*;

use self::ast::*;
use self::parser::*;

/// Parse a VMF string, returning the list of parsed blocks
pub fn parse<'a, I, K>(input: &'a I) -> Result<Vec<Block<K>>, &'static str> where I: 'a + Deref<Target=str>, K: From<&'a str> {
    match file(input) {
        IResult::Done(rem, ast) => if skip_whitespace(rem) != "" {
            Err("failed to parse the entire input")
        } else {
            Ok(ast)
        },
        IResult::Error => Err("parse error"),
    }
}
