use crate::ast::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("Interpreter error")]
    InvalidUnaryOperator { operator: Operator },
}

pub struct Interpreter {}

impl Interpreter {
    pub fn execute_ast(root: &Node) -> Result<i64, InterpreterError> {
        match root {
            Node::Number(n) => Ok(*n),
            Node::BinaryOperator {
                operator,
                left,
                right,
            } => {
                let left = Interpreter::execute_ast(left)?;
                let right = Interpreter::execute_ast(right)?;
                Ok(match operator {
                    Operator::Plus => left + right,
                    Operator::Minus => left - right,
                    Operator::Multiply => left * right,
                    Operator::Divide => left / right,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let expr_node = [
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
            Node::BinaryOperator {
                operator: Operator::Multiply,
                left: Box::new(Node::Number(3)),
                right: Box::new(Node::Number(2)),
            },
            Node::BinaryOperator {
                operator: Operator::Divide,
                left: Box::new(Node::Number(4)),
                right: Box::new(Node::Number(1)),
            },
        ];
        let expected = [3i64, 1, 6, 4];

        for (i, e) in expr_node.iter().enumerate() {
            let res = Interpreter::execute_ast(e);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), expected[i]);
        }
    }
}
