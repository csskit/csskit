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
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
pub struct SupportsRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Supports)]
	pub name: T![AtKeyword],
	pub prelude: SupportsCondition<'a>,
	pub block: SupportsRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for SupportsRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { used_at_rules: AtRuleId::Supports, node_kinds: NodeKinds::AtRule, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SupportsRuleBlock<'a>(pub RuleList<'a, Rule<'a>, CssMetadata>);

#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(#[atom(CssAtomSet::Not)] T![Ident], SupportsFeature<'a>),
	And(Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for SupportsCondition<'a> {
	type FeatureCondition = SupportsFeature<'a>;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![Function]>() || p.peek::<T!['(']>() {
			return Ok(Self::Is(p.parse::<SupportsFeature>()?));
		}
		Self::parse_condition(p)
	}
}

#[allow(clippy::large_enum_variant)] // TODO: Box?
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum SupportsFeature<'a> {
	FontTech(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		ComponentValues<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![')'],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
	FontFormat(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		ComponentValues<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![')'],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
	Selector(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		ComplexSelector<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![')'],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
	Property(
		#[cfg_attr(feature = "visitable", visit(skip))] T!['('],
		Declaration<'a, StyleValue<'a>, CssMetadata>,
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
}

impl<'a> Peek<'a> for SupportsFeature<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c2 = p.peek_n(2);
		if <T!['(']>::peek(p, c) {
			(<T![Function]>::peek(p, c2)
				&& matches!(
					p.to_atom::<CssAtomSet>(c2),
					CssAtomSet::Selector | CssAtomSet::FontTech | CssAtomSet::FontFormat
				)) || <Declaration<'a, StyleValue<'a>, CssMetadata>>::peek(p, c2)
		} else {
			(<T![Function]>::peek(p, c)
				&& matches!(
					p.to_atom::<CssAtomSet>(c),
					CssAtomSet::Selector | CssAtomSet::FontTech | CssAtomSet::FontFormat
				)) || <Declaration<'a, StyleValue<'a>, CssMetadata>>::peek(p, c)
		}
	}
}
impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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
			let property = p.parse::<Declaration<'a, StyleValue<'a>, CssMetadata>>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Property(open, property, close))
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
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
		assert_eq!(std::mem::size_of::<SupportsRule>(), 656);
		assert_eq!(std::mem::size_of::<SupportsCondition>(), 536);
		assert_eq!(std::mem::size_of::<SupportsRuleBlock>(), 96);
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
