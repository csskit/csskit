use css_parse::{Diagnostic, KindSet, Parse, Parser, Result as ParserResult, T};
use csskit_derives::{IntoCursor, Peek, ToCursors, ToSpan};

use super::Tag;

// https://drafts.csswg.org/selectors/#combinators
#[derive(Peek, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Namespace {
	pub prefix: Option<NamespacePrefix>,
	pub tag: NamespaceTag,
}

impl<'a> Parse<'a> for Namespace {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![*|]>() {
			let prefix = p.parse::<NamespacePrefix>()?;
			let tag = p.parse::<NamespaceTag>()?;
			return Ok(Self { prefix: Some(prefix), tag });
		}
		if p.peek::<T![|]>() {
			let prefix = p.parse::<NamespacePrefix>()?;
			let tag = p.parse::<NamespaceTag>()?;
			return Ok(Self { prefix: Some(prefix), tag });
		}

		let ident = p.parse::<T![Ident]>()?;
		let skip = p.set_skip(KindSet::NONE);
		if p.peek::<T![|]>() && !p.peek::<T![|=]>() {
			let pipe = p.parse::<T![|]>();
			let tag = p.parse::<NamespaceTag>();
			p.set_skip(skip);
			let prefix = NamespacePrefix::Name(ident, pipe?);
			return Ok(Self { prefix: Some(prefix), tag: tag? });
		}
		let tag = p.parse::<NamespaceTag>()?;
		Ok(Self { prefix: None, tag })
	}
}

#[derive(Peek, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamespacePrefix {
	None(T![|]),
	Name(T![Ident], T![|]),
	Wildcard(T![*], T![|]),
}

impl<'a> Parse<'a> for NamespacePrefix {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![|]>() {
			let pipe = p.parse::<T![|]>()?;
			Ok(Self::None(pipe))
		} else if p.peek::<T![*]>() {
			let star = p.parse::<T![*]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let pipe = p.parse::<T![|]>();
			p.set_skip(skip);
			let pipe = pipe?;
			Ok(Self::Wildcard(star, pipe))
		} else {
			let star = p.parse::<T![Ident]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let pipe = p.parse::<T![|]>();
			p.set_skip(skip);
			let pipe = pipe?;
			Ok(Self::Name(star, pipe))
		}
	}
}

#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamespaceTag {
	Wildcard(T![*]),
	Tag(Tag),
}

impl<'a> Parse<'a> for NamespaceTag {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Self>() {
			if p.peek::<T![*]>() { Ok(Self::Wildcard(p.parse::<T![*]>()?)) } else { Ok(Self::Tag(p.parse::<Tag>()?)) }
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Namespace>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Namespace, "*|a");
		assert_parse!(CssAtomSet::ATOMS, Namespace, "html|div");
		assert_parse!(CssAtomSet::ATOMS, Namespace, "|span");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Namespace, "* | a");
	}
}
