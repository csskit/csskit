use crate::{
	Cursor, CursorSink, DimensionUnit, Kind, KindSet, Parse, Parser, Peek, Result, Span, ToNumberValue, Token,
	diagnostics,
};

macro_rules! cursor_wrapped {
	($ident:ident) => {
		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl CursorSink) {
				s.append((*self).into());
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> Span {
				self.0.to_span()
			}
		}
	};
}

macro_rules! define_kinds {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::Cursor);

		impl $ident {
			pub const fn dummy() -> Self {
				Self($crate::Cursor::dummy($crate::Token::dummy($crate::Kind::$ident)))
			}

			pub fn associated_whitespace(&self) -> $crate::AssociatedWhitespaceRules {
				self.0.token().associated_whitespace()
			}

			pub fn with_associated_whitespace(&self, rules: $crate::AssociatedWhitespaceRules) -> Self {
				Self(self.0.with_associated_whitespace(rules))
			}
		}

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::$ident
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					Ok(Self(p.next()))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}
		)*
	};
}

macro_rules! define_kind_idents {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::Cursor);

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::$ident
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					Ok(Self(p.next()))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$ident> for $crate::Kind {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl $ident {
			/// Checks if the ident begins with two HYPHEN MINUS (`--`) characters.
			pub fn is_dashed_ident(&self) -> bool {
				self.0.token().is_dashed_ident()
			}

			pub const fn dummy() -> Self {
				Self($crate::Cursor::dummy($crate::Token::dummy($crate::Kind::$ident)))
			}
		}
		)*
	};
}

