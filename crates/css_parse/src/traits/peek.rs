use crate::{Cursor, KindSet, Parser};

/// This trait allows AST nodes to indicate whether the [Parser] is in the right position to potentially
/// [Parse][crate::Parse] the node. Returning `true` from [Peek] is not a _guarantee_ that a node will successfully
/// parse, instead it offers an indication that the node can successfully parse the first node. This is useful for
/// cheaply comparing a set of Nodes to see which one might viably parse, rather than calling [Parser::try_parse()] on
/// each.
///
/// Nodes that implement this trait are entitled to peek any number of [Cursors][Cursor] ahead from the [Parser], to
/// determine if those [Cursors][Cursor] are viable to begin parsing, however there is a cost involved in peeking, so
/// it is worth being conservative; peek the minimum amount ahead to determine this. Most implementations can peek just
/// 1 [Cursor] ahead - this is provided as the second argument. To peek further, use the [Parser::peek_n()] method.
/// Calling `peek_n(2)` will return the [Cursor] after the provided one `peek_n(3)` will return the second [Cursor]
/// after, and so on.
///
/// For simple implementations it may be sufficient to just check the [Kind][crate::Kind] of the given [Cursor].
/// Rather than implementing [Peek::peek()], supplying [Peek::PEEK_KINDSET] and relying on the provided [Peek::peek()]
/// method will work well.
///
/// However it is likely that more complex checks will be needed. In order to reason about the given [Cursor] (or other
/// cursors ahead) an implementation might want to extract an Atom from the [Cursor] (using [Parser::to_atom]) and
/// compare it against an [AtomSet][crate::AtomSet].
///
/// When peeking child nodes, implementations should _not_ call [Peek::peek()] directly. Instead - call
/// [`Parser::peek<T>()`]. [`Parser::parse_if_peek<T>()`] also exists to conveniently parse a Node if it passes the peek
/// test.
///
/// If a Node can construct itself from a single [Cursor][Cursor] it should also implement
/// [Parse][crate::Parse].
pub trait Peek<'a>: Sized {
	const PEEK_KINDSET: KindSet = KindSet::ANY;

	fn peek<I>(_: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Self::PEEK_KINDSET
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for Option<T> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for ::bumpalo::collections::Vec<'a, T> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<'a, $($T),*> Peek<'a> for ($($T),*)
        where
            $($T: Peek<'a>,)*
        {
            fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
            where
                Iter: Iterator<Item = Cursor> + Clone,
            {
                A::peek(p, c)
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
