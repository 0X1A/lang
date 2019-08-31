use crate::ast::stmt::*;
use crate::error::*;
use crate::interpreter::Interpreter;
use crate::parser::*;
use crate::resolver::*;
use crate::scanner::*;
use crate::syntax::parser;
use crate::syntax::scanner::*;
use crate::syntax::token::*;
use crate::token::*;

use std::{
    fs::File,
    io::{self, prelude::*},
};

pub struct Lang<'a> {
    interpreter: Interpreter,
    scanner: Option<Scanner<'a>>,
    scanner_two: Option<ScannerTwo<'a>>,
}

impl<'a> Lang<'a> {
    pub fn new(script: Option<&'a str>) -> Lang<'a> {
        let scanner_two = if let Some(give_script) = script {
            Some(ScannerTwo::new(give_script))
        } else {
            None
        };
        Lang {
            interpreter: Interpreter::new(),
            scanner: None,
            scanner_two,
        }
    }
}

impl<'a> Lang<'a> {
    pub fn setup_logging(_: u64) -> Result<(), LangError> {
        env_logger::builder().default_format_timestamp(false).init();
        Ok(())
    }

    pub fn build_statements(&mut self) -> Result<Vec<Stmt>, LangError> {
        if let Some(ref mut scanner) = self.scanner_two {
            let tokens: Vec<TokenTwo> = scanner.scan_tokens()?;
            for token in tokens.iter() {
                debug!("{:?}", token);
            }
            //let mut parser = parser::Parser::new(tokens);
            //let statements = parser.parse()?;
        }
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
                resolver.interpreter.interpret(s)?;
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
        resolver.interpreter.interpret(statements)?;
        Ok(())
    }

    pub fn print_ast(&mut self) -> Result<(), LangError> {
        unimplemented!()
    }

    pub fn error(token: &Token, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(token.line, &format!("at '{}'", token.lexeme), message)
    }

    pub fn error2(token: &TokenTwo, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(
            token.span.begin.line.into(),
            &format!("at '{}'", token.lexeme),
            message,
        )
    }

    pub fn error_s(token: &String, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(0, &format!("at '{}'", token), message)
    }

    pub fn report(line: u64, ware: &str, message: &str) -> LangError {
        LangErrorType::new_runtime_error(RuntimeErrorType::GenericError {
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
