use crate::CssDiagnostic;
#[cfg(feature = "visitable")]
use crate::visit::{Visit, VisitMut, Visitable, VisitableMut};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result, SemanticEq, ToCursors, ToNumberValue, ToSpan};
use csskit_derives::{ToCursors as DeriveToCursors, ToSpan as DeriveToSpan};

/// A non-negative value wrapper.
///
/// This wrapper validates that literal values are >= 0 at parse time.
#[derive(DeriveToCursors, DeriveToSpan, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct NonNegative<T>(pub T);

impl<T: Into<Cursor>> From<NonNegative<T>> for Cursor {
	fn from(value: NonNegative<T>) -> Self {
		value.0.into()
	}
}

#[cfg(feature = "visitable")]
impl<T: Visitable> Visitable for NonNegative<T> {
	fn accept<V: Visit>(&self, visitor: &mut V) {
		self.0.accept(visitor);
	}
}

#[cfg(feature = "visitable")]
impl<T: VisitableMut> VisitableMut for NonNegative<T> {
	fn accept_mut<V: VisitMut>(&mut self, visitor: &mut V) {
		self.0.accept_mut(visitor);
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for NonNegative<T> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

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

impl<T: SemanticEq> SemanticEq for NonNegative<T> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.0.semantic_eq(&other.0)
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
#[derive(DeriveToCursors, DeriveToSpan, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Positive<T>(pub T);

impl<T: Into<Cursor>> From<Positive<T>> for Cursor {
	fn from(value: Positive<T>) -> Self {
		value.0.into()
	}
}

#[cfg(feature = "visitable")]
impl<T: Visitable> Visitable for Positive<T> {
	fn accept<V: Visit>(&self, visitor: &mut V) {
		self.0.accept(visitor);
	}
}

#[cfg(feature = "visitable")]
impl<T: VisitableMut> VisitableMut for Positive<T> {
	fn accept_mut<V: VisitMut>(&mut self, visitor: &mut V) {
		self.0.accept_mut(visitor);
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for Positive<T> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

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

impl<T: SemanticEq> SemanticEq for Positive<T> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.0.semantic_eq(&other.0)
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
#[derive(DeriveToCursors, DeriveToSpan, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct NonZero<T>(pub T);

impl<T: Into<Cursor>> From<NonZero<T>> for Cursor {
	fn from(value: NonZero<T>) -> Self {
		value.0.into()
	}
}

#[cfg(feature = "visitable")]
impl<T: Visitable> Visitable for NonZero<T> {
	fn accept<V: Visit>(&self, visitor: &mut V) {
		self.0.accept(visitor);
	}
}

#[cfg(feature = "visitable")]
impl<T: VisitableMut> VisitableMut for NonZero<T> {
	fn accept_mut<V: VisitMut>(&mut self, visitor: &mut V) {
		self.0.accept_mut(visitor);
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for NonZero<T> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

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

impl<T: SemanticEq> SemanticEq for NonZero<T> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.0.semantic_eq(&other.0)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Ranged<T, const MIN: i32, const MAX: i32>(pub T);

impl<T: Into<Cursor>, const MIN: i32, const MAX: i32> From<Ranged<T, MIN, MAX>> for Cursor {
	fn from(value: Ranged<T, MIN, MAX>) -> Self {
		value.0.into()
	}
}

impl<T: ToCursors, const MIN: i32, const MAX: i32> ToCursors for Ranged<T, MIN, MAX> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		self.0.to_cursors(s);
	}
}

impl<T: ToSpan, const MIN: i32, const MAX: i32> ToSpan for Ranged<T, MIN, MAX> {
	fn to_span(&self) -> css_lexer::Span {
		self.0.to_span()
	}
}

#[cfg(feature = "visitable")]
impl<T: Visitable, const MIN: i32, const MAX: i32> Visitable for Ranged<T, MIN, MAX> {
	fn accept<V: Visit>(&self, visitor: &mut V) {
		self.0.accept(visitor);
	}
}

#[cfg(feature = "visitable")]
impl<T: VisitableMut, const MIN: i32, const MAX: i32> VisitableMut for Ranged<T, MIN, MAX> {
	fn accept_mut<V: VisitMut>(&mut self, visitor: &mut V) {
		self.0.accept_mut(visitor);
	}
}

impl<'a, T: Peek<'a>, const MIN: i32, const MAX: i32> Peek<'a> for Ranged<T, MIN, MAX> {
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

impl<'a, T: Parse<'a> + ToNumberValue, const MIN: i32, const MAX: i32> Parse<'a> for Ranged<T, MIN, MAX> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let cursor = p.peek_n(1);
		let value = p.parse::<T>()?;

		if let Some(num) = value.to_number_value()
			&& (num < MIN as f32 || num > MAX as f32)
		{
			Err(Diagnostic::new(cursor, Diagnostic::number_out_of_bounds))?;
		}

		Ok(Self(value))
	}
}

impl<T: SemanticEq, const MIN: i32, const MAX: i32> SemanticEq for Ranged<T, MIN, MAX> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.0.semantic_eq(&other.0)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct Exact<T, const VALUE: i32>(pub T);

impl<T: Into<Cursor>, const VALUE: i32> From<Exact<T, VALUE>> for Cursor {
	fn from(value: Exact<T, VALUE>) -> Self {
		value.0.into()
	}
}

impl<T: ToCursors, const VALUE: i32> ToCursors for Exact<T, VALUE> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		self.0.to_cursors(s);
	}
}

impl<T: ToSpan, const VALUE: i32> ToSpan for Exact<T, VALUE> {
	fn to_span(&self) -> css_lexer::Span {
		self.0.to_span()
	}
}

#[cfg(feature = "visitable")]
impl<T: Visitable, const VALUE: i32> Visitable for Exact<T, VALUE> {
	fn accept<V: Visit>(&self, visitor: &mut V) {
		self.0.accept(visitor);
	}
}

#[cfg(feature = "visitable")]
impl<T: VisitableMut, const VALUE: i32> VisitableMut for Exact<T, VALUE> {
	fn accept_mut<V: VisitMut>(&mut self, visitor: &mut V) {
		self.0.accept_mut(visitor);
	}
}

impl<'a, T: Peek<'a>, const VALUE: i32> Peek<'a> for Exact<T, VALUE> {
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

impl<T: SemanticEq, const VALUE: i32> SemanticEq for Exact<T, VALUE> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.0.semantic_eq(&other.0)
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{T, assert_parse, assert_parse_error};

	type ExactOne = Exact<T![Number], 1>;
	type RangedZeroOne = Ranged<T![Number], 0, 1>;

	#[test]
	fn test_exact_accepts_correct_value() {
		assert_parse!(CssAtomSet::ATOMS, ExactOne, "1");
	}

	#[test]
	fn test_exact_rejects_wrong_value() {
		assert_parse_error!(CssAtomSet::ATOMS, ExactOne, "2");
	}

	#[test]
	fn test_ranged_accepts_within_range() {
		assert_parse!(CssAtomSet::ATOMS, RangedZeroOne, "0.5");
	}

	#[test]
	fn test_ranged_rejects_out_of_range() {
		assert_parse_error!(CssAtomSet::ATOMS, RangedZeroOne, "1.5");
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
}
