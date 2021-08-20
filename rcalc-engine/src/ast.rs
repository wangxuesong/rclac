use crate::parser::{parse_str, ParseError};

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i64),
    BinaryOperator {
        operator: Operator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

pub struct AstBuilder {}

impl AstBuilder {
    pub fn build_ast(s: &str) -> Result<Node, ParseError> {
        let node = parse_str(s)?;
        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_build() {
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
            let res = AstBuilder::build_ast(e);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), expected_node[i]);
        }
    }
}
