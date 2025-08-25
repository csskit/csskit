use crate::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan, token_macros};
use std::marker::PhantomData;

/// This struct provides the generic `function()` grammar that parses a [function block][1] where the interior function
/// parameters are `<T>` and the function name is `<FT>`. The grammar is:
///
/// ```md
/// <function>
///  │├─ <FT> ─ <T> ─ ")" ─┤│
/// ```
///
/// `<AT>` must implement [Peek],  [Parse], and `Into<token_macros::Function>`. This helps enforce that this is a
/// function, that the first token has to be a specific function token.
///
/// `<T>` - the interior parameters - must implement [Parse], [ToCursors], and [ToSpan].
///
/// To specify extra restrictions on the value of the function, use [function_set][crate::function_set].
///
/// # Example
///
/// ```
/// use css_parse::*;
///
/// /// A grammar like `test(foo)`
/// #[derive(Debug)]
/// pub struct TestFunction(Function<T![Function], T![Ident]>);
///
/// impl<'a> Parse<'a> for TestFunction {
///     fn parse(p: &mut Parser<'a>) -> Result<Self> {
///         p.parse::<Function<T![Function], T![Ident]>>().map(Self)
///     }
/// }
///
/// impl ToCursors for TestFunction {
///     fn to_cursors(&self, s: &mut impl CursorSink) {
///         self.0.to_cursors(s);
///     }
/// }
///
/// assert_parse!(TestFunction, "test(foo)");
/// ```
///
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#function-block-diagram
/// [2]: https://drafts.csswg.org/css-syntax-3/#consume-function
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Function<FT, T>
where
	FT: Into<token_macros::Function>,
{
	pub name: token_macros::Function,
	pub parameters: T,
	pub close: Option<token_macros::RightParen>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: PhantomData<FT>,
}

impl<'a, FT, T> Peek<'a> for Function<FT, T>
where
	FT: Peek<'a> + Into<token_macros::Function>,
{
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<FT>::peek(p, c)
	}
}

impl<'a, FT, T> Parse<'a> for Function<FT, T>
where
	FT: Parse<'a> + Into<token_macros::Function>,
	T: Parse<'a>,
{
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let name = p.parse::<FT>()?.into();
		let parameters = p.parse::<T>()?;
		let close = p.parse_if_peek::<token_macros::RightParen>()?;
		Ok(Self { name, parameters, close, _phantom: PhantomData })
	}
}

impl<FT, T> ToCursors for Function<FT, T>
where
	FT: Into<token_macros::Function>,
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		ToCursors::to_cursors(&self.parameters, s);
		ToCursors::to_cursors(&self.close, s);
	}
}

impl<FT, T> ToSpan for Function<FT, T>
where
	FT: Into<token_macros::Function>,
	T: ToSpan,
{
	fn to_span(&self) -> Span {
		self.name.to_span() + self.parameters.to_span() + self.close.to_span()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{T, test_helpers::*};

	type FunctionBlock<'a> = Function<T![Function], T![Ident]>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionBlock>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FunctionBlock, "foo(bar)");
	}
}
