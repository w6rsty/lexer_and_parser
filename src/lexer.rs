use crate::token::*;
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    row: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            row: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peek();

        if let Some(c) = current {
            if c == '\n' {
                self.row += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.position += 1;
        }

        current
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_identifier_or_keyword(&mut self) -> Token {
        let mut ident = String::new();

        if let Some(c) = self.peek() {
            if c.is_alphabetic() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                return Token::Error(format!("Invalid start of identifier: {}", c));
            }
        }

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if KEYWORDS.contains(&ident.as_str()) {
            Token::Keyword(ident)
        } else {
            Token::Identifier(ident)
        }
    }

    fn parse_number(&mut self) -> Token {
        let mut number = String::new();
        let mut kind = NumberKind::Integer;
        let mut base = NumberBase::Decimal;
        let suffix;

        // Check for hexadecimal, octal, or binary literals
        if self.peek() == Some('0') {
            number.push(self.advance().unwrap()); // Consume '0'
            if let Some(c) = self.peek() {
                match c {
                    'x' | 'X' => {
                        base = NumberBase::Hexadecimal;
                        number.push(self.advance().unwrap()); // Consume 'x' or 'X'
                        while let Some(c) = self.peek() {
                            if c.is_digit(16) {
                                number.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    'b' | 'B' => {
                        base = NumberBase::Binary;
                        number.push(self.advance().unwrap()); // Consume 'b' or 'B'
                        while let Some(c) = self.peek() {
                            if c == '0' || c == '1' {
                                number.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    '0'..='7' => {
                        base = NumberBase::Octal;
                        while let Some(c) = self.peek() {
                            if c >= '0' && c <= '7' {
                                number.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    '.' | 'e' | 'E' => {
                        kind = NumberKind::FloatingPoint;
                        self.parse_fraction_and_exponent(&mut number);
                    }
                    _ => {
                        // It's just '0'
                    }
                }
            }
        } else {
            // Parse decimal integer or floating point
            while let Some(c) = self.peek() {
                if c.is_digit(10) {
                    number.push(c);
                    self.advance();
                } else if c == '.' || c == 'e' || c == 'E' {
                    kind = NumberKind::FloatingPoint;
                    self.parse_fraction_and_exponent(&mut number);
                    break;
                } else {
                    break;
                }
            }
        }

        // Check for suffixes
        if kind == NumberKind::Integer {
            suffix = self.parse_integer_suffix();
        } else {
            suffix = self.parse_floating_point_suffix();
        }

        Token::Number {
            literal: number,
            kind,
            base,
            suffix,
        }
    }

    fn parse_fraction_and_exponent(&mut self, number: &mut String) {
        // Parse fractional part
        if self.peek() == Some('.') {
            number.push(self.advance().unwrap()); // Consume '.'
            while let Some(c) = self.peek() {
                if c.is_digit(10) {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Parse exponent part
        if let Some(c) = self.peek() {
            if c == 'e' || c == 'E' {
                number.push(self.advance().unwrap()); // Consume 'e' or 'E'
                if let Some(c) = self.peek() {
                    if c == '+' || c == '-' {
                        number.push(self.advance().unwrap()); // Consume '+' or '-'
                    }
                }
                while let Some(c) = self.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn parse_integer_suffix(&mut self) -> Option<NumberSuffix> {
        let mut unsigned = false;
        let mut long_count = 0;

        loop {
            match self.peek() {
                Some('u') | Some('U') => {
                    if unsigned {
                        break; // Duplicate 'u'/'U' not allowed
                    }
                    unsigned = true;
                    self.advance();
                }
                Some('l') | Some('L') => {
                    let mut l_count = 0;
                    while let Some('l') | Some('L') = self.peek() {
                        l_count += 1;
                        self.advance();
                    }
                    if l_count == 1 || l_count == 2 {
                        long_count += l_count as u8;
                    } else {
                        return Some(NumberSuffix::Integer(IntegerSuffix::new(unsigned, long_count)));
                    }
                }
                _ => break,
            }
        }

        if unsigned || long_count > 0 {
            Some(NumberSuffix::Integer(IntegerSuffix::new(unsigned, long_count)))
        } else {
            None
        }
    }

    fn parse_floating_point_suffix(&mut self) -> Option<NumberSuffix> {
        match self.peek() {
            Some('f') | Some('F') => {
                self.advance();
                Some(NumberSuffix::FloatingPoint(FloatingPointSuffix::Float))
            }
            Some('l') | Some('L') => {
                self.advance();
                Some(NumberSuffix::FloatingPoint(FloatingPointSuffix::LongDouble))
            }
            _ => None,
        }
    }

    fn parse_block_comment(&mut self) -> Token {
        let mut comment = String::new();

        let mut closed = false;
        while let Some(c) = self.peek() {
            if c == '*' && self.input.get(self.position + 1) == Some(&'/') {
                comment.push(c);
                self.advance(); // Consume '*'
                comment.push(self.advance().unwrap()); // Consume '/' and add to comment
                closed = true;
                break;
            } else {
                comment.push(c);
                self.advance();
            }
        }

        if closed {
            comment.pop();
            comment.pop();
            comment = comment.trim().to_string();
            Token::Comment(comment)
        } else {
            Token::Error("Unterminated block comment".to_string())
        }
    }

    fn parse_line_comment(&mut self) -> Token {
        let mut comment = String::new();
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        // trim leading and trailing whitespace
        comment = comment.trim().to_string();
        Token::Comment(comment)
    }

    fn parse_char_literal(&mut self) -> Token {
        self.advance(); // Consume opening quote
        let char_lit = if let Some(c) = self.advance() {
            c
        } else {
            return Token::Error("Unterminated character literal".to_string());
        };

        if self.peek() == Some('\'') {
            self.advance(); // Consume closing quote
            Token::CharLiteral(char_lit.to_string())
        } else {
            Token::Error("Unclosed character literal".to_string())
        }
    }

    fn parse_string_literal(&mut self) -> Token {
        self.advance(); // Consume opening quote
        let mut string_lit = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance(); // Consume closing quote
                break;
            }
            string_lit.push(c);
            self.advance();
        }
        Token::StringLiteral(string_lit)
    }

    fn parse_symbol(&mut self) -> Token {
        let max_symbol_len = SYMBOLS.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut symbol = String::new();
        let mut temp_position = self.position;

        for _ in 0..max_symbol_len {
            if let Some(c) = self.input.get(temp_position) {
                symbol.push(*c);
                temp_position += 1;
            } else {
                break;
            }
        }

        for i in (1..=symbol.len()).rev() {
            let candidate = &symbol[..i];
            if SYMBOLS.contains(&candidate) {
                for _ in 0..i {
                    self.advance();
                }
                return Token::Symbol(candidate.to_string());
            }
        }

        Token::Error(format!("Unknown symbol starting with '{}'", self.advance().unwrap()))
    }

    pub fn next_token(&mut self) -> Option<PositionedToken> {
        self.skip_whitespace();

        let start_row = self.row;
        let start_column = self.column;

        let token = match self.peek()? {
            c if c.is_alphabetic() || c == '_' => self.parse_identifier_or_keyword(),
            c if c.is_digit(10) => self.parse_number(),
            '/' => {
                if self.input.get(self.position + 1) == Some(&'*') {
                    self.advance(); // Consume '/'
                    self.advance(); // Consume '*'
                    self.parse_block_comment()
                } else if self.input.get(self.position + 1) == Some(&'/') {
                    self.advance(); // Consume '/'
                    self.advance(); // Consume '/'
                    self.parse_line_comment()
                } else {
                    self.advance(); // Consume '/'
                    Token::Symbol("/".to_string())
                }
            },
            '\'' => self.parse_char_literal(),
            '"' => self.parse_string_literal(),
            c if SYMBOLS.iter().any(|s| s.starts_with(c)) => self.parse_symbol(),
            _ => {
                let unexpected_char = self.advance().unwrap();
                Token::Error(format!("Unexpected character: {}", unexpected_char))
            },
        };

        Some(PositionedToken {
            token,
            row: start_row,
            column: start_column,
        })
    }    

    pub fn parse(&mut self) -> Vec<TokenLine> {
        let mut token_lines: Vec<TokenLine> = Vec::new();
        let mut current_line = TokenLine {
            line_number: 1,
            tokens: Vec::new(),
        };

        while let Some(positioned_token) = self.next_token() {
            // If the token is on a new line, create a new TokenLine
            if positioned_token.row != current_line.line_number {
                // Only add non-empty token lines
                if !current_line.tokens.is_empty() {
                    token_lines.push(current_line);
                }
                current_line = TokenLine {
                    line_number: positioned_token.row,
                    tokens: vec![positioned_token],
                };
            } else {
                current_line.tokens.push(positioned_token);
            }
        }

        // Add the last line if it's not empty
        if !current_line.tokens.is_empty() {
            token_lines.push(current_line);
        }

        token_lines
    }

    // Just parse as tokens    
    pub fn tokenize(&mut self) -> Vec<PositionedToken> {
        let mut tokens: Vec<PositionedToken> = Vec::new();

        while let Some(positioned_token) = self.next_token() {
            tokens.push(positioned_token);
        }

        tokens
    }
}
