use super::prelude::*;
use crate::Computed;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
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
	#[metadata(delegate)]
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

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PropertyRuleBlock<'a>(#[metadata(delegate)] DeclarationList<'a, PropertyRuleStyleValue<'a>, CssMetadata>);

#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum PropertyRuleStyleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(SyntaxValue),
	Inherits(InheritsValue),
	Unknown(ComponentValues<'a>),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InheritsValue {
	#[atom(CssAtomSet::True)]
	True(T![Ident]),
	#[atom(CssAtomSet::False)]
	False(T![Ident]),
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SyntaxValue(T![String]);

impl<'a, M: NodeMetadata> DeclarationValue<'a, M> for PropertyRuleStyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::InitialValue | CssAtomSet::Inherits | CssAtomSet::Syntax)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

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
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::InitialValue => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
			CssAtomSet::Inherits => Self::Inherits(p.parse::<InheritsValue>()?),
			CssAtomSet::Syntax => Self::Syntax(p.parse::<SyntaxValue>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
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
	fn size_test() {
		assert_eq!(std::mem::size_of::<PropertyRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(
			CssAtomSet::ATOMS,
			PropertyRule,
			r#"@property --foo{initial-value:0;inherits:false;syntax:"<length>"}"#
		);
	}
}
