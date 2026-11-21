use crate::prelude::*;
use css_ast::{CssTypes, Url, UrlOrString, VisitMut, VisitNode};
use css_parse::format_in;

pub struct ReduceUrls<'a, 'ctx, N: NodeWithMetadata<CssMetadata>> {
	pub transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>,
}

impl<'a, 'ctx, N> Transform<'a, 'ctx, CssMetadata, N, CssMinifierFeature> for ReduceUrls<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn skips_subtree(metadata: &CssMetadata) -> bool {
		!metadata.has_value_kinds(CssTypes::Url)
	}

	fn new(transformer: &'ctx Transformer<'a, CssMetadata, N, CssMinifierFeature>) -> Self {
		Self { transformer }
	}
}

#[visitor]
impl<'a, 'ctx, N> VisitMut<'a> for ReduceUrls<'a, 'ctx, N>
where
	N: NodeWithMetadata<CssMetadata>,
{
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		if node.rule_subtree_metadata().is_some_and(|meta| Self::skips_subtree(&meta)) {
			return VisitFlow::SKIP_CHILDREN;
		}
		VisitFlow::DESCEND
	}

	fn visit_url_or_string(&mut self, url_or_string: &mut UrlOrString) {
		let UrlOrString::Url(url) = url_or_string else {
			return;
		};
		let arena = self.transformer.alloc();
		let quoted = match url {
			Url::UrlFunction(_, string, _) | Url::SrcFunction(_, string, _) => {
				let sc = self.transformer.to_source_cursor((*string).into());
				let token = sc.token();
				let start = token.leading_len() as usize;
				let end = sc.source().len() - token.trailing_len() as usize;
				format_in!(in arena, "\"{}\"", &sc.source()[start..end]).into_str()
			}
			Url::Url(url_token) => {
				let sc = self.transformer.to_source_cursor((*url_token).into());
				let token = sc.token();
				let leading_len = token.leading_len() as usize;
				let trailing_len = token.trailing_len() as usize;
				let url_content = &sc.source()[leading_len..(sc.source().len() - trailing_len)];
				let url_content = url_content.trim();
				format_in!(in arena, "\"{url_content}\"").into_str()
			}
		};
		if let Some(reduced) = self.transformer.parse::<UrlOrString>(quoted) {
			self.transformer.replace(url_or_string, reduced);
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::test_helpers::{assert_no_transform, assert_transform};
	use css_ast::{CssAtomSet, StyleSheet};

	#[test]
	fn reduces_url_function_to_string_in_import() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(\"foo.css\");",
			"@import \"foo.css\";"
		);
	}

	#[test]
	fn reduces_url_function_single_quotes_to_string_in_import() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url('foo.css');",
			"@import \"foo.css\";"
		);
	}

	#[test]
	fn reduces_bare_url_to_string_in_import() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(foo.css);",
			"@import \"foo.css\";"
		);
	}

	#[test]
	fn reduces_url_with_media_query() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(\"foo.css\") screen;",
			"@import \"foo.css\" screen;"
		);
	}

	#[test]
	fn reduces_url_with_layer() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(\"foo.css\") layer;",
			"@import \"foo.css\" layer;"
		);
	}

	#[test]
	fn reduces_url_with_supports() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(\"foo.css\") supports(not (display: flex));",
			"@import \"foo.css\" supports(not (display: flex));"
		);
	}

	#[test]
	fn reduces_url_with_layer_and_supports() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@import url(\"foo.css\") layer(base) supports(not (display: flex));",
			"@import \"foo.css\" layer(base) supports(not (display: flex));"
		);
	}

	#[test]
	fn does_not_transform_bare_string() {
		assert_no_transform!(CssMinifierFeature::ReduceUrls, CssAtomSet, StyleSheet, "@import \"foo.css\";");
	}

	#[test]
	fn reduces_namespace_url_to_string() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@namespace url(\"http://www.w3.org/1999/xhtml\");",
			"@namespace \"http://www.w3.org/1999/xhtml\";"
		);
	}

	#[test]
	fn reduces_namespace_url_with_prefix() {
		assert_transform!(
			CssMinifierFeature::ReduceUrls,
			CssAtomSet,
			StyleSheet,
			"@namespace html url(\"http://www.w3.org/1999/xhtml\");",
			"@namespace html \"http://www.w3.org/1999/xhtml\";"
		);
	}
}
