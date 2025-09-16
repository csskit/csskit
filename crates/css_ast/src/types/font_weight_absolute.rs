use crate::diagnostics::CssDiagnostic;
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, Visitable};
use csskit_proc_macro::syntax;

/// <https://drafts.csswg.org/css-fonts-4/#font-weight-absolute-values>
///
/// ```text,ignore
/// <font-weight-absolute> = [normal | bold | <number [1,1000]>]
/// ```
#[syntax(" normal | bold | <number [1,1000]> ")]
#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
#[visit(self)]
pub enum FontWeightAbsolute {}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontWeightAbsolute>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontWeightAbsolute, "normal");
		assert_parse!(FontWeightAbsolute, "bold");
		assert_parse!(FontWeightAbsolute, "100");
		assert_parse!(FontWeightAbsolute, "500");
		assert_parse!(FontWeightAbsolute, "900");
		assert_parse!(FontWeightAbsolute, "900.5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FontWeightAbsolute, "1000.1");
	}
}