/// A macro for defining a struct which captures a [Kind::Delim][Kind::Delim] with a specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// custom_delim!{
///   /// A £ character.
///   PoundSterling, '£'
/// }
///
/// assert_parse!(PoundSterling, "£");
/// ```
#[macro_export]
macro_rules! custom_delim {
	($(#[$meta:meta])* $ident:ident, $ch:literal) => {
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::T![Delim]);

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::Delim && c == $ch
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					let delim = p.parse::<$crate::T![Delim]>()?;
					Ok(Self(delim))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl PartialEq<char> for $ident {
			fn eq(&self, other: &char) -> bool {
				self.0 == *other
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! custom_dimension {
	($(#[$meta:meta])*$ident: ident, $str: tt) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::Cursor);

		impl $ident {
			/// Returns the [f32] representation of the dimension's value.
			pub fn value(&self) -> f32 {
				self.0.token().value()
			}

			pub const fn dummy() -> Self {
				Self($crate::Cursor::dummy($crate::Token::dummy($crate::Kind::Dimension)))
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl PartialEq<f32> for $ident {
			fn eq(&self, other: &f32) -> bool {
				self.value() == *other
			}
		}

		impl $crate::ToNumberValue for $ident {
			fn to_number_value(&self) -> Option<f32> {
				Some(self.value())
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::Dimension
					&& (c == $crate::DimensionUnit::$ident || p.eq_ignore_ascii_case(c, $str))
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					let c = p.next();
					Ok(Self(c))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$ident> for i32 {
			fn from(value: $ident) -> Self {
				value.value() as i32
			}
		}

		impl From<$ident> for f32 {
			fn from(value: $ident) -> Self {
				value.value()
			}
		}
	};
}

/// A macro for defining a struct which captures two adjacent [Kind::Delim][Kind::Delim] tokens, each with a
/// specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// custom_double_delim!{
///   /// Two % adjacent symbols
///   DoublePercent, '%', '%'
/// }
///
/// assert_parse!(DoublePercent, "%%");
/// ```
#[macro_export]
macro_rules! custom_double_delim {
	($(#[$meta:meta])*$ident: ident, $first: literal, $second: literal) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(pub $crate::T![Delim], pub $crate::T![Delim]);

		impl $ident {
			pub const fn dummy() -> Self {
				Self(<$crate::T![Delim]>::dummy(), <$crate::T![Delim]>::dummy())
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $first && p.peek_n(2) == $second
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let first = p.parse::<$crate::T![Delim]>()?;
				if first != $first {
					let c: Cursor = first.into();
					Err($crate::diagnostics::ExpectedDelim(c))?;
				}
				let skip = p.set_skip(KindSet::NONE);
				let second = p.parse::<$crate::T![Delim]>();
				p.set_skip(skip);
				let second = second?;
				if second != $second {
					let c:Cursor = second.into();
					Err($crate::diagnostics::ExpectedDelim(c))?;
				}
				Ok(Self(first, second))
			}
		}

		impl<'a> $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append(self.0.into());
				s.append(self.1.into());
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span() + self.1.to_span()
			}
		}
	};
}

/// A macro for defining an enum which captures a token with [Kind::Ident][Kind::Ident] that matches one of
/// the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// keyword_set!(
///   /// Some docs on this type...
///   pub enum Keywords {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Keywords, "FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Keywords, "baR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Keywords, "bing");
///
/// assert_parse_error!(Keywords, "oof");
/// ```
#[macro_export]
macro_rules! keyword_set {
	($(#[$meta:meta])* $vis:vis enum $name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis enum $name {
			$($variant($crate::token_macros::Ident)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::Ident && Self::MAP.get(&p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let ident = p.parse::<$crate::token_macros::Ident>()?;
				let val = Self::MAP.get(&p.parse_str_lower(ident.into()));
				Ok(match val {
					$(Some(Self::$variant(_)) => Self::$variant(ident),)+
					None => Err($crate::diagnostics::Unexpected(ident.into()))?
				})
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant($crate::token_macros::Ident::dummy())),+
			};

			pub fn from_cursor(p: &$crate::Parser<'_>, c: $crate::Cursor) -> Option<Self> {
				if c != $crate::Kind::Ident {
					return None;
				}
				Self::MAP.get(&p.parse_str_lower(c.into())).copied()
			}
		}

		impl From<$name> for $crate::Kind {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<$name> for $crate::token_macros::Ident {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				match self {
					$($name::$variant(t) => (t.to_span()),)+
				}
			}
		}
	};

	($(#[$meta:meta])* $vis:vis struct $name: ident $str: tt) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis struct $name($crate::T![Ident]);

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				<$crate::T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, $str)
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					let ident = p.parse::<$crate::T![Ident]>()?;
					Ok(Self(ident))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl<'a> From<$name> for $crate::token_macros::Ident {
			fn from(value: $name) -> Self {
				value.0
			}
		}
	};
}

/// A macro for defining an enum which captures a token with [Kind::Function][Kind::Function] that matches
/// one of the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// function_set!(
///   /// Some docs on this type...
///   pub enum Functions {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Functions, "FoO(");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Functions, "baR(");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Functions, "bing(");
///
/// assert_parse_error!(Functions, "oof(");
/// ```
#[macro_export]
macro_rules! function_set {
	($(#[$meta:meta])* $vis:vis enum $name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis enum $name {
			$($variant($crate::token_macros::Function)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::Function && Self::MAP.get(p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let function = p.parse::<$crate::token_macros::Function>()?;
				let val = Self::MAP.get(&p.parse_str_lower(function.into()));
				Ok(match val {
					$(Some(Self::$variant(_)) => Self::$variant(function),)+
					None => Err($crate::diagnostics::Unexpected(function.into()))?,
				})
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
				$($variant_str => $name::$variant($crate::token_macros::Function::dummy())),+
			};

			pub fn from_cursor(p: &$crate::Parser<'_>, c: $crate::Cursor) -> Option<Self> {
				if c != $crate::Kind::Function {
					return None;
				}
				Self::MAP.get(&p.parse_str_lower(c.into())).copied()
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				match self {
					$($name::$variant(t) => (t.to_span()),)+
				}
			}
		}

		impl<'a> From<$name> for $crate::token_macros::Function {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}
	};

	($(#[$meta:meta])* $vis:vis struct $name: ident $str: tt) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis struct $name($crate::T![Function]);

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				<$crate::T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, $str)
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					let function = p.parse::<$crate::T![Function]>()?;
					Ok(Self(function))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl<'a> From<$name> for $crate::token_macros::Function {
			fn from(value: $name) -> Self {
				value.0
			}
		}
	};
}

/// A macro for defining an enum which captures a token with [Kind::AtKeyword][Kind::AtKeyword] that matches one of
/// the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// atkeyword_set!(
///   /// Some docs on this type...
///   pub enum Keywords {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Keywords, "@FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Keywords, "@baR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Keywords, "@bing");
///
/// assert_parse_error!(Keywords, "@oof");
/// ```
#[macro_export]
macro_rules! atkeyword_set {
	($(#[$meta:meta])* $vis:vis enum $name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis enum $name {
			$($variant($crate::token_macros::AtKeyword)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				c == $crate::Kind::AtKeyword && Self::MAP.get(&p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let at_keyword = p.parse::<$crate::token_macros::AtKeyword>()?;
				let val = Self::MAP.get(&p.parse_str_lower(at_keyword.into()));
				Ok(match val {
					$(Some(Self::$variant(_)) => Self::$variant(at_keyword),)+
					None => Err($crate::diagnostics::Unexpected(at_keyword.into()))?,
				})
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant($crate::token_macros::AtKeyword::dummy())),+
			};

			pub fn from_cursor(p: &$crate::Parser<'_>, c: $crate::Cursor) -> Option<Self> {
				if c != $crate::Kind::AtKeyword {
					return None;
				}
				Self::MAP.get(&p.parse_str_lower(c.into())).copied()
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				match self {
					$($name::$variant(t) => (t.to_span()),)+
				}
			}
		}

		impl<'a> From<$name> for $crate::token_macros::AtKeyword {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}
	};
	($(#[$meta:meta])* $vis:vis struct $name: ident $str: tt) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		$vis struct $name($crate::T![AtKeyword]);

		impl $crate::ToCursors for $name {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				<$crate::T![AtKeyword]>::peek(p, c) && p.eq_ignore_ascii_case(c, $str)
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				if p.peek::<Self>() {
					let at_keyword = p.parse::<$crate::T![AtKeyword]>()?;
					Ok(Self(at_keyword))
				} else {
					Err($crate::diagnostics::Unexpected(p.next()))?
				}
			}
		}

		impl From<$name> for $crate::Cursor {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl From<$name> for $crate::Token {
			fn from(value: $name) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $name {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl<'a> From<$name> for $crate::token_macros::AtKeyword {
			fn from(value: $name) -> Self {
				value.0
			}
		}
	};
}

define_kinds! {
	/// Represents a token with [Kind::Eof][Kind::Eof]. Use [T![Eof]][crate::T] to refer to this.
	Eof,

	/// Represents a token with [Kind::Comment][Kind::Comment]. Use [T![Comment]][crate::T] to refer to this.
	Comment,

	/// Represents a token with [Kind::CdcOrCdo][Kind::CdcOrCdo]. Use [T![CdcOrCdo]][crate::T] to refer to this.
	CdcOrCdo,

	/// Represents a token with [Kind::BadString][Kind::BadString]. Use [T![BadString]][crate::T] to refer to this.
	BadString,

	/// Represents a token with [Kind::BadUrl][Kind::BadUrl]. Use [T![BadUrl]][crate::T] to refer to this.[
	BadUrl,

	/// Represents a token with [Kind::Delim][Kind::Delim], can be any single character. Use [T![Delim]][crate::T] to refer to this.
	Delim,

	/// Represents a token with [Kind::Colon][Kind::Colon] - a `:` character. Use [T![:]][crate::T] to refer to this.
	Colon,

	/// Represents a token with [Kind::Semicolon][Kind::Semicolon] - a `;` character. Use [T![;]][crate::T] to refer to this.
	Semicolon,

	/// Represents a token with [Kind::Comma][Kind::Comma] - a `,` character. Use [T![,]][crate::T] to refer to this.
	Comma,

	/// Represents a token with [Kind::LeftCurly][Kind::LeftCurly] - a `{` character. Use [T!['{']][crate::T] to refer to this.
	LeftCurly,

	/// Represents a token with [Kind::LeftCurly][Kind::LeftCurly] - a `}` character. Use [T!['}']][crate::T] to refer to this.
	RightCurly,

	/// Represents a token with [Kind::LeftSquare][Kind::LeftSquare] - a `[` character. Use [T!['[']][crate::T] to refer to this.
	LeftSquare,

	/// Represents a token with [Kind::RightSquare][Kind::RightSquare] - a `]` character. Use [T![']']][crate::T] to refer to this.
	RightSquare,

	/// Represents a token with [Kind::LeftParen][Kind::LeftParen] - a `(` character. Use [T!['(']][crate::T] to refer to this.
	LeftParen,

	/// Represents a token with [Kind::RightParen][Kind::RightParen] - a `(` character. Use [T![')']][crate::T] to refer to this.
	RightParen,
}

impl PartialEq<char> for Delim {
	fn eq(&self, other: &char) -> bool {
		self.0 == *other
	}
}

define_kind_idents! {
	/// Represents a token with [Kind::Ident][Kind::Ident]. Use [T![Ident]][crate::T] to refer to this.
	Ident,

	/// Represents a token with [Kind::String][Kind::String]. Use [T![String]][crate::T] to refer to this.
	String,

	/// Represents a token with [Kind::Url][Kind::Url]. Use [T![Url]][crate::T] to refer to this.
	Url,

	/// Represents a token with [Kind::Function][Kind::Function]. Use [T![Function]][crate::T] to refer to this.
	Function,

	/// Represents a token with [Kind::AtKeyword][Kind::AtKeyword]. Use [T![AtKeyword]][crate::T] to refer to this.
	AtKeyword,

	/// Represents a token with [Kind::Hash][Kind::Hash]. Use [T![Hash]][crate::T] to refer to this.
	Hash,
}

/// Represents a token with [Kind::Whitespace]. Use [T![Whitespace]][crate::T] to refer to
/// this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Whitespace(pub Cursor);
cursor_wrapped!(Whitespace);

impl<'a> Peek<'a> for Whitespace {
	fn peek(p: &Parser<'a>, _: Cursor) -> bool {
		// Whitespace needs to peek its own cursor because it was likely given one that skipped Whitespace.
		let c = p.peek_next_including_whitespace();
		c == Kind::Whitespace
	}
}

impl<'a> Parse<'a> for Whitespace {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		// Whitespace needs to implement parse so that it can change the skip-state to only ensuring Whitespace
		// is not ignored.
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.next();
		p.set_skip(skip);
		if c != Kind::Whitespace {
			Err(diagnostics::Unexpected(c))?
		}
		Ok(Self(c))
	}
}

/// Represents a token with [Kind::Ident] that also begins with two HYPHEN MINUS (`--`)
/// characters. Use [T![DashedIdent]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DashedIdent(Ident);
cursor_wrapped!(DashedIdent);

impl<'a> Peek<'a> for DashedIdent {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Ident && c.token().is_dashed_ident()
	}
}

impl<'a> Parse<'a> for DashedIdent {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(Ident(c)))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

/// Represents a token with [Kind::Dimension]. Use [T![Dimension]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Dimension(pub Cursor);
cursor_wrapped!(Dimension);

impl PartialEq<f32> for Dimension {
	fn eq(&self, other: &f32) -> bool {
		self.0.token().value() == *other
	}
}

impl<'a> Peek<'a> for Dimension {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Dimension
	}
}

impl<'a> Parse<'a> for Dimension {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(c))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

impl From<Dimension> for f32 {
	fn from(val: Dimension) -> Self {
		val.0.token().value()
	}
}

impl ToNumberValue for Dimension {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.0.token().value())
	}
}

impl From<Dimension> for (f32, DimensionUnit) {
	fn from(val: Dimension) -> Self {
		let value = val.0.token().value();
		let unit = val.0.token().dimension_unit();
		(value, unit)
	}
}

impl Dimension {
	/// Returns the [f32] representation of the dimension's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}

	/// Returns the [DimensionUnit].
	///
	/// If the dimension unit is custom (e.g. dashed), has escape characters, or is not a recognised CSS Dimension, this
	/// will return [DimensionUnit::Unknown].
	pub fn dimension_unit(&self) -> DimensionUnit {
		self.0.token().dimension_unit()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DimensionIdent(Cursor, DimensionUnit);
cursor_wrapped!(DimensionIdent);

impl<'a> Peek<'a> for DimensionIdent {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Ident::peek(p, c) && (DimensionUnit::from(p.parse_str_lower(c)) != DimensionUnit::Unknown)
	}
}

impl<'a> Parse<'a> for DimensionIdent {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(c, DimensionUnit::from(p.parse_str_lower(c))))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

/// Represents a token with [Kind::Number]. Use [T![Number]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Number(pub Cursor);
cursor_wrapped!(Number);

impl Number {
	pub const NUMBER_ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));
	pub const ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));

	/// Returns the [f32] representation of the number's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}

	pub fn is_int(&self) -> bool {
		self.0.token().is_int()
	}

	pub fn is_float(&self) -> bool {
		self.0.token().is_float()
	}

	pub fn has_sign(&self) -> bool {
		self.0.token().has_sign()
	}
}

impl<'a> Peek<'a> for Number {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Number
	}
}

