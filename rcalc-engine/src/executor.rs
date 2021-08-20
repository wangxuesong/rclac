use crate::{Executor, ExecutorError, Node, Operator};

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Number(i64),
    Operator(Operator),
}

pub struct VMExecutor {}

impl VMExecutor {
    fn compile(&self, node: &Node) -> Result<Vec<Element>, ExecutorError> {
        let mut elements = Vec::new();
        match node {
            Node::Number(n) => elements.push(Element::Number(*n)),
            Node::BinaryOperator {
                operator,
                left,
                right,
            } => {
                let mut l = self.compile(left)?.clone();
                let temp = l.as_mut();
                elements.append(temp);
                let mut r = self.compile(right)?.clone();
                let temp = r.as_mut();
                elements.append(temp);
                elements.push(Element::Operator(operator.clone()))
            }
        }
        Ok(elements)
    }

    fn eval(&self, proto: &Vec<Element>) -> Result<i64, ExecutorError> {
        let mut eval_stack = Vec::new();
        for e in proto {
            match e {
                Element::Number(n) => eval_stack.push(*n),
                Element::Operator(op) => match op {
                    Operator::Plus => {
                        let right = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        let left = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        eval_stack.push(left + right);
                    }
                    Operator::Minus => {
                        let right = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        let left = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        eval_stack.push(left - right);
                    }
                    Operator::Multiply => {
                        let right = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        let left = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        eval_stack.push(left * right);
                    }
                    Operator::Divide => {
                        let right = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        if right == 0 {
                            return Err(ExecutorError::VMExecutorError);
                        }
                        let left = eval_stack.pop().ok_or(ExecutorError::VMExecutorError)?;
                        eval_stack.push(left / right);
                    }
                },
            }
        }
        if eval_stack.len() == 1 {
            return Ok(eval_stack[0]);
        }

        Err(ExecutorError::VMExecutorError)
    }
}

impl Executor for VMExecutor {
    fn execute_ast(&self, root: &Node) -> Result<i64, ExecutorError> {
        let proto = self.compile(root)?;
        self.eval(&proto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Operator;

    #[test]
    fn test_compile() {
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
        let expected = [
            vec![
                Element::Number(1),
                Element::Number(2),
                Element::Operator(Operator::Plus),
            ],
            vec![
                Element::Number(2),
                Element::Number(1),
                Element::Operator(Operator::Minus),
            ],
            vec![
                Element::Number(3),
                Element::Number(2),
                Element::Operator(Operator::Multiply),
            ],
            vec![
                Element::Number(4),
                Element::Number(1),
                Element::Operator(Operator::Divide),
            ],
        ];

        for (i, e) in expr_node.iter().enumerate() {
            let res = VMExecutor {}.compile(e);
            assert!(res.is_ok());
            assert_eq!(res.unwrap()[..], expected[i][..]);
        }
    }

    #[test]
    fn test_eval() {
        let expr = [
            vec![
                Element::Number(1),
                Element::Number(2),
                Element::Operator(Operator::Plus),
            ],
            vec![
                Element::Number(2),
                Element::Number(1),
                Element::Operator(Operator::Minus),
            ],
            vec![
                Element::Number(3),
                Element::Number(2),
                Element::Operator(Operator::Multiply),
            ],
            vec![
                Element::Number(4),
                Element::Number(1),
                Element::Operator(Operator::Divide),
            ],
        ];
        let expected = [3, 1, 6, 4];

        for (i, e) in expr.iter().enumerate() {
            let res = VMExecutor {}.eval(e);
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), expected[i]);
        }
    }
}
