extern crate fern;
extern crate log;

use accept::*;
use ast::stmt::*;
use ast_printer::*;
use error::*;
use interpreter::Interpreter;
use parser::*;
use resolver::*;
use scanner::*;
use std::{
    fs::File,
    io::{self, prelude::*},
};
use token::*;

pub struct Lang<'a> {
    interpreter: Interpreter,
    scanner: Option<Scanner<'a>>,
}

impl<'a> Lang<'a> {
    pub fn new(script: Option<&'a str>) -> Lang<'a> {
        let scanner = if let Some(give_script) = script {
            Some(Scanner::new(give_script))
        } else {
            None
        };
        Lang {
            interpreter: Interpreter::new(),
            scanner,
        }
    }
}

impl<'a> Lang<'a> {
    pub fn setup_logging(_: u64) -> Result<(), LangError> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                if record.level() > log::LevelFilter::Info {
                    out.finish(format_args!("{:-^80}\n{}", record.level(), message))
                } else {
                    out.finish(format_args!("{}: {}", record.level(), message))
                }
            })
            .chain(io::stdout())
            .apply()?;
        Ok(())
    }

    pub fn build_statements(&mut self) -> Result<Vec<Stmt>, LangError> {
        if let Some(ref mut scanner) = self.scanner {
            let tokens: Vec<Token> = scanner.scan_tokens()?;
            debug!("{:?}", tokens);
            let mut parser = Parser::new(tokens);
            let statements = parser.parse()?;
            Ok(statements)
        } else {
            Ok(vec![])
        }
    }

    pub fn run(&mut self) -> Result<(), LangError> {
        let statements = self.build_statements();
        let mut resolver = Resolver::new(&mut self.interpreter);
        match statements {
            Ok(s) => {
                resolver.resolve(&s)?;
                resolver.interpreter.interpret_two(s)?;
            }
            Err(e) => {
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn build_and_run_statements(&mut self, script: &str) -> Result<(), LangError> {
        let mut scanner = Scanner::new(script);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut resolver = Resolver::new(&mut self.interpreter);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;
        resolver.resolve(&statements)?;
        resolver.interpreter.interpret_two(statements)?;
        Ok(())
    }

    pub fn print_ast(&mut self) -> Result<(), LangError> {
        let statements = self.build_statements();
        let mut printer = ASTPrinter::default();
        match statements {
            Ok(s) => {
                for statement in s {
                    statement.accept(&mut printer);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        Ok(())
    }

    pub fn error(token: &Token, message: &str) -> LangError {
        if token.token_type == TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        }
        Lang::report(token.line, &format!("at '{}'", token.lexeme), message)
    }

    pub fn report(line: u64, ware: &str, message: &str) -> LangError {
        LangError::new_runtime_error(RuntimeErrorType::GenericError {
            reason: format!("[line {}] Error {}: {}", line, ware, message),
        })
    }

    pub fn run_prompt(&mut self) -> Result<(), LangError> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut input)?;
            self.build_and_run_statements(&input)?;
        }
    }

    pub fn read_file(file_path: String) -> Result<String, LangError> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
