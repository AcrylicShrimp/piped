use super::super::compiler::lookahead_lexer::LookaheadLexer as Lexer;
use super::super::compiler::parser::{parse, AST};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct ImportedPipeline {
	path: PathBuf,
	ast_vec: Vec<AST>,
}

impl ImportedPipeline {
	pub fn import(path: &Path) -> Result<ImportedPipeline, String> {
		let canonicalized_path = path.canonicalize().map_err(|err| format!("{}", err))?;
		let mut lexer = Lexer::new(
			canonicalized_path
				.to_str()
				.ok_or("Unable to proceed due to the previous error.".to_owned())?
				.to_owned(),
			read_to_string(canonicalized_path.clone()).map_err(|err| format!("{}", err))?,
		);

		Ok(ImportedPipeline {
			path: canonicalized_path,
			ast_vec: parse(&mut lexer)
				.map_err(|_| "Unable to proceed due to the previous error.".to_owned())?,
		})
	}

	pub fn path(&self) -> &PathBuf {
		&self.path
	}

	pub fn ast_vec(&self) -> &Vec<AST> {
		&self.ast_vec
	}
}
