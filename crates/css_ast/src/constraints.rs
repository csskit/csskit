use crate::CssDiagnostic;
use css_parse::{Cursor, Diagnostic, KindSet, Parse, Parser, Peek, Result, ToNormalisedValue, ToNumberValue};
use csskit_derives::*;

/// A non-negative value wrapper.
///
/// This wrapper validates that literal values are >= 0 at parse time.
#[derive(Peek, IntoCursor, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct NonNegative<T>(pub T);

impl<'a, T: Parse<'a> + ToNumberValue> Parse<'a> for NonNegative<T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;
		if let Some(num) = value.to_number_value()
			&& num < 0.0
		{
			Err(Diagnostic::new(cursor, Diagnostic::non_negative))?;
		}

		Ok(Self(value))
	}
}

impl<T> NonNegative<T> {
	/// Returns a reference to the inner value.
	pub fn inner(&self) -> &T {
		&self.0
	}

	/// Consumes self and returns the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}

/// A positive value wrapper.
///
/// This wrapper validates that literal values are > 0 at parse time.
#[derive(IntoCursor, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct Positive<T>(pub T);

impl<'a, T: Parse<'a> + ToNumberValue> Parse<'a> for Positive<T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;

		if let Some(num) = value.to_number_value()
			&& num <= 0.0
		{
			Err(Diagnostic::new(cursor, Diagnostic::positive))?;
		}

		Ok(Self(value))
	}
}

impl<T> Positive<T> {
	/// Returns a reference to the inner value.
	pub fn inner(&self) -> &T {
		&self.0
	}

	/// Consumes self and returns the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}

/// A non-zero value wrapper.
///
/// This wrapper validates that literal values are != 0 at parse time.
#[derive(IntoCursor, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct NonZero<T>(pub T);

impl<'a, T: Parse<'a> + ToNumberValue> Parse<'a> for NonZero<T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;

		if let Some(num) = value.to_number_value()
			&& num == 0.0
		{
			Err(Diagnostic::new(cursor, <Diagnostic as CssDiagnostic>::unexpected_zero))?;
		}

		Ok(Self(value))
	}
}

impl<T> NonZero<T> {
	/// Returns a reference to the inner value.
	pub fn inner(&self) -> &T {
		&self.0
	}

	/// Consumes self and returns the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}

/// A range-constrained value wrapper using const generics.
///
/// This wrapper validates that literal values fall within [MIN, MAX] at parse time.
#[derive(ToSpan, IntoCursor, ToCursors, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct Ranged<T, const MIN: i32, const MAX: i32>(pub T);

impl<'a, T: Peek<'a>, const MIN: i32, const MAX: i32> Peek<'a> for Ranged<T, MIN, MAX> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !T::peek(p, c) {
			return false;
		}
		let kind = c.token().kind();
		if kind == css_lexer::Kind::Number || kind == css_lexer::Kind::Dimension {
			let num = c.token().value();
			num >= MIN as f32 && num <= MAX as f32
		} else {
			true
		}
	}
}

impl<'a, T: Parse<'a> + ToNormalisedValue, const MIN: i32, const MAX: i32> Parse<'a> for Ranged<T, MIN, MAX> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;

		if let Some(num) = value.to_normalised_value()
			&& (num < MIN as f32 || num > MAX as f32)
		{
			Err(Diagnostic::new(cursor, Diagnostic::number_out_of_bounds))?;
		}

		Ok(Self(value))
	}
}

impl<T: ToNumberValue, const MIN: i32, const MAX: i32> ToNumberValue for Ranged<T, MIN, MAX> {
	fn to_number_value(&self) -> Option<f32> {
		self.0.to_number_value()
	}
}

impl<T, const MIN: i32, const MAX: i32> Ranged<T, MIN, MAX> {
	/// Returns a reference to the inner value.
	pub fn inner(&self) -> &T {
		&self.0
	}

	/// Consumes self and returns the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}

/// An exact value wrapper using const generics.
///
/// This wrapper validates that literal values are exactly equal to the specified VALUE at parse time.
#[derive(IntoCursor, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct Exact<T, const VALUE: i32>(pub T);

impl<'a, T: Peek<'a>, const VALUE: i32> Peek<'a> for Exact<T, VALUE> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !T::peek(p, c) {
			return false;
		}
		let kind = c.token().kind();
		if kind == css_lexer::Kind::Number || kind == css_lexer::Kind::Dimension {
			c.token().value() == VALUE as f32
		} else {
			true
		}
	}
}

