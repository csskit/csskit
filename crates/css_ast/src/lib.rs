#![deny(warnings)]
mod properties;
mod rules;
mod selector;
mod specificity;
mod stylerule;
mod stylesheet;
mod traits;
mod types;
mod units;
mod values;
mod visit;

use csskit_derives::Visitable;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use stylerule::*;
pub use stylesheet::*;
pub use types::*;
pub use units::*;
pub use values::*;
pub use visit::*;

use css_lexer::{Span, ToSpan};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, diagnostics};

pub use css_parse::{Declaration, DeclarationValue};

// TODO! - delete this when we're done ;)
#[derive(Visitable, Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[visit(skip)]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Peek<'a> for Todo {
	fn peek(_p: &Parser<'a>, _c: css_lexer::Cursor) -> bool {
		false
	}
}

impl<'a> Parse<'a> for Todo {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Err(diagnostics::Unimplemented(Span::new(p.offset(), p.offset())))?
	}
}

impl ToCursors for Todo {
	fn to_cursors(&self, _: &mut impl CursorSink) {}
}

impl ToSpan for Todo {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}
