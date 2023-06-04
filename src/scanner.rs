use std::{rc::Rc};

pub struct Scanner {
    pub start : usize,
    pub current : usize,
    pub line : usize,
    pub source : Rc<String>
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
  TokenLeftParen, TokenRightParen,
  TokenLeftBrace, TokenRightBrace,
  TokenComma, TokenDot, TokenMinus, TokenPlus,
  TokenSemicolon, TokenSlash, TokenStar,
  // One or two character tokens.
  TokenBang, TokenBangEqual,
  TokenEqual, TokenEqualEqual,
  TokenGreater, TokenGreaterEqual,
  TokenLess, TokenLessEqual,
  // Literals.
  TokenIdentifier, TokenString, TokenNumber,
  // Keywords.
  TokenAnd, TokenClass, TokenElse, TokenFalse,
  TokenFor, TokenFun, TokenIf, TokenNil, TokenOr,
  TokenPrint, TokenReturn, TokenSuper, TokenThis,
  TokenTrue, TokenVar, TokenWhile,

  TokenError, TokenEof
}

pub struct Token {
    pub token_type : TokenType,
    pub value : String,
    pub line : usize
}

impl Scanner {
    fn is_at_end(&mut self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&mut self, token_type : TokenType) -> Token {
        let value = self.source.chars().skip(self.start).take(self.current - self.start).collect();
        Token { token_type: token_type, value: value, line: self.line }
    }

    fn error_token(&self, message : &str) -> Token {
        Token { token_type: TokenType::TokenError, value: message.to_string(), line: self.line }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).expect("should have been able to advance")
    }

    fn match_char(&mut self, to_match : char) -> bool {
        if self.is_at_end() {
            false
        } else {
            let c = self.source.chars().nth(self.current + 1).unwrap();
            if c == to_match {
                self.current += 1;
                true
            } else {
                false
            }
        }        
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c : char = self.peek();
            match c {
                ' ' => { self.advance(); },
                '\t' => { self.advance(); },
                '\r' => { self.advance(); },
                '\n' => { self.line += 1; self.advance(); break },
                '\\' => {
                    let next = self.peek_next();
                    if let Some('\\') = next {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }
                }
                _ => break
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().or(Some('a')).unwrap().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        self.make_token(TokenType::TokenNumber)
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        self.advance();
        self.make_token(TokenType::TokenString)
    }

/**
 *  case 'a': return checkKeyword(1, 2, "nd", TOKEN_AND);
    case 'c': return checkKeyword(1, 4, "lass", TOKEN_CLASS);
    case 'e': return checkKeyword(1, 3, "lse", TOKEN_ELSE);
    case 'i': return checkKeyword(1, 1, "f", TOKEN_IF);
    case 'n': return checkKeyword(1, 2, "il", TOKEN_NIL);
    case 'o': return checkKeyword(1, 1, "r", TOKEN_OR);
    case 'p': return checkKeyword(1, 4, "rint", TOKEN_PRINT);
    case 'r': return checkKeyword(1, 5, "eturn", TOKEN_RETURN);
    case 's': return checkKeyword(1, 4, "uper", TOKEN_SUPER);
    case 'v': return checkKeyword(1, 2, "ar", TOKEN_VAR);
    case 'w': return checkKeyword(1, 4, "hile", TOKEN_WHILE);
 */
    fn identifier_type(&mut self) -> TokenType {
        let error_message = "in identifier type. should have been able to do this.";
        match self.source.chars().nth(self.start).expect(error_message) {
            'a' => self.check_keyword("and", TokenType::TokenAnd),
            'c' => self.check_keyword("class", TokenType::TokenClass),
            'e' => self.check_keyword("else", TokenType::TokenElse),
            'i' => self.check_keyword("if", TokenType::TokenIf),
            'n' => self.check_keyword("nil", TokenType::TokenNil),
            'o' => self.check_keyword("or", TokenType::TokenOr),
            'p' => self.check_keyword("print", TokenType::TokenPrint),
            'r' => self.check_keyword("return", TokenType::TokenReturn),
            's' => self.check_keyword("super", TokenType::TokenSuper),
            'v' => self.check_keyword("var", TokenType::TokenVar),
            'w' => self.check_keyword("while", TokenType::TokenWhile),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(error_message) {
                        'a' => self.check_keyword("false", TokenType::TokenFalse),
                        'o' => self.check_keyword("for", TokenType::TokenFor),
                        'u' => self.check_keyword("fun", TokenType::TokenFun),
                        _ => TokenType::TokenIdentifier
                    } 
                } else {
                        TokenType::TokenIdentifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(error_message) {
                        'h' => self.check_keyword("this", TokenType::TokenThis),
                        'r' => self.check_keyword("true", TokenType::TokenTrue),
                        _ => TokenType::TokenIdentifier
                    } 
                } else {
                        TokenType::TokenIdentifier
                }
            }
            _ => TokenType::TokenIdentifier
        }
    }

    fn check_keyword(&mut self, to_match : &str, if_match_return : TokenType ) -> TokenType {
        let got : String = self.source.chars().skip(self.start).take(to_match.len()).collect();
        if to_match.to_string() == got {
            if_match_return
        } else {
            TokenType::TokenIdentifier
        }
    }

    fn identifier(&mut self) -> Token {
        self.advance();
        while self.peek().is_alphanumeric() || self.peek() == '_'  {
            self.advance();
        }
        let token_type = self.identifier_type();
        self.make_token(token_type)
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::TokenEof);
        }

        let c : char = self.advance();
        if c.is_alphabetic() {
            return self.identifier();
        }
        if c.is_ascii_digit() {
            return self.number();
        }
        match c {
            '(' => self.make_token(TokenType::TokenLeftParen),
            ')' => self.make_token(TokenType::TokenRightParen),
            '{' => self.make_token(TokenType::TokenLeftBrace),
            '}' => self.make_token(TokenType::TokenRightBrace),
            ';' => self.make_token(TokenType::TokenSemicolon),
            ',' => self.make_token(TokenType::TokenComma),
            '.' => self.make_token(TokenType::TokenDot),
            '-' => self.make_token(TokenType::TokenMinus),
            '+' => self.make_token(TokenType::TokenPlus),
            '/' => self.make_token(TokenType::TokenSlash),
            '*' => self.make_token(TokenType::TokenStar),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::TokenBangEqual)
                } else {
                    self.make_token(TokenType::TokenBang)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::TokenEqualEqual)
                } else {
                    self.make_token(TokenType::TokenEqual)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::TokenLessEqual)
                } else {
                    self.make_token(TokenType::TokenLess)
                }                
            },
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::TokenGreaterEqual)
                } else {
                    self.make_token(TokenType::TokenGreater)
                }                
            },
            '"' => self.string(),
            _ => self.error_token("unexpected character.")
        }
    }
}

pub fn init_scanner(source : Rc<String>) -> Scanner {
    Scanner { start: 0, current: 0, line: 1, source: source }
}