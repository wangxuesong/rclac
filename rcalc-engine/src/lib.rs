use derive_more::Display;
use thiserror::Error;

mod ast;
mod interpreter;
pub mod parser;

pub use ast::*;
pub use interpreter::Interpreter;

#[derive(Error, Debug, Display)]
pub enum ExecutorError {
    InterpreterError(#[from] interpreter::InterpreterError),
}

pub trait Executor {
    fn execute_ast(&self, root: &ast::Node) -> Result<i64, ExecutorError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
