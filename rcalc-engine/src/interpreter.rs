use crate::ast::*;
use crate::{Executor, ExecutorError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("attempt to divide by zero")]
    DivideByZero,
    #[error("Interpreter error")]
    InvalidOperator { operator: Operator },
}

pub struct Interpreter {}

impl Executor for Interpreter {
    fn execute_ast(&self, root: &Node) -> Result<i64, ExecutorError> {
        match root {
            Node::Number(n) => Ok(*n),
            Node::BinaryOperator {
                operator,
                left,
                right,
            } => {
                let left = Interpreter::execute_ast(self, left)?;
                let right = Interpreter::execute_ast(self, right)?;
                match operator {
                    Operator::Plus => Ok(left + right),
                    Operator::Minus => Ok(left - right),
                    Operator::Multiply => Ok(left * right),
                    Operator::Divide => {
                        if right == 0 {
                            return Err(ExecutorError::InterpreterError(
                                InterpreterError::DivideByZero,
                            ));
                        }
                        Ok(left / right)
                    }
                }
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
            let res = Interpreter {}.execute_ast(e);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), expected[i]);
        }
    }
}
