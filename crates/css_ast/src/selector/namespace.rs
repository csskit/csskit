use css_lexer::{Cursor, KindSet};
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{IntoCursor, ToCursors};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

use super::Tag;

// https://drafts.csswg.org/selectors/#combinators
#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
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

impl<'a> Visitable<'a> for Namespace {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_namespace(self);
	}
}

#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NamespaceTag {
	Tag(Tag),
	Wildcard(T![*]),
}

impl<'a> Peek<'a> for NamespaceTag {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![*]>::peek(p, c) || Tag::peek(p, c)
	}
}

impl<'a> Build<'a> for NamespaceTag {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![*]>::peek(p, c) { Self::Wildcard(<T![*]>::build(p, c)) } else { Self::Tag(Tag::build(p, c)) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Namespace>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Namespace, "*|a");
		assert_parse!(Namespace, "html|div");
		assert_parse!(Namespace, "|span");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Namespace, "* | a");
	}
}
