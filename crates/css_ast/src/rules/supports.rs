use super::prelude::*;
use crate::selector::ComplexSelector;
use crate::{FontFormat, FontTech};
use css_parse::Box;

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
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.property"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Supports)]
pub struct SupportsRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Supports)]
	pub name: T![AtKeyword],
	pub prelude: SupportsCondition<'a>,
	#[metadata(block)]
	pub block: SupportsRuleBlock<'a>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SupportsRuleBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);

#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(#[atom(CssAtomSet::Not)] T![Ident], SupportsFeature<'a>),
	#[peek(skip)]
	And(Vec<'a, (SupportsFeature<'a>, Option<T![Ident]>)>),
	#[peek(skip)]
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
		Self::parse_condition(p)
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SupportsFeature<'a> {
	FontTech(
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		FontTech,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T![')'],
	),
	FontFormat(
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		FontFormat,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T![')'],
	),
	Selector(
		#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
		ComplexSelector<'a>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T![')'],
	),
	Property(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T!['('],
		Box<'a, Declaration<'a, StyleValue<'a>, CssMetadata>>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![')']>,
	),
	/// A parenthesized, nested `<supports-condition>`, e.g. the outer parens in
	/// `(selector(a) or selector(b))`. This is `<supports-in-parens> = ( <supports-condition> )`.
	Condition(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T!['('],
		Box<'a, SupportsCondition<'a>>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		T![')'],
	),
}

impl<'a> Peek<'a> for SupportsFeature<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Function]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T!['(']>::peek(p, c)
			|| (<T![Function]>::peek(p, c)
				&& matches!(
					p.to_atom::<CssAtomSet>(c),
					CssAtomSet::Selector | CssAtomSet::FontTech | CssAtomSet::FontFormat
				))
	}
}
impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if let Some(open) = p.parse_if_peek::<T!['(']>()? {
			let is_declaration = p.peek_n(1) == Kind::Ident && p.peek_n(2) == Kind::Colon;
			if is_declaration {
				let property = p.parse::<Declaration<'a, StyleValue<'a>, CssMetadata>>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				return Ok(Self::Property(open, Box::new_in(p.alloc(), property), close));
			}
			let condition = p.parse::<SupportsCondition>()?;
			let close = p.parse::<T![')']>()?;
			return Ok(Self::Condition(open, Box::new_in(p.alloc(), condition), close));
		}
		if p.peek::<T![Function]>() {
			let function = p.parse::<T![Function]>()?;
			return match p.to_atom::<CssAtomSet>(function.into()) {
				CssAtomSet::Selector => {
					let selector = p.parse::<ComplexSelector>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::Selector(function, selector, close))
				}
				CssAtomSet::FontTech => {
					let tech = p.parse::<FontTech>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::FontTech(function, tech, close))
				}
				CssAtomSet::FontFormat => {
					let format = p.parse::<FontFormat>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::FontFormat(function, format, close))
				}
				_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected_function))?,
			};
		}
		Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(color:black){}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width:1px){body{width:1px}}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not (width:1--foo){}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width: 1--foo) or (width: 1foo) {\n\n}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(width: 1--foo) and (width: 1foo) {\n\n}");
		assert_parse!(
			CssAtomSet::ATOMS,
			SupportsRule,
			"@supports(width: 100vw) {\n\tbody {\n\t\twidth: 100vw;\n\t}\n}"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			SupportsRule,
			"@supports not ((text-align-last: justify) or (-moz-text-align-last: justify)) {\n\n}"
		);
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports((position:-webkit-sticky)or (position:sticky)) {}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports selector(h2 > p) {\n\n}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(selector(h2 > p)) {}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not selector(h2 > p) {\n\n}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports not (selector(h2 > p)) {}");
		assert_parse!(
			CssAtomSet::ATOMS,
			SupportsRule,
			"@supports (selector(::-moz-meter-bar) or selector(::-webkit-meter-bar)) {\n\n}"
		);
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports font-tech(color-COLRv1) {\n\n}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(font-tech(variations)) {}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports font-format(woff2) {\n\n}");
		assert_parse!(CssAtomSet::ATOMS, SupportsRule, "@supports(font-format(\"woff2-variations\")) {}");
	}
}