impl<'a> Parse<'a> for Number {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(c))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

impl From<Number> for f32 {
	fn from(value: Number) -> Self {
		value.value()
	}
}

impl From<Number> for i32 {
	fn from(value: Number) -> Self {
		value.value() as i32
	}
}

impl PartialEq<f32> for Number {
	fn eq(&self, other: &f32) -> bool {
		self.value() == *other
	}
}

impl ToNumberValue for Number {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.value())
	}
}

/// Various [T!s][crate::T] representing a tokens with [Kind::Delim], but each represents a discrete character.
pub mod delim {
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `&`. Use [T![&]][crate::T] to
		/// refer to this.
		And, '&'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `@`. Use [T![@]][crate::T] to
		/// refer to this. Not to be conused with [T![AtKeyword]][crate::T] which represents a token with
		/// [Kind::AtKeyword][crate::Kind::AtKeyword].
		At, '@'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `^`. Use [T![^]][crate::T] to
		/// refer to this.
		Caret, '^'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `-`. Use [T![-]][crate::T] to
		/// refer to this.
		Dash, '-'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `$`. Use [T![$]][crate::T] to
		/// refer to this.
		Dollar, '$'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `.`. Use [T![.]][crate::T] to
		/// refer to this.
		Dot, '.'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `=`. Use [T![=]][crate::T] to
		/// refer to this.
		Eq, '='
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `>`. Use [T![>]][crate::T] to
		/// refer to this.
		Gt, '>'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `#`. Use [T![#]][crate::T] to
		/// refer to this. Not to be conused with [T![Hash]][crate::T] which represents a token with
		/// [Kind::Hash][crate::Kind::Hash].
		Hash, '#'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `<`. Use [T![<]][crate::T] to
		/// refer to this.
		Lt, '<'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `!`. Use [T![!]][crate::T] to
		/// refer to this.
		Bang, '!'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `|`. Use [T![|]][crate::T] to
		/// refer to this.
		Or, '|'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `%`. Use [T![%]][crate::T] to
		/// refer to this.
		Percent, '%'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `+`. Use [T![+]][crate::T] to
		/// refer to this.
		Plus, '+'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `?`. Use [T![?]][crate::T] to
		/// refer to this.
		Question, '?'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `/`. Use [T![/]][crate::T] to
		/// refer to this.
		Slash, '/'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `*`. Use [T![*]][crate::T] to
		/// refer to this.
		Star, '*'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `~`. Use [T![~]][crate::T] to
		/// refer to this.
		Tilde, '~'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `_`. Use [T![_]][crate::T] to
		/// refer to this.
		Underscore, '_'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char ```. Use [T!['`']][crate::T] to
		/// refer to this.
		Backtick, '`'
	}
}

/// Various [T!s][crate::T] representing two consecutive tokens that cannot be separated by any other tokens. These are
/// convenient as it can be tricky to parse two consecutive tokens given the default behaviour of the parser is to skip
/// whitespace and comments.
pub mod double {
	use crate::{Cursor, CursorSink, Kind, KindSet, Parse, Parser, Peek, Result, Span, T, ToCursors, ToSpan};

	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `>` while the second has the char `=`, representing `>=`. Use
		/// [T![>=]][crate::T] to refer to this.
		GreaterThanEqual, '>', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `<` while the second has the char `=`, representing `<=`. Use
		/// [T![<=]][crate::T] to refer to this.
		LessThanEqual, '<', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `|`, representing `*|`. Use
		/// [T![*|]][crate::T] to refer to this.
		StarPipe, '*', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `|`, representing `||`. Use
		/// [T![||]][crate::T] to refer to this.
		PipePipe, '|', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `=` while the second has the char `=`, representing `==`. Use
		/// [T![==]][crate::T] to refer to this.
		EqualEqual, '=', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `~` while the second has the char `=`, representing `~=`. Use
		/// [T![~=]][crate::T] to refer to this.
		TildeEqual, '~', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `=`, representing `|=`. Use
		/// [T![|=]][crate::T] to refer to this.
		PipeEqual, '|', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `^` while the second has the char `=`, representing `^=`. Use
		/// [T![\^=]][crate::T] to refer to this.
		CaretEqual, '^', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `$` while the second has the char `=`, representing `$=`. Use
		/// [T![$=]][crate::T] to refer to this.
		DollarEqual, '$', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `=`, representing `*=`. Use
		/// [T![*=]][crate::T] to refer to this.
		StarEqual, '*', '='
	}

