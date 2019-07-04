extern crate nom;

use std::collections::HashMap;

use crate::error::*;
use crate::syntax::span::*;
use crate::syntax::token::*;

use nom::{character::complete::one_of, character::complete::char, IResult};

trait SubStr {
    fn substr(&self, beg: usize, end: usize) -> String;
}

impl SubStr for String {
    /// Creates a substring from a string using indices `from` and `to`
    fn substr(&self, from: usize, to: usize) -> String {
        self.chars().skip(from).take(to - from).collect()
    }
}

impl SubStr for str {
    /// Creates a substring from a string using indices `from` and `to`
    fn substr(&self, from: usize, to: usize) -> String {
        self.chars().skip(from).take(to - from).collect()
    }
}

pub struct ScannerTwo<'a> {
    source: &'a str,
    pub tokens: Vec<TokenTwo>,
    start: usize,
    current: usize,
    line: u64,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> ScannerTwo<'a> {
    pub fn new(script_content: &'a str) -> ScannerTwo<'a> {
        let mut keywords = HashMap::new();
        keywords.insert("break", TokenType::Break);
        keywords.insert("enum", TokenType::Enum);
        keywords.insert("and", TokenType::And);
        keywords.insert("struct", TokenType::Struct);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fn", TokenType::Fn);
        keywords.insert("if", TokenType::If);
        keywords.insert("unit", TokenType::Unit);
        keywords.insert("or", TokenType::Or);
        keywords.insert("impl", TokenType::Impl);
        keywords.insert("trait", TokenType::Trait);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("true", TokenType::True);
        keywords.insert("let", TokenType::Let);
        keywords.insert("while", TokenType::While);
        keywords.insert("self", TokenType::SelfIdent);
        ScannerTwo {
            source: script_content,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    /// Returns true when the current position has reached the end of the source's string
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn clear(&mut self) {
        self.source = "";
        self.tokens = Vec::with_capacity(0);
    }

    /// Advances the `current` position, and returns the character at the previous `current` position
    pub fn pop(&mut self) -> Result<char, LangError> {
        self.current += 1;
        if let Some(next) = self.source.chars().nth(self.current - 1) {
            return Ok(next);
        } else {
            Err(LangError::new_parser_error(
                "Reached end of source at advance()".to_string(),
            ))
        }
    }

    /// Adds a token to the scanner with token type `token` and lexeme `value`
    pub fn add_token_value(&mut self, token: TokenType, value: &str) -> Result<(), LangError> {
        self.tokens.push(TokenTwo::new(token, &value)?);
        Ok(())
    }

    /// Checks the type annotation string and returns its corresponding `TypeAnnotation`
    pub fn match_type_annotation(&mut self, value: &str) -> Result<TypeAnnotation, LangError> {
        match value {
            "i32" => Ok(TypeAnnotation::I32),
            "i64" => Ok(TypeAnnotation::I64),
            "f32" => Ok(TypeAnnotation::F32),
            "f64" => Ok(TypeAnnotation::F64),
            "bool" => Ok(TypeAnnotation::Bool),
            "String" => Ok(TypeAnnotation::String),
            "fn" => Ok(TypeAnnotation::Fn),
            "()" => Ok(TypeAnnotation::Unit),
            "Array" => {
                self.start = self.current;
                let type_annotation = self.template_type()?;
                Ok(TypeAnnotation::Array(Box::new(type_annotation)))
            }
            _ => Ok(TypeAnnotation::User(value.to_string())),
        }
    }

    pub fn at_index(&self, index: usize) -> Result<TokenType, LangError> {
        self.tokens.get(index).map_or(
            Err(LangError::new_parser_error(
                "Tried to peek token when empty".to_string(),
            )),
            |token| Ok(token.token_type.clone()),
        )
    }

    /// Returns the last token within `tokens`, errors when the token vector is empty
    pub fn prev_token(&self) -> Result<TokenType, LangError> {
        self.tokens.last().map_or(
            Err(LangError::new_parser_error(
                "Tried to peek token when empty".to_string(),
            )),
            |token| Ok(token.token_type.clone()),
        )
    }

    /// Creates and adds TokenType `token` to `tokens`. The Token is created
    /// using `Token::from`
    pub fn add_token(&mut self, token: TokenType) -> Result<(), LangError> {
        self.tokens.push(TokenTwo::new(
            token,
            &self.source.substr(self.start, self.current),
        )?);
        Ok(())
    }

    /// Checks if the character at `current` matches `expected`
    pub fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(next) = self.source.chars().nth(self.current) {
            if next != expected {
                return false;
            }
        }
        self.current += 1;
        true
    }

    /// Returns the character in the `current` position. Does not move the position forward
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        if let Some(next) = self.source.chars().nth(self.current) {
            return next;
        }
        '\0'
    }

    /// Returns the character in the position ahead of `current`. Does not move the position forward
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        if let Some(next) = self.source.chars().nth(self.current + 1) {
            return next;
        }
        '\0'
    }

    /// Pops the character at `current` and matches its representation in Token
    pub fn scan_token(&mut self) -> Result<(), LangError> {
        let c: char = self.pop()?;
        match c {
            '(' => {
                let prev_token = self.prev_token()?;
                let right_paren = self.peek() == ')';
                if right_paren {
                    if prev_token == TokenType::Colon || prev_token == TokenType::ReturnType {
                        self.current += 1;
                        self.add_token(TokenType::Type(TypeAnnotation::Unit))?;
                    } else if prev_token == TokenType::Equal {
                        self.current += 1;
                        self.add_token(TokenType::Unit)?;
                    } else if prev_token == TokenType::Identifier {
                        self.add_token(TokenType::LeftParen)?;
                    }
                } else {
                    self.add_token(TokenType::LeftParen)?;
                }
            }
            ')' => self.add_token(TokenType::RightParen)?,
            '{' => self.add_token(TokenType::LeftBrace)?,
            '}' => self.add_token(TokenType::RightBrace)?,
            '[' => self.add_token(TokenType::LeftBracket)?,
            ']' => self.add_token(TokenType::RightBracket)?,
            ',' => self.add_token(TokenType::Comma)?,
            '.' => self.add_token(TokenType::Dot)?,
            '-' => {
                if self.match_next('>') {
                    self.add_token(TokenType::ReturnType)?;
                } else {
                    self.add_token(TokenType::Minus)?
                }
            }
            '+' => self.add_token(TokenType::Plus)?,
            ';' => self.add_token(TokenType::SemiColon)?,
            '*' => self.add_token(TokenType::Star)?,
            '!' => {
                let bang_eq = self.match_next('=');
                self.add_token(if bang_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })?;
            }
            '=' => {
                let eq_eq = self.match_next('=');
                self.add_token(if eq_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })?;
            }
            '<' => {
                let bang_eq = self.match_next('=');
                self.add_token(if bang_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })?;
            }
            '>' => {
                let bang_eq = self.match_next('=');
                self.add_token(if bang_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })?;
            }
            '/' => {
                if self.match_next('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.pop()?;
                    }
                } else {
                    self.add_token(TokenType::Slash)?;
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()?;
            }
            ':' => {
                if self.prev_token()? == TokenType::Colon {
                    self.tokens.pop();
                    self.tokens.push(TokenTwo::new(
                        TokenType::PathSeparator,
                        &self.source.substr(self.start - 1, self.current),
                    )?);
                } else {
                    self.add_token(TokenType::Colon)?;
                }
            }
            _ => {
                if c.is_digit(10) {
                    self.number()?;
                } else if c.is_alphanumeric() || c == '_' {
                    self.identifier()?;
                } else {
                    return Err(LangError::new_parser_error(format!(
                        "Unexpected character '{}'",
                        c
                    )));
                }
            }
        };
        Ok(())
    }

    /// Parses a string delimited by the character '"'
    fn string(&mut self) -> Result<(), LangError> {
        loop {
            if self.peek() == '"' || self.is_at_end() {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.pop()?;
        }
        if self.is_at_end() {
            return Err(LangError::new_parser_error(
                "Unterminated string".to_string(),
            ));
        }
        self.pop()?;
        let value = self.source.substr(self.start + 1, self.current - 1);
        self.add_token_value(TokenType::String, &value)?;
        Ok(())
    }

    /// Parses a template type parameter between angle brackets
    fn template_type(&mut self) -> Result<TypeAnnotation, LangError> {
        let less = self.pop()?;
        if less != '<' {
            return Err(LangError::new_parser_error(
                "Expected '<' after template type".to_string(),
            ));
        }
        // self.add_token(TokenType::Less)?;
        loop {
            if !self.peek().is_alphanumeric() && self.peek() != '_' {
                break;
            }
            self.pop()?;
        }
        let greater = self.pop()?;
        if greater != '>' {
            return Err(LangError::new_parser_error(
                "Expected '>' after template type".to_string(),
            ));
        }
        //self.add_token(TokenType::Greater)?;
        let value: String = self.source.substr(self.start + 1, self.current - 1);
        Ok(self.match_type_annotation(&value)?)
    }

    /// Parses an integer or float
    fn number(&mut self) -> Result<(), LangError> {
        let mut is_float = false;
        loop {
            if !self.peek().is_digit(10) {
                break;
            }
            self.pop()?;
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.pop()?;
            loop {
                if !self.peek().is_digit(10) {
                    break;
                }
                self.pop()?;
            }
            is_float = true;
        }
        let value = self.source.substr(self.start, self.current);
        if is_float {
            self.add_token_value(TokenType::Float, &value)?;
        } else {
            self.add_token_value(TokenType::Integer, &value)?;
        }
        Ok(())
    }

    /// Gets the keyword token corresponding to lexeme `lexeme`
    fn get_keyword(&self, lexeme: &str) -> Option<TokenType> {
        self.keywords
            .get(lexeme)
            .and_then(|token| Some(token.clone()))
    }

    /// TODO: name is misleading
    /// Parses strings of type `([a-zA-Z0-9]|'_')*`, adds them as
    /// 1. a keyword if they match a keyword
    /// 2. a type annotation if the previously parsed token was a colon
    /// 3. or a plain identifier if none of those criterion are met
    fn identifier(&mut self) -> Result<(), LangError> {
        loop {
            if !self.peek().is_alphanumeric() && self.peek() != '_' {
                break;
            }
            self.pop()?;
        }
        let value: String = self.source.substr(self.start, self.current);
        if let Some(token) = self.get_keyword(value.as_str()) {
            self.add_token(token)?;
        } else if self.prev_token()? == TokenType::Colon
            || self.prev_token()? == TokenType::ReturnType
        {
            let type_annotation = self.match_type_annotation(&value)?;
            self.add_token(TokenType::Type(type_annotation))?;
        } else {
            self.add_token(TokenType::Identifier)?;
        }
        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<TokenTwo>, LangError> {
        loop {
            if self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens
            .push(TokenTwo::new(TokenType::Eof, &"EoF".to_string())?);
        let tokens = self.tokens.clone();
        self.clear();
        Ok(tokens)
    }
}

#[cfg(test)]
mod scanner_tests {
    use super::*;
    #[test]
    fn test_subtr() {
        let string = "ChangeToSubstring".to_string();
        assert_eq!(string.substr(0, 6), "Change".to_string());
        assert_eq!(string.substr(6, 8), "To".to_string());
    }

    #[test]
    fn test_scanner_new() {
        let script_content = "let n = 100; let string = 100; print n;".to_string();
        let mut scanner = ScannerTwo::new(&script_content);
        assert_eq!(scanner.tokens.len(), 0);
        let tokens = scanner.scan_tokens().unwrap();
        assert_eq!(TokenType::Let, tokens[0].token_type);
        assert_eq!(TokenType::Eof, tokens[tokens.len() - 1].token_type);
    }
}
