use crate::ast::stmt::*;
use crate::error::*;
use crate::interpreter::Interpreter;
use crate::resolver::*;
use crate::syntax::parser::Parser;
use crate::syntax::scanner::*;
use crate::syntax::token::*;

use std::{
    fs::File,
    io::{self, prelude::*},
};

pub struct Lang<'a> {
    interpreter: Interpreter,
    scanner_two: Option<Scanner<'a>>,
}

impl<'a> Lang<'a> {
    pub fn new(script: Option<&'a str>) -> Lang<'a> {
        let scanner_two = if let Some(give_script) = script {
            Some(Scanner::new(give_script))
        } else {
            None
        };
        Lang {
            interpreter: Interpreter::new(),
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
            let source = scanner.source.clone();
            let tokens: Vec<Token> = scanner.scan_tokens()?;
            let mut parser = Parser::new(source, tokens);
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
        let mut parser = Parser::new(script, tokens);
        let statements = parser.parse()?;
        resolver.resolve(&statements)?;
        resolver.interpreter.interpret(statements)?;
        Ok(())
    }

    pub fn print_ast(&mut self) -> Result<(), LangError> {
        unimplemented!()
    }

    pub fn error2(token: &Token, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(
            token.span.begin.line.into(),
            &format!("at '{}'", token.span.content.input),
            message,
        )
    }

    pub fn error_ir(line: u32, lexeme: &str, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(line.into(), &format!("at '{}'", lexeme), message)
    }

    pub fn error_s(token: &str, message: &str) -> LangError {
        /*         if token.token_type == syntax::TokenType::Eof {
            return Lang::report(token.line, "at end ", message);
        } */
        Lang::report(0, &format!("at '{}'", token), message)
    }

    pub fn parser_error(line: &str, token: &Token, error_mesg: &str) -> LangError {
        LangErrorType::new_parser_error(format!(
            "\n\t|\n{}\t|{}\n\t|\n{}",
            token.span.begin.line, line, error_mesg
        ))
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
