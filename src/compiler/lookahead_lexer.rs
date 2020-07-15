use super::lexer::{Lexer, Token};

pub struct LookaheadLexer {
	lexer: Lexer,
	index: usize,
	line_offset: usize,
	line_number: usize,
}

impl LookaheadLexer {
	pub fn new(content: String) -> LookaheadLexer {
		let lexer = Lexer::new(content);
		let index = lexer.index;
		let line_offset = lexer.line_offset;
		let line_number = lexer.line_number;

		LookaheadLexer {
			lexer,
			index,
			line_offset,
			line_number,
		}
	}

	pub fn next_lookahead(&mut self) -> Token {
		self.lexer.index = self.index;
		self.lexer.line_offset = self.line_offset;
		self.lexer.line_number = self.line_number;
		self.lexer.next()
	}

	pub fn next(&mut self) -> Token {
		self.lexer.index = self.index;
		self.lexer.line_offset = self.line_offset;
		self.lexer.line_number = self.line_number;

		let token = self.lexer.next();

		self.index = self.lexer.index;
		self.line_offset = self.lexer.line_offset;
		self.line_number = self.lexer.line_number;

		token
	}
}
