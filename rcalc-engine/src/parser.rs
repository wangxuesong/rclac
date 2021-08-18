use crate::ast::{Node, Operator};

use derive_more::Display;
use nom::{
    branch::alt, character::complete::char, character::complete::digit1, combinator::map,
    sequence::tuple, IResult,
};
use nom_locate::LocatedSpan;
use std::str::FromStr;
use thiserror::Error;

pub type Span<'a> = LocatedSpan<&'a [u8]>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Position {
    pub line: u32,
    pub offset: usize,
}

impl Position {
    pub fn new(line: u32, offset: usize) -> Position {
        Position { line, offset }
    }

    pub fn new_empty() -> Position {
        Position { line: 0, offset: 0 }
    }

    fn from_span(span: Span) -> Position {
        Position {
            line: span.location_line(),
            offset: span.get_column(),
        }
    }
}

impl<'a> From<Span<'a>> for Position {
    fn from(s: Span<'a>) -> Self {
        Position::from_span(s)
    }
}

#[derive(Error, Debug, Display)]
pub enum ParseError {
    // #[error("Parse error")]
    ParseError(String),
}

pub fn parse_str(s: &str) -> Result<Node, ParseError> {
    let result =
        parse(Span::new(s.as_bytes())).map_err(|e| ParseError::ParseError(e.to_string()))?;
    let node = result.1;
    Ok(node)
}

pub fn parse(i: Span) -> IResult<Span, Node> {
    map(
        tuple((digit1, operator, digit1)),
        |tup: (Span, Operator, Span)| {
            let left = i64::from_str(std::str::from_utf8(tup.0.fragment()).unwrap()).unwrap();
            let right = i64::from_str(std::str::from_utf8(tup.2.fragment()).unwrap()).unwrap();
            Node::BinaryOperator {
                operator: tup.1,
                left: Box::new(Node::Number(left)),
                right: Box::new(Node::Number(right)),
            }
        },
    )(i)
}

fn operator(i: Span) -> IResult<Span, Operator> {
    alt((
        map(char('+'), |_| Operator::Plus),
        map(char('-'), |_| Operator::Minus),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_add_sub_expression() {
        let expr_str = ["1+2", "2-1"];

        let expected_node = [
            Node::BinaryOperator {
                operator: Operator::Plus,
                left: Box::new(Node::Number(1)),
                right: Box::new(Node::Number(2)),
            },
            Node::BinaryOperator {
                operator: Operator::Minus,
                left: Box::new(Node::Number(2)),
                right: Box::new(Node::Number(1)),
            },
        ];

        for (i, e) in expr_str.iter().enumerate() {
            let res = parse(Span::new(e.as_bytes()));
            assert!(res.is_ok());
            assert_eq!(res.unwrap().1, expected_node[i]);
        }
    }
}