impl<'a, T: Parse<'a> + ToNumberValue, const VALUE: i32> Parse<'a> for Exact<T, VALUE> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;

		if let Some(num) = value.to_number_value()
			&& num != VALUE as f32
		{
			Err(Diagnostic::new(cursor, Diagnostic::number_out_of_bounds))?;
		}

		Ok(Self(value))
	}
}

impl<T: ToNumberValue, const VALUE: i32> ToNumberValue for Exact<T, VALUE> {
	fn to_number_value(&self) -> Option<f32> {
		self.0.to_number_value()
	}
}

impl<T, const VALUE: i32> Exact<T, VALUE> {
	/// Returns a reference to the inner value.
	pub fn inner(&self) -> &T {
		&self.0
	}

	/// Consumes self and returns the inner value.
	pub fn into_inner(self) -> T {
		self.0
	}
}

/// A non-empty collection wrapper.
///
/// Wraps any collection type that derefs to a slice (`Deref<Target = [_]>`)
/// and validates at parse time that the collection contains at least one item.
///
/// Works with [`bumpalo::collections::Vec`] and any other slice-backed type.
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(NodeWithMetadata)]
pub struct NonEmpty<T>(pub T);

impl<'a, T, Item> Parse<'a> for NonEmpty<T>
where
	T: Peek<'a> + Parse<'a> + std::ops::Deref<Target = [Item]>,
{
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;
		if value.is_empty() {
			Err(Diagnostic::new(cursor, <Diagnostic as CssDiagnostic>::empty_collection))?;
		}
		Ok(Self(value))
	}
}

impl<T> NonEmpty<T> {
	pub fn inner(&self) -> &T {
		&self.0
	}

	pub fn into_inner(self) -> T {
		self.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use bumpalo::collections::Vec;
	use css_parse::{T, assert_parse, assert_parse_error, assert_peek_false};

	type ExactOne = Exact<T![Number], 1>;
	type RangedZeroOne = Ranged<T![Number], 0, 1>;

	#[test]
	fn test_exact_accepts_correct_value() {
		assert_parse!(CssAtomSet::ATOMS, ExactOne, "1");
	}

	#[test]
	fn test_exact_rejects_wrong_value() {
		assert_peek_false!(CssAtomSet::ATOMS, ExactOne, "2");
	}

	#[test]
	fn test_ranged_accepts_within_range() {
		assert_parse!(CssAtomSet::ATOMS, RangedZeroOne, "0.5");
	}

	#[test]
	fn test_ranged_rejects_out_of_range() {
		assert_peek_false!(CssAtomSet::ATOMS, RangedZeroOne, "1.5");
	}

	#[test]
	fn test_non_negative_accepts_zero() {
		assert_parse!(CssAtomSet::ATOMS, NonNegative<T![Number]>, "0");
	}

	#[test]
	fn test_non_negative_rejects_negative() {
		assert_parse_error!(CssAtomSet::ATOMS, NonNegative<T![Number]>, "-1");
	}

	#[test]
	fn test_positive_accepts_positive() {
		assert_parse!(CssAtomSet::ATOMS, Positive<T![Number]>, "1");
	}

	#[test]
	fn test_positive_rejects_zero() {
		assert_parse_error!(CssAtomSet::ATOMS, Positive<T![Number]>, "0");
	}

	#[test]
	fn test_non_zero_accepts_positive() {
		assert_parse!(CssAtomSet::ATOMS, NonZero<T![Number]>, "1");
	}

	#[test]
	fn test_non_zero_accepts_negative() {
		assert_parse!(CssAtomSet::ATOMS, NonZero<T![Number]>, "-1");
	}

	#[test]
	fn test_non_zero_rejects_zero() {
		assert_parse_error!(CssAtomSet::ATOMS, NonZero<T![Number]>, "0");
	}

	#[test]
	fn test_non_empty_accepts_one() {
		assert_parse!(CssAtomSet::ATOMS, NonEmpty<Vec<T![Ident]>>, "foo");
	}

	#[test]
	fn test_non_empty_accepts_multiple() {
		assert_parse!(CssAtomSet::ATOMS, NonEmpty<Vec<T![Ident]>>, "foo bar");
	}

	#[test]
	fn test_non_empty_rejects_empty() {
		assert_peek_false!(CssAtomSet::ATOMS, NonEmpty<Vec<T![Ident]>>, "");
	}
}
