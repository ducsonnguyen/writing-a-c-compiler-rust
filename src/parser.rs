use crate::ast::{Exp, Function, Program, Statement, UnaryOperator};
use crate::lexer::Token;
use std::fmt;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn take_token(&mut self) -> Result<&'a Token, ParseError> {
        if self.pos < self.tokens.len() {
            let tok = &self.tokens[self.pos];
            self.pos += 1;
            Ok(tok)
        } else {
            Err(ParseError("unexpected end of input".to_string()))
        }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let actual = self.take_token()?;
        if actual == expected {
            Ok(())
        } else {
            Err(ParseError(format!("expected {expected:?}, got {actual:?}")))
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let func = self.parse_function()?;
        if self.pos < self.tokens.len() {
            return Err(ParseError(format!(
                "unexpected token {:?} after function definition",
                self.tokens[self.pos]
            )));
        }
        Ok(Program::Program(func))
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect(&Token::Int)?;
        let name = match self.take_token()? {
            Token::Identifier(name) => name.clone(),
            tok => return Err(ParseError(format!("expected identifier, got {tok:?}"))),
        };
        self.expect(&Token::OpenParen)?;
        self.expect(&Token::Void)?;
        self.expect(&Token::CloseParen)?;
        self.expect(&Token::OpenBrace)?;
        let body = self.parse_statement()?;
        self.expect(&Token::CloseBrace)?;
        Ok(Function { name, body })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        self.expect(&Token::Return)?;
        let exp = self.parse_exp()?;
        self.expect(&Token::Semicolon)?;
        Ok(Statement::Return(exp))
    }

    fn parse_exp(&mut self) -> Result<Exp, ParseError> {
        let next = self
            .peek()
            .ok_or_else(|| ParseError("unexpected end of input in expression".to_string()))?;
        match next {
            Token::Constant(n) => {
                let n = *n;
                self.pos += 1;
                Ok(Exp::Constant(n))
            }
            Token::Tilde | Token::Hyphen => {
                let op = self.parse_unop()?;
                let inner = self.parse_exp()?;
                Ok(Exp::Unary(op, Box::new(inner)))
            }
            Token::OpenParen => {
                self.pos += 1;
                let inner = self.parse_exp()?;
                self.expect(&Token::CloseParen)?;
                Ok(inner)
            }
            tok => Err(ParseError(format!("malformed expression: unexpected token {tok:?}"))),
        }
    }

    fn parse_unop(&mut self) -> Result<UnaryOperator, ParseError> {
        match self.take_token()? {
            Token::Tilde => Ok(UnaryOperator::Complement),
            Token::Hyphen => Ok(UnaryOperator::Negate),
            tok => Err(ParseError(format!("expected unary operator, got {tok:?}"))),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Program, ParseError> {
    Parser::new(tokens).parse_program()
}
