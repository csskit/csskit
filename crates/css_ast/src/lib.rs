#![deny(warnings)]
mod css_atom_set;
mod diagnostics;
mod functions;
mod properties;
mod rules;
mod selector;
mod specificity;
mod stylerule;
mod stylesheet;
#[cfg(test)]
mod test_helpers;
mod traits;
mod types;
mod units;
mod values;
pub mod visit;

pub use css_atom_set::*;
pub use css_parse::{Declaration, DeclarationValue, Diagnostic};
pub use functions::*;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use stylerule::*;
pub use stylesheet::*;
pub use types::*;
pub use units::*;
pub use values::*;
pub use visit::*;

use crate::diagnostics::CssDiagnostic;

use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan};
use csskit_derives::Visitable;

// TODO! - delete this when we're done ;)
#[derive(Visitable, Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[visit(skip)]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Peek<'a> for Todo {
	fn peek(_p: &Parser<'a>, _c: Cursor) -> bool {
		false
	}
}

impl<'a> Parse<'a> for Todo {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Err(Diagnostic::new(p.next(), Diagnostic::unimplemented))?
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
