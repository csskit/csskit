use crate::{CliResult, GlobalConfig, InputArgs};
use clap::Args;
use css_ast::CssAtomSet;
use css_lexer::{Kind, Lexer};
use std::io::Read;

/// Show the debug output for lexed tokens from a file
#[derive(Debug, Args)]
pub struct DbgLex {
	#[command(flatten)]
	content: InputArgs,
}

impl DbgLex {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let DbgLex { content } = self;
		for (_file_name, mut source) in content.sources()? {
			let mut source_string = String::new();
			source.read_to_string(&mut source_string)?;
			let source_text = source_string.as_str();
			let mut lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);

			loop {
				let offset = lexer.offset();
				let token = lexer.advance();
				let kind = token.kind();

				if kind == Kind::Eof {
					break;
				}

				let cursor = token.with_cursor(offset);
				let slice = cursor.str_slice(lexer.source());

				println!("{:?} @ {} (len={}): {:?}", kind, offset.0, token.len(), slice);
			}
		}
		Ok(())
	}
}
