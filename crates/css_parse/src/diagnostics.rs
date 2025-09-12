use crate::{Cursor, Kind, Span};
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("This declaration wasn't understood, and so was disregarded.")]
#[diagnostic(code(css_parse::BadDeclaration))]
#[help("The declaration contains invalid syntax, and will be ignored.")]
pub struct BadDeclaration(#[label("This is not valid syntax for a declaration.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected `{}`", Kind::from(self.0))]
#[diagnostic(code(css_parse::Unexpected))]
#[help("This is not correct CSS syntax.")]
pub struct Unexpected(#[label("This wasn't expected here")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected identifier '{0}'")]
#[diagnostic(code(css_parse::UnexpectedIdent))]
#[help("There is an extra word which shouldn't be in this position.")]
pub struct UnexpectedIdent(pub String, #[label("Try removing the word here.")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected delimeter '{0}'")]
#[diagnostic(code(css_parse::UnexpectedDelim))]
#[help("Try removing the the character.")]
pub struct UnexpectedDelim(pub char, #[label("This character wasn't understood")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected tag name ':{0}'")]
#[diagnostic(code(css_parse::UnexpectedTag))]
#[help("This isn't a valid tag name.")]
pub struct UnexpectedTag(pub String, #[label("This tag")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected ID selector ':{0}'")]
#[diagnostic(code(css_parse::UnexpectedId))]
#[help("This isn't a valid ID.")]
pub struct UnexpectedId(pub String, #[label("This ID")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Ignored property due to parse error.")]
#[diagnostic(code(css_parse::UnknownDeclaration))]
#[help("This property is going to be ignored because it doesn't look valid. If it is valid, please file an issue!")]
pub struct UnknownDeclaration(#[label("This property was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected this to be the end of the file, but there was more content.")]
#[diagnostic(code(css_parse::ExpectedEnd))]
#[help("This is likely a problem with the parser. Please submit a bug report!")]
pub struct ExpectedEnd(#[label("All of this extra content was ignored.")] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected more content but reached the end of the file.")]
#[diagnostic(code(css_parse::UnexpectedEnd))]
#[help("Perhaps this file isn't finished yet?")]
pub struct UnexpectedEnd();

#[derive(Debug, Error, Diagnostic)]
#[error("Expected more content before this curly brace.")]
#[diagnostic(code(css_parse::UnexpectedCloseCurly))]
#[help("This needed more content here")]
pub struct UnexpectedCloseCurly(pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected an identifier but found `{}`", Kind::from(self.0))]
#[diagnostic(code(css_parse::ExpectedIdent))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedIdent(#[label("This should be an identifier")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected the identifier `{0}` but found `{1}`")]
#[diagnostic(code(css_parse::ExpectedIdentOf))]
#[help("Try changing `{1}` to `{0}`.")]
pub struct ExpectedIdentOf(pub &'static str, pub String, #[label("This should be `{0}`")] pub Cursor);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a delimiter but saw `{}`", Kind::from(self.0))]
#[diagnostic(code(css_parse::ExpectedDelim))]
#[help("This is not correct CSS syntax.")]
pub struct ExpectedDelim(#[label("Here")] pub Cursor);
