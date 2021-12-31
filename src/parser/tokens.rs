
enum TokenType {
  LeftParen, RightParen,
  EOF

}


enum LiteralType {
	String(String),
	Int(i32),
	Bool(bool),
	Null 
}

struct Token {
	token_type: TokenType,
	lexem: String,
	literal: LiteralType,
	line: u32
}

// impl Token {
// 	fn to_string(&self) -> String {
// 		format!("{} {} {}", self.token_type, self.lexem, self.literal)
// 	}
// }


pub struct Scanner {
	source: String,
	token_list: Vec<Token>,
	start: u32,
	current: u32,
	line: u32,
}

 
impl Default for Scanner {
	fn default() -> Self {
	    Scanner {
	    	source: String::from(""),
	    	token_list: Vec::new(),
	    	start: 0,
	    	current: 0,
	    	line: 1,
	    }
	}
}

impl Scanner {
	fn at_end(&self) -> bool {
		self.current >= self.source.len() as u32
	}

	fn scan_token(&self) {

	}

	fn scan_tokens(&mut self) {
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

