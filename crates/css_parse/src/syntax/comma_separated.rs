use crate::{
	Cursor, CursorSink, KindSet, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan,
	token_macros::Comma,
};
use bumpalo::{
	Bump,
	collections::{Vec, vec::IntoIter},
};
use std::{
	ops::{Index, IndexMut},
	slice::{Iter, IterMut},
};

/// This is a generic type that can be used for AST nodes representing multiple multiple items separated with commas.
///
/// This can be used for any grammar which defines a Comma Separated group (`[]#`).
///
/// The given `<T>` will be parsed first, followed by a comma. Parsing completes if the comma isn't found.
///
/// As `<T>` is parsed first, it can have any number of interior commas, however if T should ideally not consume
/// trailing commas, as doing so would likely mean only a single T in this struct.
///
/// The effective grammar for this struct is:
///
/// ```md
/// <comma-separated>
///  │├─╭─ <T> ─╮─ "," ─╭─┤│
///     │       ╰───────╯
///     ╰───────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CommaSeparated<'a, T> {
	items: Vec<'a, (T, Option<Comma>)>,
}

impl<'a, T> CommaSeparated<'a, T> {
	pub fn new_in(bump: &'a Bump) -> Self {
		Self { items: Vec::new_in(bump) }
	}
}

impl<'a, T> CommaSeparated<'a, T> {
	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for CommaSeparated<'a, T> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for CommaSeparated<'a, T> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut items = Self::new_in(p.bump());
		loop {
			let item = p.parse::<T>()?;
			let comma = p.parse_if_peek::<Comma>()?;
			items.items.push((item, comma));
			if comma.is_none() {
				return Ok(items);
			}
		}
	}
}

impl<'a, T: ToCursors> ToCursors for CommaSeparated<'a, T> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.items, s);
	}
}

impl<'a, T: ToSpan> ToSpan for CommaSeparated<'a, T> {
	fn to_span(&self) -> Span {
		let first = self.items[0].to_span();
		first + self.items.last().map(|t| t.to_span()).unwrap_or(first)
	}
}

impl<'a, T> IntoIterator for CommaSeparated<'a, T> {
	type Item = (T, Option<Comma>);
	type IntoIter = IntoIter<'a, Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

impl<'a, 'b, T> IntoIterator for &'b CommaSeparated<'a, T> {
	type Item = &'b (T, Option<Comma>);
	type IntoIter = Iter<'b, (T, Option<Comma>)>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter()
	}
}

impl<'a, 'b, T> IntoIterator for &'b mut CommaSeparated<'a, T> {
	type Item = &'b mut (T, Option<Comma>);
	type IntoIter = IterMut<'b, (T, Option<Comma>)>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.iter_mut()
	}
}

impl<'a, T, I> Index<I> for CommaSeparated<'a, T>
where
	I: ::core::slice::SliceIndex<[(T, Option<Comma>)]>,
{
	type Output = I::Output;

	#[inline]
	fn index(&self, index: I) -> &Self::Output {
		Index::index(&self.items, index)
	}
}

impl<'a, T, I> IndexMut<I> for CommaSeparated<'a, T>
where
	I: ::core::slice::SliceIndex<[(T, Option<Comma>)]>,
{
	#[inline]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		IndexMut::index_mut(&mut self.items, index)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{T, test_helpers::*};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CommaSeparated<T![Ident]>>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CommaSeparated<T![Ident]>, "foo");
		assert_parse!(CommaSeparated<T![Ident]>, "one,two");
		assert_parse!(CommaSeparated<T![Ident]>, "one,two,three");
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			CommaSeparated<T![Ident]>,
			r#"
			foo bar
			^^^
		"#
		);
		assert_parse_span!(
			CommaSeparated<T![Ident]>,
			r#"
			foo, bar, baz 1
			^^^^^^^^^^^^^
		"#
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CommaSeparated<T![Ident]>, ",");
		assert_parse_error!(CommaSeparated<T![Ident]>, "one,two,three,");
		assert_parse_error!(CommaSeparated<T![Ident]>, "one two");
	}
}
