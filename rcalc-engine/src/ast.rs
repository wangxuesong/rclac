#[derive(Debug, PartialEq)]
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
