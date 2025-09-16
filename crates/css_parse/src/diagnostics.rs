use crate::{Cursor, Kind};
use css_lexer::Span;
#[cfg(feature = "miette")]
use miette::{MietteDiagnostic, Severity as MietteSeverity};
use std::fmt::{Display, Formatter, Result};

type DiagnosticFormatter = fn(&Diagnostic, &str) -> DiagnosticMeta;

/// An issue that occured during parse time.
#[repr(C, align(64))]
#[derive(Debug, Copy, Clone)]
pub struct Diagnostic {
	/// How severe this error is.
	pub severity: Severity,
	/// The first cursor where this error occured.
	pub start_cursor: Cursor,
	/// The last cursor that was consumed to recover from this error.
	pub end_cursor: Cursor,
	/// A cursor representing what was expected.
	pub desired_cursor: Option<Cursor>,
	/// Function pointer to format the message template with cursor/span data
	pub formatter: DiagnosticFormatter,
}

pub struct DiagnosticMeta {
	pub code: &'static str,
	pub message: String,
	pub help: String,
	pub labels: Vec<(Span, String)>,
}

#[derive(Debug, Clone, Copy)]
pub enum Severity {
	Advice,
	Warning,
	Error,
}

impl Severity {
	pub const fn as_str(&self) -> &str {
		match *self {
			Self::Advice => "Advice",
			Self::Warning => "Warning",
			Self::Error => "Error",
		}
	}
}

impl Display for Severity {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.as_str())
	}
}

#[cfg(feature = "miette")]
impl From<Severity> for MietteSeverity {
	fn from(value: Severity) -> Self {
		match value {
			Severity::Advice => MietteSeverity::Advice,
			Severity::Warning => MietteSeverity::Warning,
			Severity::Error => MietteSeverity::Error,
		}
	}
}

impl Diagnostic {
	/// Create a new diagnostic
	pub fn new(start_cursor: Cursor, formatter: DiagnosticFormatter) -> Self {
		Self { severity: Severity::Error, start_cursor, end_cursor: start_cursor, desired_cursor: None, formatter }
	}

	/// Apply a severity to the given Diagnostic.
	pub fn with_severity(mut self, severity: Severity) -> Self {
		self.severity = severity;
		self
	}

	/// Apply an end Cursor to the given Diagnostic.
	pub fn with_end_cursor(mut self, end_cursor: Cursor) -> Self {
		self.end_cursor = end_cursor;
		self
	}

	/// Get formatted message
	pub fn message(&self, source: &str) -> String {
		let DiagnosticMeta { message, .. } = (self.formatter)(self, source);
		message
	}

	/// Get diagnostic code
	pub fn code(&self, source: &str) -> &'static str {
		let DiagnosticMeta { code, .. } = (self.formatter)(self, source);
		code
	}

	/// Get help text
	pub fn help(&self, source: &str) -> String {
		let DiagnosticMeta { help, .. } = (self.formatter)(self, source);
		help
	}

	/// Add a desired cursor (what was expected)
	pub fn with_desired_cursor(mut self, cursor: Cursor) -> Self {
		self.desired_cursor = Some(cursor);
		self
	}

	/// Convert to a full miette diagnostic for display
	#[cfg(feature = "miette")]
	pub fn into_diagnostic(self, source: &str) -> MietteDiagnostic {
		use miette::LabeledSpan;
		let DiagnosticMeta { code, message, help, mut labels } = (self.formatter)(&self, source);
		let miette_labels = labels.drain(0..).map(|(span, label)| LabeledSpan::new_with_span(Some(label), span));
		MietteDiagnostic::new(message)
			.with_code(code)
			.with_severity(self.severity.into())
			.with_help(help)
			.with_labels(miette_labels)
	}

	// Fomatting functions

	pub fn unexpected(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::Unexpected",
			message: format!("Unexpected `{:?}`", Kind::from(diagnostic.start_cursor)),
			help: "This is not correct CSS syntax.".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_ident(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let message = if start + len <= source.len() {
			let text = &source[start..start + len];
			format!("Unexpected identifier '{text}'")
		} else {
			"Unexpected identifier".to_string()
		};
		DiagnosticMeta {
			code: "css_parse::UnexpectedIdent",
			message,
			help: "There is an extra word which shouldn't be in this position.".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_delim(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let message = if let Some(char) = cursor.token().char() {
			format!("Unexpected delimiter '{char}'")
		} else {
			"Unexpected delimiter".to_string()
		};
		DiagnosticMeta {
			code: "css_parse::UnexpectedDelim",
			message,
			help: "Try removing the character.".into(),
			labels: vec![],
		}
	}

	pub fn expected_ident(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::ExpectedIdent",
			message: format!("Expected an identifier but found `{:?}`", Kind::from(diagnostic.start_cursor)),
			help: "This is not correct CSS syntax.".into(),
			labels: vec![],
		}
	}

	pub fn expected_delim(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::ExpectedDelim",
			message: format!("Expected a delimiter but saw `{:?}`", Kind::from(diagnostic.start_cursor)),
			help: "This is not correct CSS syntax.".into(),
			labels: vec![],
		}
	}

	pub fn bad_declaration(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::BadDeclaration",
			message: "This declaration wasn't understood, and so was disregarded.".to_string(),
			help: "The declaration contains invalid syntax, and will be ignored.".into(),
			labels: vec![],
		}
	}

	pub fn unknown_declaration(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::UnknownDeclaration",
			message: "Ignored property due to parse error.".to_string(),
			help: "This property is going to be ignored because it doesn't look valid. If it is valid, please file an issue!"
				.into(),
			labels: vec![],
		}
	}

	pub fn expected_end(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::ExpectedEnd",
			message: "Expected this to be the end of the file, but there was more content.".to_string(),
			help: "This is likely a problem with the parser. Please submit a bug report!".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_end(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::UnexpectedEnd",
			message: "Expected more content but reached the end of the file.".to_string(),
			help: "Perhaps this file isn't finished yet?".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_close_curly(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "css_parse::UnexpectedCloseCurly",
			message: "Expected more content before this curly brace.".to_string(),
			help: "This needed more content here".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_tag(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let message = if start + len <= source.len() {
			let text = &source[start..start + len];
			format!("Unexpected tag name '{text}'")
		} else {
			"Unexpected tag name".to_string()
		};
		DiagnosticMeta {
			code: "css_parse::UnexpectedTag",
			message,
			help: "This isn't a valid tag name.".into(),
			labels: vec![],
		}
	}

	pub fn unexpected_id(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let message = if start + len <= source.len() {
			let text = &source[start..start + len];
			format!("Unexpected ID selector '{text}'")
		} else {
			"Unexpected ID selector".to_string()
		};
		DiagnosticMeta {
			code: "css_parse::UnexpectedId",
			message,
			help: "This isn't a valid ID.".into(),
			labels: vec![],
		}
	}
}
