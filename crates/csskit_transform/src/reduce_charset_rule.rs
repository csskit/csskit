use crate::prelude::*;
use css_ast::{AtRuleId, AtomSet, CharsetRule, EncodingLabel, NodeId, Rule, StyleSheet, VisitMut, VisitNode};
use css_lexer::Span;
use css_parse::format_in;

pub struct ReduceCharsetRule<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	kept: Option<Span>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceCharsetRule<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.used_at_rules.contains(AtRuleId::Charset)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, kept: None }
	}
}

impl<'a, 'ctx, N> ReduceCharsetRule<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn shorten_label(&self, rule: &mut CharsetRule) -> bool {
		let (encoding, is_shortest) = {
			let text = self.transformer.text();
			let encoding = rule.encoding(&text);
			(encoding, encoding.to_str() == rule.label(&text))
		};
		if encoding == EncodingLabel::Unknown {
			return false;
		}
		let shortest = encoding.to_str();
		if !is_shortest
			&& let Some(relabelled) = self
				.transformer
				.parse::<CharsetRule>(format_in!(in self.transformer.alloc(), "@charset \"{shortest}\";").into_str())
		{
			self.transformer.replace(rule, relabelled);
		}
		true
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for ReduceCharsetRule<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta)) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn retain(&mut self, node: VisitNode) -> bool {
		if node.node_id == Some(NodeId::CharsetRule) && self.kept != Some(node.span()) {
			self.transformer.mark_changed();
			return false;
		}
		true
	}

	fn visit_style_sheet(&mut self, sheet: &mut StyleSheet<'a>) {
		let first_valid = sheet.rules.iter_mut().enumerate().find_map(|(index, rule)| match rule {
			Rule::Charset(charset) => self.shorten_label(charset).then(|| (index, charset.to_span())),
			_ => None,
		});
		let Some((index, kept)) = first_valid else { return };
		self.kept = Some(kept);
		if index > 0 {
			sheet.rules[..=index].rotate_right(1);
			self.transformer.mark_changed();
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn removes_utf8_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"UTF-8\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn keeps_valid_charset_after_dropped_one() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"utf-8\";@charset \"gbk\";a{color:red}",
			"@charset \"gbk\";a{color:red}"
		);
	}

	#[test]
	fn removes_lowercase_utf8_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"utf-8\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn removes_duplicate_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"gbk\";@charset \"big5\";a{color:red}",
			"@charset \"gbk\";a{color:red}"
		);
	}

	#[test]
	fn keeps_shortest_leading_label() {
		assert_no_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"gbk\";a{color:red}"
		);
	}

	#[test]
	fn removes_uppercase_at_keyword() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@CHARSET \"gbk\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn removes_single_quoted_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset 'gbk';a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn removes_escaped_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"\\67 bk\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn shortens_label_to_shortest_alias() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"ISO-8859-1\";a{color:red}",
			"@charset \"l1\";a{color:red}"
		);
	}

	#[test]
	fn removes_utf16_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"utf-16be\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn removes_unknown_label_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"@charset \"not-a-charset\";a{color:red}",
			"a{color:red}"
		);
	}

	#[test]
	fn moves_trailing_charset_to_top() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"a{color:red}@charset \"gbk\";",
			"@charset \"gbk\";a{color:red}"
		);
	}

	#[test]
	fn moves_and_shortens_trailing_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"a{color:red}@charset \"ISO-8859-1\";",
			"@charset \"l1\";a{color:red}"
		);
	}

	#[test]
	fn moves_first_charset_to_top_and_drops_rest() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			".foo{color:red}@charset \"gbk\";.bar{color:tan}@charset \"big5\";.baz{background:red}",
			"@charset \"gbk\";.foo{color:red}.bar{color:tan}.baz{background:red}"
		);
	}

	#[test]
	fn removes_trailing_utf8_charset() {
		assert_transform!(
			CssMinifierFeature::ReduceCharsetRule,
			CssAtomSet,
			StyleSheet,
			"a{color:red}@charset \"UTF-8\";",
			"a{color:red}"
		);
	}
}
