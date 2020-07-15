use super::lexer::{Lexer, Token};

pub struct LookaheadLexer {
	lexer: Lexer,
	index: usize,
}

impl LookaheadLexer {
	pub fn new(content: String) -> LookaheadLexer {
		let lexer = Lexer::new(content);
		let index = lexer.index;

		LookaheadLexer { lexer, index }
	}

	pub fn next_lookahead(&mut self) -> Token {
		self.lexer.next()
	}

	pub fn next(&mut self) -> Token {
		self.lexer.index = self.index;
		let token = self.lexer.next();
		self.index = self.lexer.index;
		token
	}
}
