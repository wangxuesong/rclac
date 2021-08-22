use crate::ast::{Node, Operator};

use derive_more::Display;
use nom::{
    branch::alt, character::complete::char, character::complete::digit1,
    character::complete::multispace0, combinator::map, multi::many0, sequence::preceded,
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
    if result.0.len() > 0 {
        return Err(ParseError::ParseError(format!(
            "syntax error in row {}, column {}: {}",
            result.0.location_line(),
            result.0.get_utf8_column(),
            std::str::from_utf8(result.0.fragment()).unwrap()
        )));
    }
    let node = result.1;
    Ok(node)
}

pub fn parse(i: Span) -> IResult<Span, Node> {
    alt((
        map(
            tuple((term, add_sub_operator, term)),
            |(left, op, right)| Node::BinaryOperator {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            },
        ),
        term,
    ))(i)
}

fn term(i: Span) -> IResult<Span, Node> {
    map(
        tuple((number, many0(tuple((mul_div_operator, number))))),
        |(n, v)| {
            v.into_iter().fold(n, |l, (o, r)| Node::BinaryOperator {
                operator: o,
                left: Box::new(l),
                right: Box::new(r),
            })
        },
    )(i)
}

fn number(i: Span) -> IResult<Span, Node> {
    preceded(
        multispace0,
        map(digit1, |d: Span| {
            Node::Number(i64::from_str(std::str::from_utf8(d.fragment()).unwrap()).unwrap())
        }),
    )(i)
}

fn add_sub_operator(i: Span) -> IResult<Span, Operator> {
    preceded(
        multispace0,
        alt((
            map(char('+'), |_| Operator::Plus),
            map(char('-'), |_| Operator::Minus),
        )),
    )(i)
}

fn mul_div_operator(i: Span) -> IResult<Span, Operator> {
    preceded(
        multispace0,
        alt((
            map(char('*'), |_| Operator::Multiply),
            map(char('/'), |_| Operator::Divide),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_add_sub_expression() {
        let expr_str = [" 1 + 2", "2-1", "3", "2*1", "3/1", " 1 + 2 * \t\n3"];

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
            Node::Number(3),
            Node::BinaryOperator {
                operator: Operator::Multiply,
                left: Box::new(Node::Number(2)),
                right: Box::new(Node::Number(1)),
            },
            Node::BinaryOperator {
                operator: Operator::Divide,
                left: Box::new(Node::Number(3)),
                right: Box::new(Node::Number(1)),
            },
            Node::BinaryOperator {
                operator: Operator::Plus,
                left: Box::new(Node::Number(1)),
                right: Box::new(Node::BinaryOperator {
                    operator: Operator::Multiply,
                    left: Box::new(Node::Number(2)),
                    right: Box::new(Node::Number(3)),
                }),
            },
        ];

        for (i, e) in expr_str.iter().enumerate() {
            let res = parse(Span::new(e.as_bytes()));
            assert!(res.is_ok());
            assert_eq!(res.as_ref().unwrap().0.len(), 0);
            assert_eq!(res.unwrap().1, expected_node[i]);
        }
    }
}