	/// Represents a two consecutive tokens with [Kind::Colon] that cannot be separated by any other token, representing
	/// `::`. Use [T![::]][crate::T] to refer to this.
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub struct ColonColon(T![:], T![:]);

	impl ColonColon {
		pub const fn dummy() -> Self {
			Self(<T![:]>::dummy(), <T![:]>::dummy())
		}
	}

	impl<'a> Peek<'a> for ColonColon {
		fn peek(p: &Parser<'a>, c: Cursor) -> bool {
			c == Kind::Colon && p.peek_n(2) == Kind::Colon
		}
	}

	impl<'a> Parse<'a> for ColonColon {
		fn parse(p: &mut Parser<'a>) -> Result<Self> {
			let first = p.parse::<T![:]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let second = p.parse::<T![:]>();
			p.set_skip(skip);
			Ok(Self(first, second?))
		}
	}

	impl ToCursors for ColonColon {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			s.append(self.0.into());
			s.append(self.1.into());
		}
	}

	impl ToSpan for ColonColon {
		fn to_span(&self) -> Span {
			self.0.to_span() + self.1.to_span()
		}
	}
}

/// Represents any possible single token. Use [T![Any]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Any(pub Cursor);
cursor_wrapped!(Any);

impl<'a> Peek<'a> for Any {
	fn peek(_: &Parser<'a>, _: Cursor) -> bool {
		true
	}
}

