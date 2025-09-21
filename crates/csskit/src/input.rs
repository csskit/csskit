use clap::Args;
use std::{
	fs::File,
	io::{BufRead, BufReader, Cursor, Read, Result, StdinLock, stdin},
};

#[derive(Debug)]
pub enum InputSource<'a> {
	Content(Cursor<&'a str>),
	File(BufReader<File>),
	Stdin(StdinLock<'a>),
}

impl<'a> Read for InputSource<'a> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		match self {
			InputSource::Content(cursor) => cursor.read(buf),
			InputSource::File(reader) => reader.read(buf),
			InputSource::Stdin(stdin) => stdin.read(buf),
		}
	}
}

impl<'a> BufRead for InputSource<'a> {
	fn fill_buf(&mut self) -> Result<&[u8]> {
		match self {
			InputSource::Content(cursor) => cursor.fill_buf(),
			InputSource::File(reader) => reader.fill_buf(),
			InputSource::Stdin(stdin) => stdin.fill_buf(),
		}
	}

	fn consume(&mut self, amt: usize) {
		match self {
			InputSource::Content(cursor) => cursor.consume(amt),
			InputSource::File(reader) => reader.consume(amt),
			InputSource::Stdin(stdin) => stdin.consume(amt),
		}
	}
}

#[derive(Debug, Args)]
#[group(required = false, multiple = false)]
pub struct InputArgs {
	/// Process CONTENT as CSS. Use this rather than a file or stdin.
	#[arg(short, long)]
	content: Option<String>,

	/// A list of input files, or `-` to read from stdin.
	#[arg(default_values = &["-"])]
	files: Vec<String>,
}

impl InputArgs {
	pub fn sources(&self) -> Result<Vec<(&str, InputSource<'_>)>> {
		if let Some(content) = &self.content {
			Ok(vec![("<content>", InputSource::Content(Cursor::new(content)))])
		} else {
			let mut sources = Vec::new();
			for file in &self.files {
				let source = if file == "-" {
					InputSource::Stdin(stdin().lock())
				} else {
					InputSource::File(BufReader::new(File::open(file)?))
				};
				sources.push((file.as_str(), source));
			}
			Ok(sources)
		}
	}
}
