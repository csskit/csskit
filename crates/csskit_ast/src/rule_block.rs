use crate::{CsskitAtomSet, WhenRule};
use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, KindSet};
use css_parse::{
	Block, ComponentValues, DeclarationValue, Diagnostic, NodeWithMetadata, Parse, Parser, Peek,
	Result as ParserResult, RuleVariants, T,
};
use csskit_derives::*;

/// A block containing declarations and optionally nested node rules.
///
/// Used by both `NodeRule` and `WhenRule`.
pub type RuleBlock<'a> = Block<'a, RuleDeclarationValue<'a>, NestedRule<'a>, ()>;

/// A nested rule within a block.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NestedRule<'a> {
	/// A nested node rule.
	NodeRule(NestedNodeRule<'a>),
	/// A nested @when rule.
	WhenRule(WhenRule<'a>),
	/// Unknown rule (for error recovery).
	Unknown(ComponentValues<'a>),
}

impl<'a> Parse<'a> for NestedRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

impl<'a> RuleVariants<'a> for NestedRule<'a> {
	type DeclarationValue = RuleDeclarationValue<'a>;
	type Metadata = ();

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let atom = p.to_atom::<CsskitAtomSet>(c);
		match atom {
			CsskitAtomSet::When => Ok(Self::WhenRule(p.parse::<WhenRule>()?)),
			_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
		}
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::Unknown(p.parse::<ComponentValues>()?))
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::NodeRule(p.parse::<NestedNodeRule>()?))
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::Unknown(p.parse::<ComponentValues>()?))
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}
}

impl<'a> Peek<'a> for NestedRule<'a> {
	fn peek<I>(_p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c != Kind::Eof && c != Kind::RightCurly
	}
}

impl NodeWithMetadata<()> for NestedRule<'_> {
	fn metadata(&self) {}
}

/// A nested node rule (selector + block).
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NestedNodeRule<'a> {
	pub selector: crate::QuerySelectorList<'a>,
	pub block: RuleBlock<'a>,
}

impl<'a> Parse<'a> for NestedNodeRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let selector = p.parse::<crate::QuerySelectorList>()?;
		let block = p.parse::<RuleBlock>()?;
		Ok(Self { selector, block })
	}
}

/// A component of a diagnostic message template.
///
/// Diagnostic messages can be composed of:
/// - String literals: `"text"`
/// - Dashed identifiers (stat references): `--stat-name`
/// - Diagnostic functions: `attr(name)`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticComponent {
	/// A string literal.
	String(T![String]),
	/// A dashed identifier (stat reference).
	DashedIdent(T![DashedIdent]),
	/// A diagnostic function.
	Function(DiagnosticFunction),
}

/// A diagnostic function component.
///
/// Supported functions:
/// - `attr(name)` - Get an attribute value from the matched node
/// - `size()` - Get the size of the matched node
#[derive(ToCursors, ToSpan, Peek, Parse, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticFunction {
	/// The `attr(name)` function.
	Attr(DiagnosticAttrFunction),
	/// The `size()` function.
	Size(DiagnosticSizeFunction),
}

/// The `attr(name)` diagnostic function.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticAttrFunction {
	#[atom(CsskitAtomSet::Attr)]
	pub open: T![Function],
	pub name: T![Ident],
	pub close: T![')'],
}

/// The `size()` diagnostic function.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticSizeFunction {
	#[atom(CsskitAtomSet::Size)]
	pub open: T![Function],
	pub close: T![')'],
}

/// The value of a declaration within a rule block.
///
/// Declarations can be:
/// - `collect: --stat-name;` - collect stats (value is a `--stat-name`)
/// - `diagnostic: [ <string> | <ident> | <dashed-ident> | <diagnostic-function> ]+;` - diagnostic message template
/// - `level: warning | error | help;` - diagnostic severity level
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuleDeclarationValue<'a> {
	/// A stat name reference (e.g., `--stat-name`).
	Collect(T![DashedIdent]),
	/// A diagnostic message template composed of multiple components.
	Diagnostic(Vec<'a, DiagnosticComponent>),
	/// A diagnostic severity level.
	Level(DiagnosticLevel),
}

/// Diagnostic severity level: `warning`, `error`, or `advice`.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
	#[atom(CsskitAtomSet::Warning)]
	Warning(T![Ident]),
	#[atom(CsskitAtomSet::Error)]
	Error(T![Ident]),
	#[atom(CsskitAtomSet::Advice)]
	Advice(T![Ident]),
}

impl<'a> NodeWithMetadata<()> for RuleDeclarationValue<'a> {
	fn metadata(&self) {}
}

impl<'a> DeclarationValue<'a, ()> for RuleDeclarationValue<'a> {
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
		matches!(atom, CsskitAtomSet::Collect | CsskitAtomSet::Diagnostic | CsskitAtomSet::Level)
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let atom = p.to_atom::<CsskitAtomSet>(name);
		match atom {
			CsskitAtomSet::Collect => Ok(Self::Collect(p.parse::<T![DashedIdent]>()?)),
			CsskitAtomSet::Diagnostic => Ok(Self::Diagnostic(p.parse::<Vec<'_, DiagnosticComponent>>()?)),
			CsskitAtomSet::Level => Ok(Self::Level(p.parse::<DiagnosticLevel>()?)),
			_ => Err(Diagnostic::new(name, Diagnostic::unexpected))?,
		}
	}
}

impl<'a> Peek<'a> for RuleDeclarationValue<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::String]);
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_diagnostic_single_string() {
		assert_parse!(CsskitAtomSet::ATOMS, Vec<'_, DiagnosticComponent>, r#""message""#, _);
	}

	#[test]
	fn test_diagnostic_multiple_components() {
		assert_parse!(CsskitAtomSet::ATOMS, Vec<'_, DiagnosticComponent>, r#""Foo" --times " times""#, _);
	}

	#[test]
	fn test_diagnostic_with_attr_function() {
		assert_parse!(CsskitAtomSet::ATOMS, Vec<'_, DiagnosticComponent>, r#""Foo" attr(name)"#, _);
	}

	#[test]
	fn test_diagnostic_with_size_function() {
		assert_parse!(CsskitAtomSet::ATOMS, Vec<'_, DiagnosticComponent>, r#""Size: " size()"#, _);
	}
}