impl<'a> Parse<'a> for Any {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let c = p.next();
		Ok(Self(c))
	}
}

/// Represents a token with either [Kind::LeftCurly], [Kind::LeftParen] or [Kind::LeftSquare]. Use
/// [T![PairWiseStart]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseStart(pub Cursor);
cursor_wrapped!(PairWiseStart);

impl PairWiseStart {
	pub fn kind(&self) -> Kind {
		self.0.token().kind()
	}

	pub fn end(&self) -> Kind {
		match self.kind() {
			Kind::LeftCurly => Kind::RightCurly,
			Kind::LeftParen => Kind::RightParen,
			Kind::LeftSquare => Kind::RightSquare,
			k => k,
		}
	}
}

impl<'a> Peek<'a> for PairWiseStart {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly, Kind::LeftSquare, Kind::LeftParen]);
}

impl<'a> Parse<'a> for PairWiseStart {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(c))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

/// Represents a token with either [Kind::RightCurly], [Kind::RightParen] or [Kind::RightSquare]. Use
/// [T![PairWiseEnd]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseEnd(pub Cursor);
cursor_wrapped!(PairWiseEnd);

impl PairWiseEnd {
	pub fn kind(&self) -> Kind {
		self.0.token().kind()
	}

	pub fn start(&self) -> Kind {
		match self.kind() {
			Kind::RightCurly => Kind::LeftCurly,
			Kind::RightParen => Kind::LeftParen,
			Kind::RightSquare => Kind::LeftSquare,
			k => k,
		}
	}
}

