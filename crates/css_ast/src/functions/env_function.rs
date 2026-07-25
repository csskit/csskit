use super::prelude::*;

/// <https://drafts.csswg.org/css-env/#env-function>
///
/// ```text,ignore
/// env() = env( <custom-ident> <integer [0,∞]>*, <declaration-value>? )
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(uses_substitution, declaration_kinds = Computed)]
pub struct EnvFunction<'a, V> {
	#[semantic_eq(skip)]
	#[atom(CssAtomSet::Env)]
	pub function: Function,
	pub name: Ident,
	pub dimensions: Vec<'a, CSSInt>,
	#[semantic_eq(skip)]
	pub comma: Option<Comma>,
	pub fallback: Option<Box<'a, V>>,
	#[semantic_eq(skip)]
	pub close: RightParen,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CalcableValue, CssAtomSet, Length, Value};
	use css_parse::assert_parse;

	type EnvLength<'a> = EnvFunction<'a, Value<'a, Length>>;
	type CalcableEnvLength<'a> = EnvFunction<'a, CalcableValue<'a, Length>>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<EnvLength>(), 96);
		assert_eq!(std::mem::size_of::<CalcableEnvLength>(), 96);
	}

	#[test]
	fn test_env_function() {
		assert_parse!(CssAtomSet::ATOMS, EnvLength, "env(safe-area-inset-top)");
		assert_parse!(CssAtomSet::ATOMS, EnvLength, "env(safe-area-inset-top, 10px)");
	}

	#[test]
	fn test_calcable_env_function() {
		assert_parse!(CssAtomSet::ATOMS, CalcableEnvLength, "env(x, calc(1px + 2px))");
	}
}
