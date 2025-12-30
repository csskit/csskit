use css_lexer::{Cursor, Kind, KindSet};
use css_parse::{DeclarationList, DeclarationValue, NodeWithMetadata, Parser, Peek, Result as ParserResult, T};
use csskit_derives::*;

use crate::CsskitAtomSet;

/// The `@stat` rule defines a named statistic for collecting CSS metrics.
///
/// # Syntax
///
/// ```css
/// @stat --stat-name {
///   type: counter | bytes | lines | unique | list;
/// }
/// ```
///
/// # Examples
///
/// ```css
/// @stat --total-selectors { type: counter; }
/// @stat --unique-selectors { type: unique; }
/// @stat --complexity-values { type: list; }
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatRule<'a> {
	#[atom(CsskitAtomSet::Stat)]
	pub at_keyword: T![AtKeyword],
	pub name: T![DashedIdent],
	pub block: StatRuleBlock<'a>,
}

/// The block of a `@stat` rule containing declarations.
///
/// Declarations can be:
/// - `type: counter | bytes | lines | unique | list`
pub type StatRuleBlock<'a> = DeclarationList<'a, StatDeclarationValue, ()>;

/// A declaration value within a `@stat` rule block.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatDeclarationValue {
	/// The `type:` declaration specifying the stat type.
	Type(StatTypeValue),
}

impl NodeWithMetadata<()> for StatDeclarationValue {
	fn metadata(&self) {}
}

impl<'a> DeclarationValue<'a, ()> for StatDeclarationValue {
	type ComputedValue = T![Eof];

	fn is_initial(&self) -> bool {
		false
	}

	fn is_inherit(&self) -> bool {
		false
	}

	fn is_unset(&self) -> bool {
		false
	}

	fn is_revert(&self) -> bool {
		false
	}

	fn is_revert_layer(&self) -> bool {
		false
	}

	fn needs_computing(&self) -> bool {
		false
	}

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if c != Kind::Ident {
			return false;
		}
		let atom = p.to_atom::<CsskitAtomSet>(c);
		matches!(atom, CsskitAtomSet::Type)
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let atom = p.to_atom::<CsskitAtomSet>(name);
		match atom {
			CsskitAtomSet::Type => Ok(Self::Type(p.parse::<StatTypeValue>()?)),
			_ => Err(css_parse::Diagnostic::new(name, css_parse::Diagnostic::unexpected))?,
		}
	}
}

impl<'a> Peek<'a> for StatDeclarationValue {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);
}

/// The value for a `type:` declaration in a `@stat` rule.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatTypeValue {
	/// `type: counter` - counts matching nodes
	#[atom(CsskitAtomSet::Counter)]
	Counter(T![Ident]),
	/// `type: bytes` - sums byte sizes of matching nodes
	#[atom(CsskitAtomSet::Bytes)]
	Bytes(T![Ident]),
	/// `type: lines` - counts lines in matching nodes
	#[atom(CsskitAtomSet::Lines)]
	Lines(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_counter_type() {
		assert_parse!(CsskitAtomSet::ATOMS, StatRule, "@stat --total-selectors{type:counter}");
	}

	#[test]
	fn test_bytes_type() {
		assert_parse!(CsskitAtomSet::ATOMS, StatRule, "@stat --stylesheet-bytes{type:bytes}");
	}

	#[test]
	fn test_lines_type() {
		assert_parse!(CsskitAtomSet::ATOMS, StatRule, "@stat --source-lines{type:lines}");
	}
}