impl<'a> Peek<'a> for PairWiseEnd {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::RightCurly, Kind::RightSquare, Kind::RightParen]);
}

impl<'a> Parse<'a> for PairWiseEnd {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(c))
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

/// The [T!][crate::T] macro expands to the name of a type representing the Token of the same name. These can be used in struct
/// fields to type child nodes.
#[macro_export]
macro_rules! T {
	[:] => { $crate::token_macros::Colon };
	[;] => { $crate::token_macros::Semicolon };
	[,] => { $crate::token_macros::Comma };
	['{'] => { $crate::token_macros::LeftCurly };
	['}'] => { $crate::token_macros::RightCurly };
	['['] => { $crate::token_macros::LeftSquare };
	[']'] => { $crate::token_macros::RightSquare };
	['('] => { $crate::token_macros::LeftParen };
	[')'] => { $crate::token_macros::RightParen };
	[' '] => { $crate::token_macros::Whitespace };

	[&] => { $crate::token_macros::delim::And };
	[@] => { $crate::token_macros::delim::At };
	[^] => { $crate::token_macros::delim::Caret };
	[-] => { $crate::token_macros::delim::Dash };
	[$] => { $crate::token_macros::delim::Dollar };
	[.] => { $crate::token_macros::delim::Dot };
	[=] => { $crate::token_macros::delim::Eq };
	[>] => { $crate::token_macros::delim::Gt };
	[#] => { $crate::token_macros::delim::Hash };
	[<] => { $crate::token_macros::delim::Lt };
	[!] => { $crate::token_macros::delim::Bang };
	[|] => { $crate::token_macros::delim::Or };
	[%] => { $crate::token_macros::delim::Percent };
	[+] => { $crate::token_macros::delim::Plus };
	[?] => { $crate::token_macros::delim::Question };
	[/] => { $crate::token_macros::delim::Slash };
	[*] => { $crate::token_macros::delim::Star };
	[~] => { $crate::token_macros::delim::Tilde };
	[_] => { $crate::token_macros::delim::Underscore };
	['`'] => { $crate::token_macros::delim::Backtick };

	[>=] => { $crate::token_macros::double::GreaterThanEqual };
	[<=] => { $crate::token_macros::double::LessThanEqual };
	[*|] => { $crate::token_macros::double::StarPipe };
	[::] => { $crate::token_macros::double::ColonColon };
	[||] => { $crate::token_macros::double::PipePipe };
	[==] => { $crate::token_macros::double::EqualEqual };
	[~=] => { $crate::token_macros::double::TildeEqual };
	[|=] => { $crate::token_macros::double::PipeEqual };
	[^=] => { $crate::token_macros::double::CaretEqual };
	["$="] => { $crate::token_macros::double::DollarEqual };
	[*=] => { $crate::token_macros::double::StarEqual };

	[Dimension::$ident: ident] => { $crate::token_macros::dimension::$ident };
	[DimensionIdent] => { $crate::token_macros::DimensionIdent };

	[!important] => { $crate::token_macros::double::BangImportant };

	[$ident:ident] => { $crate::token_macros::$ident }
}
