use super::prelude::*;

/// <https://drafts.csswg.org/css-values-5/#css-syntax>
///
/// ```text,ignore
/// <syntax> = '*' | <syntax-component> [ <syntax-combinator> <syntax-component> ]* | <syntax-string>
/// <syntax-component> = <syntax-single-component> <syntax-multiplier>?
///                    | '<' transform-list '>'
/// <syntax-single-component> = '<' <syntax-type-name> '>' | <ident>
/// <syntax-type-name> = angle | color | custom-ident | image | integer
///                    | length | length-percentage | number
///                    | percentage | resolution | string | time
///                    | url | transform-function
/// <syntax-combinator> = '|'
/// <syntax-multiplier> = [ '#' | '+' ]
///
/// <syntax-string> = <string>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Syntax<'a> {
	Universal(T![*]),
	Components(SyntaxComponent, Vec<'a, (T![|], SyntaxComponent)>),
	String(T![String]),
}

/// ```text,ignore
/// <syntax-component> = <syntax-single-component> <syntax-multiplier>?
///                    | '<' transform-list '>'
/// ```
///
/// Whitespace is not allowed between the angle brackets and the type name they enclose, nor between a component and
/// its multiplier. `<transform-list>` may not be followed by a multiplier.
#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SyntaxComponent {
	Type(T![<], SyntaxTypeName, T![>], Option<SyntaxMultiplier>),
	TransformList(T![<], T![Ident], T![>]),
	Keyword(T![Ident], Option<SyntaxMultiplier>),
}

impl SyntaxComponent {
	fn parse_bracketed<'a, I>(p: &mut Parser<'a, I>, open: T![<]) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if c == Kind::Ident && p.to_atom::<CssAtomSet>(c) == CssAtomSet::TransformList {
			let ident = p.parse::<T![Ident]>()?;
			let close = p.parse::<T![>]>()?;
			return Ok(Self::TransformList(open, ident, close));
		}
		let name = p.parse::<SyntaxTypeName>()?;
		let close = p.parse::<T![>]>()?;
		let multiplier = p.parse_if_peek::<SyntaxMultiplier>()?;
		Ok(Self::Type(open, name, close, multiplier))
	}
}

impl<'a> Peek<'a> for SyntaxComponent {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::Delim]);

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Kind::Ident || <T![<] as Peek>::peek(p, c)
	}
}

impl<'a> Parse<'a> for SyntaxComponent {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![Ident]>() {
			let ident = p.parse::<T![Ident]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let multiplier = p.parse_if_peek::<SyntaxMultiplier>();
			p.set_skip(skip);
			return Ok(Self::Keyword(ident, multiplier?));
		}
		let open = p.parse::<T![<]>()?;
		let skip = p.set_skip(KindSet::NONE);
		let component = Self::parse_bracketed(p, open);
		p.set_skip(skip);
		component
	}
}

/// ```text,ignore
/// <syntax-type-name> = angle | color | custom-ident | image | integer
///                    | length | length-percentage | number
///                    | percentage | resolution | string | time
///                    | url | transform-function
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SyntaxTypeName {
	#[atom(CssAtomSet::Angle)]
	Angle(T![Ident]),
	#[atom(CssAtomSet::Color)]
	Color(T![Ident]),
	#[atom(CssAtomSet::CustomIdent)]
	CustomIdent(T![Ident]),
	#[atom(CssAtomSet::Image)]
	Image(T![Ident]),
	#[atom(CssAtomSet::Integer)]
	Integer(T![Ident]),
	#[atom(CssAtomSet::Length)]
	Length(T![Ident]),
	#[atom(CssAtomSet::LengthPercentage)]
	LengthPercentage(T![Ident]),
	#[atom(CssAtomSet::Number)]
	Number(T![Ident]),
	#[atom(CssAtomSet::Percentage)]
	Percentage(T![Ident]),
	#[atom(CssAtomSet::Resolution)]
	Resolution(T![Ident]),
	#[atom(CssAtomSet::String)]
	String(T![Ident]),
	#[atom(CssAtomSet::Time)]
	Time(T![Ident]),
	#[atom(CssAtomSet::Url)]
	Url(T![Ident]),
	#[atom(CssAtomSet::TransformFunction)]
	TransformFunction(T![Ident]),
}

/// ```text,ignore
/// <syntax-multiplier> = [ '#' | '+' ]
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SyntaxMultiplier {
	Hash(T![#]),
	Plus(T![+]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Syntax, "*");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "auto");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "auto+");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "auto#");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<length>");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<length-percentage>");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<custom-ident>");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<color>#");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<transform-function>+");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<transform-list>");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "<percentage> | <number> | auto");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "red|<color>");
		assert_parse!(CssAtomSet::ATOMS, Syntax, "\"<length> | auto\"");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "< length>");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "<length >");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "<length> +");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "<foo>");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "<transform-list>+");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "<length> <color>");
		assert_parse_error!(CssAtomSet::ATOMS, Syntax, "auto |");
		assert_peek_false!(CssAtomSet::ATOMS, Syntax, "1");
	}
}
