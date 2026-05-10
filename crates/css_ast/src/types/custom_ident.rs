use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#custom-idents>
///
/// Wraps `T![Ident]`, but exists for the purposes of Visitable/VisitableMut.
/// Excludes CSS-wide keywords: `initial`, `inherit`, `unset`, `revert`, `revert-layer`, `default`.
#[derive(IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CustomIdent(T![Ident]);

impl<'a> Peek<'a> for CustomIdent {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !<T![Ident]>::peek(p, c) {
			return false;
		}
		!matches!(
			p.to_atom::<CssAtomSet>(c),
			CssAtomSet::Initial
				| CssAtomSet::Inherit
				| CssAtomSet::Unset
				| CssAtomSet::Revert
				| CssAtomSet::RevertLayer
				| CssAtomSet::Default
		)
	}
}

impl<'a> Parse<'a> for CustomIdent {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if !Self::peek(p, c) {
			return Err(Diagnostic::new(c, Diagnostic::unexpected_ident));
		}
		Ok(Self(p.parse::<T![Ident]>()?))
	}
}
