use crate::{Block as BlockTrait, CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};
use bumpalo::collections::Vec;
use css_lexer::{Kind, KindSet};

use super::{Declaration, Rule};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Block<'a> {
	pub open_curly: T!['{'],
	pub declarations: Vec<'a, (Declaration<'a>, Option<T![;]>)>,
	pub rules: Vec<'a, Rule<'a>>,
	pub close_curly: Option<T!['}']>,
}

impl<'a> Peek<'a> for Block<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a> Parse<'a> for Block<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open_curly, declarations, rules, close_curly) = Self::parse_block(p)?;
		Ok(Self { open_curly, declarations, rules, close_curly })
	}
}

impl<'a> BlockTrait<'a> for Block<'a> {
	type Declaration = Declaration<'a>;
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for Block<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Block>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Block, "{color:black}");
	}
}
