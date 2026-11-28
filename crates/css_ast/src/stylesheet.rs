use crate::{CssAtomSet, CssMetadata, StyleValue, rules, stylerule::StyleRule};
use css_lexer::KindSet;
use css_parse::Vec;
use css_parse::{
	BadDeclaration, Box, ComponentValues, Cursor, Diagnostic, NodeWithMetadata, Parse, Parser, Peek, QualifiedRule,
	Result as ParserResult, RuleVariants, SimpleBlock, StyleSheet as StyleSheetTrait, T, UnknownRuleBlock,
};
use csskit_derives::*;
use csskit_proc_macro::node;

#[cfg(feature = "visitable")]
use crate::{
	PropertyKind,
	visit::{NodeId, QueryableNode},
};

/// <https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface>
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
	#[to_cursors(skip)]
	#[cfg_attr(feature = "serde", serde(skip))]
	#[cfg_attr(feature = "visitable", visit(skip))]
	meta: CssMetadata,
}

impl<'a> NodeWithMetadata<CssMetadata> for StyleSheet<'a> {
	fn metadata(&self) -> CssMetadata {
		self.meta
	}
}

impl<'a> Peek<'a> for StyleSheet<'a> {
	const PEEK_KINDSET: KindSet = KindSet::ANY;
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let (rules, meta) = Self::parse_stylesheet(p)?;
		Ok(Self { rules, meta })
	}
}

impl<'a> StyleSheetTrait<'a, CssMetadata> for StyleSheet<'a> {
	type Rule = Rule<'a>;
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Charset(CharsetRule): CssAtomSet::Charset,
			ColorProfile(ColorProfileRule<'a>): CssAtomSet::ColorProfile,
			Container(ContainerRule<'a>): CssAtomSet::Container,
			CounterStyle(CounterStyleRule<'a>): CssAtomSet::CounterStyle,
			CustomMedia(CustomMediaRule<'a>): CssAtomSet::CustomMedia,
			FontFace(FontFaceRule<'a>): CssAtomSet::FontFace,
			FontFeatureValues(FontFeatureValuesRule<'a>): CssAtomSet::FontFeatureValues,
			FontPaletteValues(FontPaletteValuesRule<'a>): CssAtomSet::FontPaletteValues,
			Keyframes(KeyframesRule<'a>): CssAtomSet::Keyframes,
			Layer(LayerRule<'a>): CssAtomSet::Layer,
			Media(MediaRule<'a>): CssAtomSet::Media,
			Namespace(NamespaceRule): CssAtomSet::Namespace,
			Page(PageRule<'a>): CssAtomSet::Page,
			Property(PropertyRule<'a>): CssAtomSet::Property,
			Scope(ScopeRule<'a>): CssAtomSet::Scope,
			StartingStyle(StartingStyleRule<'a>): CssAtomSet::StartingStyle,

			// Deprecated Rules
			Document(DocumentRule<'a>): CssAtomSet::Document,

			// Vendor Prefixed
			WebkitKeyframes(WebkitKeyframesRule<'a>): CssAtomSet::_WebkitKeyframes,

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocument(MozDocumentRule<'a>): CssAtomSet::_MozDocument,
		}
	};
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self), queryable(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Unknown | Effective, property_kinds = Name)]
pub struct UnknownAtRule<'a> {
	pub name: T![AtKeyword],
	pub prelude: UnknownAtRulePrelude<'a>,
	pub block: Option<SimpleBlock<'a>>,
	#[semantic_eq(skip)]
	pub semicolon: Option<T![;]>,
}

#[node]
#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(stop = KindSet::LEFT_CURLY_OR_SEMICOLON)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct UnknownAtRulePrelude<'a>(pub ComponentValues<'a>);

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for UnknownAtRule<'a> {
	const NODE_ID: NodeId = NodeId::UnknownAtRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.name.into()),
			_ => None,
		}
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Unknown | Effective)]
pub struct UnknownQualifiedRule<'a>(
	pub  QualifiedRule<
		'a,
		UnknownRuleBlock<'a>,
		StyleValue<'a>,
		UnknownRuleBlock<'a, StyleValue<'a>, CssMetadata>,
		CssMetadata,
	>,
);

macro_rules! rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		/// <https://drafts.csswg.org/cssom-1/#the-cssrule-interface>
		#[node]
		#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		#[derive(csskit_derives::NodeWithMetadata)]
		pub enum Rule<'a> {
			$(
				$name(Box<'a, rules::$ty$(<$a>)?>),
			)+
			Import(Box<'a, rules::ImportRule<'a>>),
			Supports(Box<'a, rules::SupportsRule<'a>>),

			UnknownAt(Box<'a, UnknownAtRule<'a>>),
			Style(Box<'a, StyleRule<'a>>),
			Unknown(Box<'a, UnknownQualifiedRule<'a>>),
			BadDeclaration(Box<'a, BadDeclaration<'a>>)
		}
	}
}

apply_rules!(rule);

impl<'a> RuleVariants<'a> for Rule<'a> {
	type DeclarationValue = StyleValue<'a>;
	type Metadata = CssMetadata;

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $atoms: pat,
			)+ ) => {
				match p.to_atom::<CssAtomSet>(c) {
					$($atoms => p.parse::<Box<rules::$ty>>().map(Self::$name),)+
					CssAtomSet::Import => p.parse::<rules::ImportRule>().map(|r| Self::Import(Box::new_in(p.alloc(), r))),
					CssAtomSet::Supports => p.parse::<rules::SupportsRule>().map(|r| Self::Supports(Box::new_in(p.alloc(), r))),
					_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
				}
			}
		}
		apply_rules!(parse_rule)
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<UnknownAtRule>>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<StyleRule>>().map(Self::Style)
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Box<UnknownQualifiedRule>>().map(Self::Unknown)
	}

	fn bad_declaration(declaration: Box<'a, BadDeclaration<'a>>) -> Option<Self> {
		Some(Self::BadDeclaration(declaration))
	}
}

