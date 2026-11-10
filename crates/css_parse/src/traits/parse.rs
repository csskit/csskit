use super::prelude::*;

/// This trait allows AST nodes to construct themselves from a mutable [Parser] instance.
///
/// Nodes that implement this trait are entitled to consume any number of [Cursors][crate::Cursor] from [Parser] in
/// order to construct themselves. They may also consume some amount of tokens and still return an [Err] - there is no
/// need to try and reset the [Parser] state on failure ([Parser::try_parse()] exists for this reason).
///
/// When wanting to parse child nodes, implementations should _not_ call [Parse::parse()] directly. Instead - call
/// [`Parser::parse<T>()`]. Other convenience methods such as [`Parser::parse_if_peek<T>()`] and [`Parser::try_parse<T>()`]
/// exist.
///
/// Any node implementing [Parse::parse()] gets [Parse::try_parse()] for free. It's unlikely that nodes can come up with
/// a more efficient algorithm than the provided one, so it is not worth re-implementing [Parse::try_parse()].
/// [Parse::try_parse()] only succeeds on a clean parse: if the node recovered from an error and recorded a diagnostic
/// (see [Parser::diagnose()]), the parser rewinds and that diagnostic is returned as the [Err].
///
/// If a Node can construct itself from a single [Cursor][crate::Cursor] it should implement
/// [Peek][crate::Peek] and [Parse], where [Parse::parse()] calls [Parser::next()] and constructs from the cursor.
pub trait Parse<'a>: Sized {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone;

	fn try_parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let checkpoint = p.checkpoint();
		Self::parse(p)
			.and_then(|value| match p.diagnostic_since(&checkpoint) {
				None => Ok(value),
				Some(diagnostic) => Err(diagnostic),
			})
			.inspect_err(|_| p.rewind(checkpoint))
	}
}

/// A node that always constructs itself from the [Parser], recovering from bad input instead of returning an [Err].
///
/// Implementors keep the tokens they could not parse (for example in a `Bad` variant) and record the failure with
/// [Parser::diagnose()]. Every [ParseInfallible] node is also [Parse], so it can be used anywhere a fallible node can.
pub trait ParseInfallible<'a>: Sized {
	fn parse_infallible<I>(p: &mut Parser<'a, I>) -> Self
	where
		I: Iterator<Item = Cursor> + Clone;
}

impl<'a, T: ParseInfallible<'a>> Parse<'a> for T {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(T::parse_infallible(p))
	}
}

impl<'a, T> Parse<'a> for Option<T>
where
	T: Peek<'a> + Parse<'a>,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		p.parse_if_peek::<T>()
	}
}

impl<'a, T> Parse<'a> for Vec<'a, T>
where
	T: Peek<'a> + Parse<'a>,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		let mut vec = Vec::new_in(p.alloc());
		while let Some(item) = p.parse_if_peek::<T>()? {
			vec.push(item);
		}
		Ok(vec)
	}
}

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<'a, $($T),*> Parse<'a> for ($($T),*)
        where
            $($T: Parse<'a>),*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
            fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
            where
                Iter: Iterator<Item = Cursor> + Clone,
            {
                $(let $T = p.parse::<$T>()?;)*
                Ok(($($T),*))
            }
        }
    };
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
