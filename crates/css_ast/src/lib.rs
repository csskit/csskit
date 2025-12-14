#![deny(warnings)]

mod css_atom_set;
mod diagnostics;
mod functions;
mod metadata;
mod properties;
mod property_atoms;
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
#[cfg(feature = "visitable")]
pub mod visit;

pub use css_atom_set::*;
pub use css_parse::{Declaration, DeclarationValue, Diagnostic};
pub use functions::*;
pub use metadata::*;
pub use properties::*;
pub use rules::*;
pub use selector::*;
pub use stylerule::*;
pub use stylesheet::*;
pub use traits::*;
pub use types::*;
pub use units::*;
pub use values::*;
#[cfg(feature = "visitable")]
pub use visit::*;

use crate::diagnostics::CssDiagnostic;

use css_parse::{
	Cursor, CursorSink, NodeMetadata, NodeWithMetadata, Parse, Parser, Peek, Result as ParserResult, SemanticEq, Span,
	ToCursors, ToSpan,
};

// TODO! - delete this when we're done ;)
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Peek<'a> for Todo {
	fn peek<I>(_p: &Parser<'a, I>, _c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		false
	}
}

impl<'a> Parse<'a> for Todo {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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

impl SemanticEq for Todo {
	fn semantic_eq(&self, _: &Self) -> bool {
		false
	}
}

impl<M: NodeMetadata> NodeWithMetadata<M> for Todo {
	fn metadata(&self) -> M {
		M::default()
	}
}
