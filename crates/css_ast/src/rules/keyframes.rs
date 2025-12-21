use super::prelude::*;
use crate::Percentage;
use css_parse::NoBlockAllowed;

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.keyframes"))]
pub struct KeyframesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Keyframes)]
	pub name: T![AtKeyword],
	pub prelude: KeyframesName,
	pub block: KeyframesRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for KeyframesRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { used_at_rules: AtRuleId::Keyframes, node_kinds: NodeKinds::AtRule, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum KeyframesName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframesName {
	fn invalid_ident(atom: CssAtomSet) -> bool {
		matches!(atom, CssAtomSet::Default | CssAtomSet::Initial | CssAtomSet::Unset | CssAtomSet::None)
	}
}

// Must use Parse rather than Build so ReservedKeyframeName errors can be emitted
impl<'a> Parse<'a> for KeyframesName {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![String]>() {
			return Ok(Self::String(p.parse::<T![String]>()?));
		}
		let ident = p.parse::<T![Ident]>()?;
		if KeyframesName::invalid_ident(p.to_atom::<CssAtomSet>(ident.into())) {
			Err(Diagnostic::new(ident.into(), Diagnostic::reserved_keyframe_name))?
		}
		Ok(Self::Ident(ident))
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct KeyframesRuleBlock<'a>(pub RuleList<'a, Keyframe<'a>, CssMetadata>);

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
pub struct Keyframe<'a>(
	QualifiedRule<'a, KeyframeSelectors<'a>, StyleValue<'a>, NoBlockAllowed<StyleValue<'a>, CssMetadata>, CssMetadata>,
);

impl<'a> NodeWithMetadata<CssMetadata> for Keyframe<'a> {
	fn metadata(&self) -> CssMetadata {
		self.0.metadata()
	}
}

#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct KeyframeSelectors<'a>(pub CommaSeparated<'a, KeyframeSelector>);

#[derive(Peek, Parse, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum KeyframeSelector {
	#[atom(CssAtomSet::From)]
	From(T![Ident]),
	#[atom(CssAtomSet::To)]
	To(T![Ident]),
	Percent(Percentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<KeyframesRule>(), 120);
		assert_eq!(std::mem::size_of::<KeyframeSelector>(), 16);
		assert_eq!(std::mem::size_of::<KeyframesName>(), 16);
		assert_eq!(std::mem::size_of::<KeyframesRuleBlock>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes foo{}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes\"include\"{}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{0%{rotate:0deg}100%{rotate:360deg}}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{from,0%{rotate:0deg}to,100%{rotate:360deg}}");
		assert_parse!(CssAtomSet::ATOMS, KeyframesRule, "@keyframes spin{to{rotate:360deg}}");
		assert_parse!(
			CssAtomSet::ATOMS,
			KeyframesRule,
			"@keyframes x{to{animation-timing-function:cubic-bezier(0,0,0.2,1)}}"
		);
	}
}
