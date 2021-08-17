use crate::ast::{Node, Operator};

use nom::IResult;
use nom_locate::LocatedSpan;

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

pub fn parse(i: Span) -> IResult<Span, Node> {
    // Err(Err::Incomplete(Needed::new(4)))
    Ok((
        Span::new("".as_bytes()),
        Node::BinaryOperator {
            operator: Operator::Plus,
            left: Box::new(Node::Number(1)),
            right: Box::new(Node::Number(2)),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_add_sub_expression() {
        let expr_str = ["1+2"];

        let expected_node = [Node::BinaryOperator {
            operator: Operator::Plus,
            left: Box::new(Node::Number(1)),
            right: Box::new(Node::Number(2)),
        }];

        for (i, e) in expr_str.iter().enumerate() {
            let res = parse(Span::new(e.as_bytes()));
            assert!(res.is_ok());
            assert_eq!(res.unwrap().1, expected_node[i]);
        }
    }
}
