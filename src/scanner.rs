use crate::token;
use crate::token::{Token, TokenType};
use crate::error::{error};

struct ScannerState {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl ScannerState {
    fn new(source: &String) -> ScannerState {
        let source: Vec<char> = source.chars().collect();

        ScannerState {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn get_current_lexeme(&self) -> Vec<char> {
        self.source[self.start..self.current].to_vec()
    }

    fn match_next_and_add(&mut self, char: &str, left_type: TokenType, right_type: TokenType) {
        if self.is_at_end() {
        } else if char == self.source[self.current + 1].to_string().as_str() {
            self.current += 1;
            self.add_token(left_type)
        } else {
            self.add_token(right_type)
        }
    }

    fn match_next(&mut self, char: &str) -> bool {
        if self.is_at_end() {
            false
        } else if char == self.source[self.current + 1].to_string().as_str() {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            String::from_iter(self.get_current_lexeme()),
            self.line,
        ));
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == self.current
    }

    fn scan_string(&mut self) {
        let current_init = self.current;
        'loop_: for char_ in self.source[current_init..].iter() {
            self.current += 1;
            if *char_ == '"' {
                break 'loop_;
            } else if *char_ == '\n' {
                self.line += 1;
            }
        }

        if self.current == self.source.len(){
            error(self.line,"string not ended");
        }
    }

    fn scan_token(&mut self) {
        match self.source[self.current] {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '{' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => self.match_next_and_add("=", TokenType::BangEqual, TokenType::BANG),
            '=' => self.match_next_and_add("=", TokenType::EqualEqual, TokenType::EQUAL),
            '<' => self.match_next_and_add("=", TokenType::LessEqual, TokenType::LESS),
            '>' => self.match_next_and_add("=", TokenType::GreaterEqual, TokenType::GREATER),
            '/' => {
                if self.match_next("/") {
                    let current_init = self.current;
                    'loop_: for char_ in self.source[current_init..].iter() {
                        if *char_ == '\n' {
                            break 'loop_;
                        } else {
                            self.current += 1;
                        }
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }

            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                self.line += 1;
            }
            '"' => self.scan_string(),
            _ => (),
        };
    }
}



pub fn scan_source(source: &String) -> Vec<Token> {
    let mut scanner = ScannerState::new(source);

    while scanner.current < source.len() {

        scanner.start = scanner.current;
        scanner.scan_token();
        scanner.current += 1;
    }
    scanner.tokens
}