impl<'a> Peek<'a> for Rule<'a> {
	const PEEK_KINDSET: KindSet = KindSet::ANY;
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_recovers};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{color:red;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "a{b{color:red};c{color:blue}}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{width:1px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{width:1px;}.a{width:2px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "one:1;a{two:2}");
		assert_parse!(CssAtomSet::ATOMS, Rule, "@media screen{}", Rule::Media(_));
		assert_parse!(CssAtomSet::ATOMS, Rule, "@layer foo{}", Rule::Layer(_));
	}

	#[test]
	fn cdc_in_unknown_rule_block() {
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "={-->}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "={-->", "={-->}");
	}

	#[test]
	fn stray_close_bracket_in_block() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "%{)", "%{)}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "t% {90)", "t% {90)}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "{--ty-2% {90)", "{--ty-2% {90)}}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "%{]", "%{]}");
	}

	#[test]
	fn bad_top_level_rule_keeps_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "+10", "+10");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "url(foo)", "url(foo)");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:red}+10", "a{color:red}+10");
	}

	#[test]
	fn stray_top_level_closer_keeps_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "}", "}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, ")", ")");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "]", "]");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{}}b{}", "a{}}b{}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "P0A}808G", "P0A}808G");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a b 1%);c:d;}e{}", "a b 1%);c:d;}e{}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{b:url(/x}c{d:e}", "a{b:url(/x}c{d:e}}");
	}

	#[test]
	fn top_level_bad_tokens_keep_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "url(e %);", "url(e %);");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "\"a\nb{}", "\"a\nb{}");
	}

	#[test]
	fn top_level_stray_semicolon_keeps_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, ";", ";");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{};b{}", "a{};b{}");
	}

	#[test]
	fn stray_tokens_in_block_keep_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:red;)}", "a{color:red;)}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{);color:red}", "a{);color:red}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{]color:red}", "a{]color:red}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "a{;;color:red}");
	}

	#[test]
	fn stray_tokens_in_unknown_rule_block_keep_tokens() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "(){;\n", "(){;\n}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "(){);a:b}", "(){);a:b}");
	}

	#[test]
	fn missing_close_curly_recovers() {
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:red", "a{color:red}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:red;", "a{color:red;}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{b{color:red", "a{b{color:red}}");
		assert_parse_recovers!(
			CssAtomSet::ATOMS,
			StyleSheet,
			"@media screen{a{color:red",
			"@media screen{a{color:red}}"
		);
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:rgb(1 2 3", "a{color:rgb(1 2 3)}");
	}

	#[test]
	fn missing_value_keeps_declaration() {
		use crate::{Rule, StyleValue, Value};
		use css_lexer::Lexer;
		use css_parse::{Arena, Parser, ToSpan};
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:", "a{color:}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:fnord}", "a{color:fnord}");
		assert_parse_recovers!(CssAtomSet::ATOMS, StyleSheet, "a{color:fnord;width:1px}", "a{color:fnord;width:1px}");

		let input = "a{color:";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let sheet = p.parse_entirely::<StyleSheet>().output.expect("sheet");
		let Rule::Style(rule) = &sheet.rules[0] else { panic!("expected style rule") };
		let decl = &rule.rule.block.declarations[0];
		let StyleValue::Color(color) = &decl.value else { panic!("expected color, got {:?}", decl.value) };
		assert!(matches!(color.0, Value::Bad(_)));
		assert_eq!(color.0.to_span(), css_lexer::Span::new(css_lexer::SourceOffset(8), css_lexer::SourceOffset(8)));
	}

	#[test]
	fn bad_value_stops_at_semicolon() {
		use crate::Rule;
		use css_lexer::Lexer;
		use css_parse::{Arena, Parser};
		let input = "a{color:fnord;width:1px}";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let sheet = p.parse_entirely::<StyleSheet>().output.expect("sheet");
		let Rule::Style(rule) = &sheet.rules[0] else { panic!("expected style rule") };
		assert_eq!(rule.rule.block.declarations.len(), 2);
	}

	#[test]
	fn unknown_at_rule_ends_at_semicolon_or_block() {
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "@foo bar;@media screen{}", |sheet| {
			assert!(matches!(sheet.rules[..], [Rule::UnknownAt(_), Rule::Media(_)]))
		});
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "@foo bar{baz}@media screen{}", |sheet| {
			assert!(matches!(sheet.rules[..], [Rule::UnknownAt(_), Rule::Media(_)]))
		});
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "a{@foo bar;color:red}", |sheet| {
			assert!(matches!(sheet.rules[..], [Rule::Style(_)]))
		});
	}
}
