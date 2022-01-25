use std::collections::HashMap;

mod errors;

#[derive(Clone)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, 
    Comma, Dot, Minus, Plus, SemiColon, Star,

    // One/Two character tokens
    Bang, BangEqual, 
    Equal, DoubleEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Ampersand, DoubleAmpersand,
    Modulo, DoubleModulo,
    ForwardSlash, DoubleForwardSlash,


    // Literals 
    Identifier, String, Number,

    // Keywords
    Class, Func, Var,
    If, Else, And, Or, Not,
    For, While, Null,

    // End of File
    EOF
}


#[derive(Debug)]
#[allow(dead_code)]
pub enum LiteralType {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Null 
}


#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexem: String,
    literal: LiteralType,
    line: u32
}


// TODO: Make extra parameter which is source as a vector of characters 
pub struct Scanner {
    pub source: String,
    pub source_chars: Vec<char>,
    pub token_list: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: u32,
    pub keywords: HashMap<String, TokenType>
}

 
impl Default for Scanner {
    fn default() -> Self {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("func"), TokenType::Func);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("not"), TokenType::Not);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("while"), TokenType::While);
        keywords.insert(String::from("null"), TokenType::Null);

        Scanner {
            source: String::new(),
            source_chars: Vec::new(),
            token_list: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords
        }
    }
}

impl Scanner {
    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    fn add_ntoken(&mut self, t_type: TokenType) {
        let start_1 = self.start as usize;
        let end_1 = self.current as usize;
        let text: String = self.source[start_1..end_1].to_string();
        self.token_list.push(Token{
            token_type: t_type,
            lexem: text,
            literal: LiteralType::Null,
            line: self.line
        });
    }

    fn add_token(&mut self, t_type: TokenType, t_value: LiteralType) {
        let start_1 = self.start;
        let end_1 = self.current;
        let text: String = self.source[start_1..end_1].to_string();
        self.token_list.push(Token{
            token_type: t_type,
            lexem: text,
            literal: t_value,
            line: self.line
        });
    }

    fn expect_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source_chars[self.current] != expected {
            return false;
        }


        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0'
        }

        self.source_chars[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len(){
            return '\0'
        }

        self.source_chars[self.current + 1]
    }

    /// Consumes a multi line string
    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            errors::error(self.line, "Unterminated string".to_string());
        }


        // The closing "
        self.advance();

        // Get string content(excluding the doubel quotes)
        let s: String = self.source[self.start..self.current].to_string();

        self.add_token(TokenType::String, LiteralType::String(s));
    }

    /// Consumes a multi digit integer or floating point value
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for floating point or decimal `.`
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the `.`
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Getting string
        let sub_string: String = self.source[self.start..self.current].trim().to_string();

        // println!("{:?}", sub_string);
        // Converting string to floating point
        let f: f32 = sub_string.parse().unwrap();

        // Passing in the value
        self.add_token(TokenType::Number, LiteralType::Float(f));
    }

    /// Consumes a multi character Identifier, reserved words or variable names
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();


        let t_type = if let Some(val) = self.keywords.get(&text) {
            val.clone()
        } else {
            TokenType::Identifier
        };

        self.add_ntoken(t_type);
    }


    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Handle valid characters
            ')' => self.add_ntoken(TokenType::RightParen),
            '(' => self.add_ntoken(TokenType::LeftParen),
            '{' => self.add_ntoken(TokenType::LeftBrace),
            '}' => self.add_ntoken(TokenType::RightBrace),
            '.' => self.add_ntoken(TokenType::Dot),
            ',' => self.add_ntoken(TokenType::Comma),
            '-' => self.add_ntoken(TokenType::Minus),
            '+' => self.add_ntoken(TokenType::Plus),
            ';' => self.add_ntoken(TokenType::SemiColon),
            '*' => self.add_ntoken(TokenType::Star),
            '!' => {
                if self.expect_char('=') {
                    self.add_ntoken(TokenType::BangEqual);
                } else {
                    self.add_ntoken(TokenType::Bang);
                }
            },
            '=' => {
                if self.expect_char('=') {
                    self.add_ntoken(TokenType::DoubleEqual);
                } else {
                    self.add_ntoken(TokenType::Equal);
                }
            },
            '>' => {
                if self.expect_char('=') {
                    self.add_ntoken(TokenType::GreaterEqual);
                } else {
                    self.add_ntoken(TokenType::Greater);
                }
            },
            '<' => {
                if self.expect_char('=') {
                    self.add_ntoken(TokenType::LessEqual);
                } else {
                    self.add_ntoken(TokenType::Less);
                }
            },
            '/' => {
                if self.expect_char('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    } 
                } else {
                    self.add_ntoken(TokenType::ForwardSlash);
                }
            }
            '"' => {
                self.string();
            }

            // Ignore useless characters
            ' ' => {},
            '\t' => {},
            '\r' => {},

            // Go to next line on `\n`
            '\n' => {
                self.line += 1;
            },

            // Check Default case
            _ => {
                // If character is a digit, make a number
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    // Report error if invalid character
                    errors::error(self.line, String::from("Unexpected character"));
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.token_list.push(Token{
            token_type: TokenType::EOF,
            lexem: String::from(""),
            literal: LiteralType::Null,
            line: self.line
        });
    }
}

