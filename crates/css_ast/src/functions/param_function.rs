use css_parse::{ComponentValues, Function, T, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

function_set!(struct ParamFunctionName "param");

/// <https://drafts.csswg.org/css-link-params-1/#funcdef-param>
///
/// ```text,ignore
/// <param()> = param( <dashed-ident> , <declaration-value>? )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct ParamFunction<'a>(Function<'a, ParamFunctionName, (T![DashedIdent], T![,], Option<ComponentValues<'a>>)>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ParamFunction>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ParamFunction, "param(--foo,12px)");
		assert_parse!(ParamFunction, "param(--foo,var(--bar))");
	}
}
