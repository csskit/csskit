use super::prelude::*;
use crate::Computed;

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
#[visit]
pub struct PropertyRule<'a> {
	#[visit(skip)]
	#[atom(CssAtomSet::Property)]
	pub name: T![AtKeyword],
	pub prelude: PropertyPrelude,
	pub block: PropertyRuleBlock<'a>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct PropertyPrelude(T![DashedIdent]);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PropertyRuleBlock<'a>(DeclarationList<'a, PropertyRuleValue<'a>>);

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum PropertyRuleValue<'a> {
	InitialValue(ComponentValues<'a>),
	Syntax(SyntaxValue),
	Inherits(InheritsValue),
	Unknown(ComponentValues<'a>),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum InheritsValue {
	#[atom(CssAtomSet::True)]
	True(T![Ident]),
	#[atom(CssAtomSet::False)]
	False(T![Ident]),
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SyntaxValue(T![String]);

impl<'a> DeclarationValue<'a> for PropertyRuleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
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

	fn parse_declaration_value(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::InitialValue => Self::InitialValue(p.parse::<ComponentValues<'a>>()?),
			CssAtomSet::Inherits => Self::Inherits(p.parse::<InheritsValue>()?),
			CssAtomSet::Syntax => Self::Syntax(p.parse::<SyntaxValue>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PropertyRule>(), 88);
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
