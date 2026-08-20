use super::prelude::*;
use crate::selector::ForgivingSelector;
use css_parse::Box;

/// Represents the `@scope` at-rule, e.g. `@scope (.card) to (.content) { p { color: red } }`.
///
/// ```md
/// <scope-rule>
///  │├─ "@scope" ─╮─────────────────────────────────────────╭─╮────────────────────────────────────────────────╭─ <block> ─┤│
///                ╰─ "(" ─ <forgiving-selector-list> ─ ")" ─╯ ╰─ "to" ─ "(" ─ <forgiving-selector-list> ─ ")" ─╯
/// ```
///
/// <https://drafts.csswg.org/css-cascade-6/#at-ruledef-scope>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.scope"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Scope)]
pub struct ScopeRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Scope)]
	pub name: T![AtKeyword],
	pub start: Option<Box<'a, ScopeStart<'a>>>,
	pub end: Option<Box<'a, ScopeEnd<'a>>>,
	pub block: ScopeRuleBlock<'a>,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScopeStart<'a>(
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub T!['('],
	pub Option<ForgivingSelector<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub T![')'],
);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScopeEnd<'a>(
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::To)]
	pub T![Ident],
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub T!['('],
	pub Option<ForgivingSelector<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub T![')'],
);

#[node]
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ScopeRuleBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope{}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(.card){}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(.card) to (.content){}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope to (.content){}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(.card,.tile) to (.content>img){}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(.card){color:black}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(.card){img{color:black}}");
		assert_parse!(CssAtomSet::ATOMS, ScopeRule, "@scope(){}");
	}
}
