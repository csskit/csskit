use crate::prelude::*;
use css_ast::{AtRuleId, AtomSet, CharsetRule, EncodingLabel, VisitNode, Visitable};
use css_parse::{SourceOffset, format_in};

pub struct ReduceCharsetRule<'a, 'ctx, N: Visitable + NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
	seen: bool,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceCharsetRule<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.used_at_rules.contains(AtRuleId::Charset)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer, seen: false }
	}
}

#[visitor]
impl<'a, 'ctx, N> Visit for ReduceCharsetRule<'a, 'ctx, N>
where
	N: Visitable + NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.node_id.is_some() && Self::skips_subtree(&node.subtree_metadata()) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn visit_charset_rule(&mut self, rule: &CharsetRule) {
		let span = rule.to_span();
		let source_text = self.transformer.source_text;
		let encoding = rule.encoding(source_text);
		if encoding == EncodingLabel::Unknown || self.seen {
			self.transformer.delete(span);
			return;
		}
		self.seen = true;
		let label = rule.label(source_text);
		let shortest = encoding.to_str();
		let at_top = span.start() == SourceOffset(0);
		if shortest == label {
			if !at_top {
				self.transformer.insert_before(SourceOffset(0), self.transformer.to_source_cursors(rule));
				self.transformer.delete(span);
			}
			return;
		}
		let arena = self.transformer.alloc();
		let css = format_in!(in arena, "@charset \"{shortest}\";").into_str();
		if at_top {
			self.transformer.replace_parsed::<CharsetRule>(span, css);
		} else {
			self.transformer.insert_before(SourceOffset(0), self.transformer.parse_value::<CharsetRule>(css));
			self.transformer.delete(span);
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
