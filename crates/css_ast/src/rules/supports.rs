use super::prelude::*;
use crate::selector::ComplexSelector;

///
/// ```md
/// <general-enclosed>
///  │├─╮─ <function-token> ─╭─╮─ <any-value> ─╭─ ")" ─┤│
///     ╰─ "(" ──────────────╯ ╰───────────────╯
///
///
/// <supports-in-parens>
///  │├─╮─ "(" ─ <supports-condition> ─ ")" ─╭──┤│
///     ├─────── <supports-feature> ─────────┤
///     ╰─────── <general-enclosed> ─────────╯
///
/// <supports-feature>
///  │├─ <supports-decl> ──┤│
///
/// <supports-feature>
///  │├─ "(" ─ <declaration> ─ ")" ─┤│
///
///
/// <container-condition> = [ <container-name>? <container-query>? ]!
/// <container-name> = <custom-ident>
/// <container-query> = not <query-in-parens>
///                   | <query-in-parens> [ [ and <query-in-parens> ]* | [ or <query-in-parens> ]* ]
/// <query-in-parens> = ( <container-query> )
///                   | ( <size-feature> )
///                   | style( <style-query> )
///                   | scroll-state( <scroll-state-query> )
///                   | <general-enclosed>
///
/// <https://drafts.csswg.org/css-conditional-3/#at-supports>
/// <https://drafts.csswg.org/css-conditional-3/#at-ruledef-supports>
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
#[visit]
pub struct SupportsRule<'a> {
	#[visit(skip)]
	#[atom(CssAtomSet::Supports)]
	pub name: T![AtKeyword],
	pub prelude: SupportsCondition<'a>,
	pub block: SupportsRuleBlock<'a>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SupportsRuleBlock<'a>(RuleList<'a, Rule<'a>>);

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(T![Ident], SupportsFeature<'a>),
	And(Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for SupportsCondition<'a> {
	type FeatureCondition = SupportsFeature<'a>;
	fn keyword_is_not(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: SupportsFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: SupportsFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for SupportsCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>() || p.peek::<T!['(']>() {
			return Ok(Self::Is(p.parse::<SupportsFeature>()?));
		}
		Self::parse_condition(p)
	}
}

impl<'a> VisitableTrait for SupportsCondition<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => feature.accept(v),
			Self::Not(_, feature) => feature.accept(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
		}
	}
}

impl<'a> VisitableMut for SupportsCondition<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Is(feature) => feature.accept_mut(v),
			Self::Not(_, feature) => feature.accept_mut(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
		}
	}
}

#[allow(clippy::large_enum_variant)] // TODO: Box?
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsFeature<'a> {
	FontTech(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	FontFormat(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	Selector(Option<T!['(']>, T![Function], ComplexSelector<'a>, T![')'], Option<T![')']>),
	Property(T!['('], Declaration<'a, StyleValue<'a>>, Option<T![')']>),
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsFeatureKeyword {
	#[atom(CssAtomSet::FontTech)]
	FontTech(T![Function]),
	#[atom(CssAtomSet::FontFormat)]
	FontFormat(T![Function]),
	#[atom(CssAtomSet::Selector)]
	Selector(T![Function]),
}

impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse_if_peek::<T!['(']>()?;
		if p.peek::<T![Function]>() {
			let function = p.parse::<T![Function]>()?;
			match p.to_atom::<CssAtomSet>(function.into()) {
				CssAtomSet::Selector => {
					let selector = p.parse::<ComplexSelector>()?;
					// End function
					let close = p.parse::<T![')']>()?;
					let open_close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
					Ok(Self::Selector(open, function, selector, close, open_close))
				}
				CssAtomSet::FontTech => {
					todo!();
				}
				CssAtomSet::FontFormat => {
					todo!();
				}
				_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected_function))?,
			}
		} else if let Some(open) = open {
			let property = p.parse::<Declaration<'a, StyleValue<'a>>>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Property(open, property, close))
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

impl<'a> VisitableTrait for SupportsFeature<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::FontTech(_, _, _, _, _) => todo!(),
			Self::FontFormat(_, _, _, _, _) => todo!(),
			Self::Selector(_, _, selector, _, _) => selector.accept(v),
			Self::Property(_, property, _) => property.accept(v),
		}
	}
}

impl<'a> VisitableMut for SupportsFeature<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::FontTech(_, _, _, _, _) => todo!(),
			Self::FontFormat(_, _, _, _, _) => todo!(),
			Self::Selector(_, _, selector, _, _) => selector.accept_mut(v),
			Self::Property(_, property, _) => property.accept_mut(v),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SupportsRule>(), 496);
		assert_eq!(std::mem::size_of::<SupportsCondition>(), 416);
		assert_eq!(std::mem::size_of::<SupportsRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(color:black){}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width:1px){body{width:1px}}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not (width:1--foo){}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width: 1--foo) or (width: 1foo) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width: 1--foo) and (width: 1foo) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width: 100vw) {\n\tbody {\n\t\twidth: 100vw;\n\t}\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not ((text-align-last: justify) or (-moz-text-align-last: justify)) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports((position:-webkit-sticky)or (position:sticky)) {}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports selector(h2 > p) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(selector(h2 > p)) {}", "@supports selector(h2 > p) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not selector(h2 > p) {\n\n}");
		// assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not (selector(h2 > p)) {}", "@supports not selector(h2 > p) {\n\n}");
	}
}
