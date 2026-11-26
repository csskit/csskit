use super::prelude::*;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};

/// <https://drafts.css-houdini.org/css-properties-values-api/#at-property-rule>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Property, property_kinds = Name)]
pub struct PropertyRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Property)]
	pub name: T![AtKeyword],
	pub prelude: PropertyPrelude,
	#[metadata(block)]
	pub block: PropertyRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for PropertyRule<'a> {
	const NODE_ID: NodeId = NodeId::PropertyRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.ident()),
			_ => None,
		}
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PropertyPrelude(T![DashedIdent]);

impl PropertyPrelude {
	/// Returns a cursor to the dashed identifier (e.g., `--my-color`).
	pub fn ident(&self) -> Cursor {
		self.0.into()
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PropertyRuleBlock<'a>(pub DeclarationList<'a, PropertyRuleStyleValue<'a>, CssMetadata>);

#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum PropertyRuleStyleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(SyntaxValue),
	Inherits(InheritsValue),
	Unknown(ComponentValues<'a>),
}

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InheritsValue {
	#[atom(CssAtomSet::True)]
	True(T![Ident]),
	#[atom(CssAtomSet::False)]
	False(T![Ident]),
}

#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SyntaxValue(T![String]);

impl<'a, M: NodeMetadata> DeclarationValue<'a, M> for PropertyRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::InitialValue | CssAtomSet::Inherits | CssAtomSet::Syntax)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let stop = p.set_stop(KindSet::RIGHT_CURLY_OR_SEMICOLON);
		let value = match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::InitialValue => p.parse::<ComponentValues<'a>>().map(Self::InitialValue),
			CssAtomSet::Inherits => p.parse::<InheritsValue>().map(Self::Inherits),
			CssAtomSet::Syntax => p.parse::<SyntaxValue>().map(Self::Syntax),
			_ => p.parse::<ComponentValues<'a>>().map(Self::Unknown),
		};
		p.set_stop(stop);
		value
	}
}

impl<'a, M: NodeMetadata> NodeWithMetadata<M> for PropertyRuleStyleValue<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(
			CssAtomSet::ATOMS,
			PropertyRule,
			r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#
		);
	}

	#[test]
	fn test_initial_value_ends_at_semicolon() {
		let source = "@property --foo{initial-value: #000 ;inherits:false}";
		let arena = css_parse::Arena::default();
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, source);
		let mut parser = Parser::new(&arena, source, lexer);
		let rule = parser.parse_entirely::<PropertyRule>().output.unwrap();
		let declarations = &rule.block.0.declarations;
		assert_eq!(declarations.len(), 2);
		assert!(matches!(declarations[0].value, PropertyRuleStyleValue::InitialValue(_)));
		assert!(matches!(declarations[1].value, PropertyRuleStyleValue::Inherits(_)));
	}
}
