use crate::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, token_macros};
use css_lexer::{Cursor, ToSpan};
use csskit_derives::ToSpan;
use std::marker::PhantomData;

/// This struct provides the generic [`<at-rule>` grammar][1]. It will [consume an at-rule][2]. This is defined as:
///
/// ```md
/// <at-rule>
///  │├─ <AT> ─ <P> ─ <B> ─╭───────╮┤│
///                        ╰─ ";" ─╯
/// ```
///
/// `<AT>` must implement [Peek],  [Parse], and `Into<token_macros::AtKeyword>`. This helps enforce that this is an
/// at-rule, that the first token has to be a specific AtKeyword.
///
/// `<P>` - the prelude - must implement [Parse], [ToCursors], and [ToSpan]. To make the prelude optional simply use an
/// [Option]. To enforce no prelude the [NoPreludeAllowed][crate::NoPreludeAllowed] type can be used. Non-optional
/// types are considered required.
///
/// `<B>` - the block - must implement [Parse], [ToCursors], and [ToSpan]. To make the block optional simply use an
/// [Option]. To enforce no block the [NoBlockAllowed][crate::NoBlockAllowed] type can be used. Non-optional types are
/// considered required. Ideally the block should implement one of [Block][crate::Block],
/// [DeclarationList][crate::DeclarationList], or [DeclarationRuleList][crate::DeclarationRuleList].
///
/// A generic AtRule could be `AtRule<'a, T![AtKeyword], ComponentValues<'a>, SimpleBlock>`.
///
/// To specify extra restrictions on the value of the at-keyword, use [atkeyword_set][crate::atkeyword_set].
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use css_lexer::*;
///
/// /// A grammar like `@test foo {}`
/// #[derive(Debug)]
/// pub struct TestAtRule<'a>(AtRule<'a, T![AtKeyword], T![Ident], SimpleBlock<'a>>);
///
/// impl<'a> Parse<'a> for TestAtRule<'a> {
///     fn parse(p: &mut Parser<'a>) -> Result<Self> {
///         Ok(Self(p.parse::<AtRule<'a, T![AtKeyword], T![Ident], SimpleBlock<'a>>>()?))
///     }
/// }
///
/// impl ToCursors for TestAtRule<'_> {
///     fn to_cursors(&self, s: &mut impl CursorSink) {
///         self.0.to_cursors(s);
///     }
/// }
///
/// assert_parse!(TestAtRule, "@test foo{}");
/// ```
///
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#at-rule-diagram
/// [2]: https://drafts.csswg.org/css-syntax-3/#consume-an-at-rule
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: Parse<'a> + ToCursors + ToSpan,
	B: Parse<'a> + ToCursors + ToSpan,
{
	pub name: token_macros::AtKeyword,
	pub prelude: P,
	pub block: B,
	pub semicolon: Option<token_macros::Semicolon>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: PhantomData<&'a AT>,
}

impl<'a, AT, P, B> Peek<'a> for AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: Parse<'a> + ToCursors + ToSpan,
	B: Parse<'a> + ToCursors + ToSpan,
{
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<AT>::peek(p, c)
	}
}

impl<'a, AT, P, B> Parse<'a> for AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: Parse<'a> + ToCursors + ToSpan,
	B: Parse<'a> + ToCursors + ToSpan,
{
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let name = p.parse::<AT>()?.into();
		let prelude = p.parse::<P>()?;
		let block = p.parse::<B>()?;
		let semicolon = p.parse_if_peek::<T![;]>()?;
		Ok(Self { name, prelude, block, semicolon, _phantom: PhantomData })
	}
}

impl<'a, AT, P, B> ToCursors for AtRule<'a, AT, P, B>
where
	AT: Peek<'a> + Parse<'a> + Into<token_macros::AtKeyword>,
	P: Parse<'a> + ToCursors + ToSpan,
	B: Parse<'a> + ToCursors + ToSpan,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.name, s);
		ToCursors::to_cursors(&self.prelude, s);
		ToCursors::to_cursors(&self.block, s);
		ToCursors::to_cursors(&self.semicolon, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{SimpleBlock, test_helpers::*};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AtRule<T![AtKeyword], T![Ident], T![Ident]>>(), 52);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AtRule < T![AtKeyword], Option<T![Ident]>, SimpleBlock>, "@foo{}");
		assert_parse!(AtRule < T![AtKeyword], Option<T![Ident]>, SimpleBlock>, "@foo prelude{}");
		assert_parse!(AtRule < T![AtKeyword], T![Ident], SimpleBlock>, "@foo prelude{}");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(AtRule < T![AtKeyword], T![Ident], SimpleBlock>, "@foo{}");
	}
}
