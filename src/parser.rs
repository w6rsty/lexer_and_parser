use crate::lexer::Lexer;
use crate::token::{PositionedToken, Token};

pub struct Parser {
    tokens: Vec<PositionedToken>,
    pos: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokenize();
        Self {
            tokens,
            pos: 0,
        }
    }

    fn current_token(&self) -> &PositionedToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.current_token().token != *expected {
            return Err(format!(
                "Expected token {:?}, found {:?}",
                expected,
                self.current_token()
            ));
        }
        self.advance();
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<(), String> {
        self.parse_block()?;
        Ok(())
    }

    fn parse_block(&mut self) -> Result<(), String> {
        self.expect(&Token::Symbol("{".to_string()))?; // {
        self.parse_statements()?;
        self.expect(&Token::Symbol("}".to_string()))?; // }
        Ok(())
    }

    fn parse_statements(&mut self) -> Result<(), String> {
        while self.current_token().token != Token::Symbol("}".to_string()) {
            self.parse_statement()?;
        }
        Ok(())
    }

    fn parse_statement(&mut self) -> Result<(), String> {
        match self.current_token().token {
            Token::Identifier(_) => {
                self.advance();
                self.expect(&Token::Symbol("=".to_string()))?; // =
                self.parse_expression()?;
                self.expect(&Token::Symbol(";".to_string()))?; // ;
            }
            Token::Keyword(ref kw) if kw == "if" => {
                self.advance(); // if
                self.expect(&Token::Symbol("(".to_string()))?; // (
                self.parse_boolean_expression()?;
                self.expect(&Token::Symbol(")".to_string()))?; // )
                self.parse_statement()?;
                if self.current_token().token == Token::Keyword("else".to_string()) {
                    self.advance(); // else
                    self.parse_statement()?;
                }
            }
            Token::Keyword(ref kw) if kw == "while" => {
                self.advance(); // while
                self.expect(&Token::Symbol("(".to_string()))?; // (
                self.parse_boolean_expression()?;
                self.expect(&Token::Symbol(")".to_string()))?; // )
                self.parse_statement()?;
            }
            Token::Keyword(ref kw) if kw == "do" => {
                self.advance(); // do
                self.parse_statement()?;
                self.expect(&Token::Keyword("while".to_string()))?; // while
                self.expect(&Token::Symbol("(".to_string()))?; // (
                self.parse_boolean_expression()?;
                self.expect(&Token::Symbol(")".to_string()))?; // )
                self.expect(&Token::Symbol(";".to_string()))?; // ;
            }
            Token::Keyword(ref kw) if kw == "break" => {
                self.advance(); // break
                self.expect(&Token::Symbol(";".to_string()))?; // ;
            }
            Token::Symbol(ref sym) if sym == "{" => {
                self.parse_block()?;
            }
            _ => return Err(format!("Unexpected token: {:?}", self.current_token())),
        }
        Ok(())
    }

    fn parse_boolean_expression(&mut self) -> Result<(), String> {
        self.parse_expression()?;
        match self.current_token().token {
            Token::Symbol(ref sym)
                if sym == "<" || sym == "<=" || sym == ">" || sym == ">=" =>
            {
                self.advance();
                self.parse_expression()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_expression(&mut self) -> Result<(), String> {
        self.parse_term()?;
        while matches!(
            self.current_token().token,
            Token::Symbol(ref sym) if sym == "+" || sym == "-"
        ) {
            self.advance(); // + -
            self.parse_term()?;
        }
        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), String> {
        self.parse_factor()?;
        while matches!(
            self.current_token().token,
            Token::Symbol(ref sym) if sym == "*" || sym == "/"
        ) {
            self.advance(); // * /
            self.parse_factor()?;
        }
        Ok(())
    }

    fn parse_factor(&mut self) -> Result<(), String> {
        match self.current_token().token {
            Token::Symbol(ref sym) if sym == "(" => {
                self.advance(); // (
                self.parse_expression()?;
                self.expect(&Token::Symbol(")".to_string()))?; // )
            }
            Token::Identifier(_) | Token::Number { .. } => {
                self.advance(); // identifier or number
            }
            _ => return Err(format!("Unexpected token in factor: {:?}", self.current_token())),
        }
        Ok(())
    }
}
