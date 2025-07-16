use css_ast::Url;
use css_parse::{AtRule, NoBlockAllowed, atkeyword_set};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtImportKeyword "import");

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ImportRule<'a>(AtRule<'a, AtImportKeyword, Url, NoBlockAllowed>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImportRule>(), 84);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ImportRule, r#"@import url("https://example.com");"#);
		assert_parse!(ImportRule, r#"@import url("./foo.css");"#);
	}
}
