use crate::lexer::Lexer;
use crate::token::{PositionedToken, Token};

pub struct Parser {
    tokens: Vec<PositionedToken>,
    pos: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let tokens = lexer.tokenize();
        Self { tokens, pos: 0 }
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
                expected, self.current_token(),
            ));
        }
        self.advance();
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<(), String> {
        self.parse_block()
    }

    fn parse_block(&mut self) -> Result<(), String> {
        self.expect(&Token::Symbol("{".to_string()))?;
        self.parse_stmts()?;
        self.expect(&Token::Symbol("}".to_string()))?;
        Ok(())
    }

    fn parse_stmts(&mut self) -> Result<(), String> {
        // stmts -> stmt stmts | ε
        // 尝试解析 stmt，如果失败或下一个是 } 则为空产生式
        while self.current_token().token != Token::Symbol("}".to_string()) {
            self.parse_stmt()?;
        }
        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<(), String> {
        // 根据当前 token 来判断进入哪个产生式
        match &self.current_token().token {
            // id = expr ;
            Token::Identifier(_) => {
                let _id = self.current_token();
                self.advance();
                self.expect(&Token::Symbol("=".to_string()))?;
                self.parse_expr()?;
                self.expect(&Token::Symbol(";".to_string()))?;
            }

            // if (bool) stmt restIf
            Token::Keyword(k) if k == "if" => {
                self.advance();
                self.expect(&Token::Symbol("(".to_string()))?;
                self.parse_bool()?;
                self.expect(&Token::Symbol(")".to_string()))?;
                self.parse_stmt()?;
                self.parse_rest_if()?;
            }

            // while (bool) stmt
            Token::Keyword(k) if k == "while" => {
                self.advance();
                self.expect(&Token::Symbol("(".to_string()))?;
                self.parse_bool()?;
                self.expect(&Token::Symbol(")".to_string()))?;
                self.parse_stmt()?;
            }

            // do stmt while (bool)
            Token::Keyword(k) if k == "do" => {
                self.advance();
                self.parse_stmt()?;
                self.expect(&Token::Keyword("while".to_string()))?;
                self.expect(&Token::Symbol("(".to_string()))?;
                self.parse_bool()?;
                self.expect(&Token::Symbol(")".to_string()))?;
            }

            // break
            Token::Keyword(k) if k == "break" => {
                self.advance();
            }

            // block
            Token::Symbol(s) if s == "{" => {
                self.parse_block()?;
            }

            _ => return Err(format!("Unexpected token: {:?}", self.current_token())),
        }

        Ok(())
    }

    fn parse_rest_if(&mut self) -> Result<(), String> {
        // restIf -> else stmt | ε
        if let Token::Keyword(k) = &self.current_token().token {
            if k == "else" {
                self.advance();
                self.parse_stmt()?;
            }
        }
        Ok(())
    }

    fn parse_bool(&mut self) -> Result<(), String> {
        // bool -> expr bop
        self.parse_expr()?;
        self.parse_bop()?;
        Ok(())
    }

    fn parse_bop(&mut self) -> Result<(), String> {
        // bop -> < expr | <= expr | > expr | >= expr | ε
        match &self.current_token().token {
            Token::Symbol(sym) if ["<", "<=", ">", ">="].contains(&sym.as_str()) => {
                self.advance();
                self.parse_expr()?;
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_expr(&mut self) -> Result<(), String> {
        // expr -> term expr'
        self.parse_term()?;
        self.parse_expr_prime()?;
        Ok(())
    }

    fn parse_expr_prime(&mut self) -> Result<(), String> {
        // expr' -> + term expr' | - term expr' | ε
        while let Token::Symbol(sym) = &self.current_token().token {
            if sym == "+" || sym == "-" {
                self.advance(); // + or -
                self.parse_term()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), String> {
        // term -> factor term'
        self.parse_factor()?;
        self.parse_term_prime()?;
        Ok(())
    }

    fn parse_term_prime(&mut self) -> Result<(), String> {
        // term' -> * factor term' | / factor term' | ε
        while let Token::Symbol(sym) = &self.current_token().token {
            if sym == "*" || sym == "/" {
                self.advance(); // * or /
                self.parse_factor()?;
            } else {
                break;
            }
        }
        Ok(())
    }

    fn parse_factor(&mut self) -> Result<(), String> {
        // factor -> ( expr ) | id | num
        match &self.current_token().token {
            Token::Symbol(s) if s == "(" => {
                self.advance();
                self.parse_expr()?;
                self.expect(&Token::Symbol(")".to_string()))?;
            }
            Token::Identifier(_) | Token::Number { .. } => {
                self.advance();
            }
            _ => return Err(format!("Unexpected token in factor: {:?}", self.current_token())),
        }
        Ok(())
    }
}
