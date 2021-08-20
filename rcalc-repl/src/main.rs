use derive_more::Display;
use rcalc_engine::parser::ParseError;
use rcalc_engine::Executor;
use rcalc_engine::ExecutorError;
use rcalc_engine::Interpreter;
use rcalc_engine::{AstBuilder, VMExecutor};
use rustyline::{error::ReadlineError, Editor};
use thiserror::Error;

#[derive(Error, Debug, Display)]
enum ReplError {
    ParseError(#[from] ParseError),
    ExecutorError(#[from] ExecutorError),
    ReadlineError(#[from] ReadlineError),
}

fn main() -> Result<(), ReplError> {
    println!("Hello, world!");
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline(">> ");
        match line {
            Ok(l) => {
                let node = AstBuilder::build_ast(l.as_str());
                match node {
                    Ok(n) => {
                        let executor = VMExecutor {};
                        let result = executor.execute_ast(&n);
                        match result {
                            Ok(i) => println!("{}", i),
                            Err(e) => println!("{}", e),
                        }
                    }
                    Err(e) => println!("{}", ReplError::ParseError(e)),
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => break,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }

    Ok(())
}
