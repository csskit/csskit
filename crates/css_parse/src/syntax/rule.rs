use crate::{CursorSink, Parse, Parser, Result as ParserResult, T, ToCursors};

use super::{AtRule, QualifiedRule};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Rule<'a> {
	AtRule(AtRule<'a>),
	QualifiedRule(QualifiedRule<'a>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![AtKeyword]>() {
			return p.parse::<AtRule>().map(Self::AtRule);
		}
		p.parse::<QualifiedRule>().map(Self::QualifiedRule)
	}
}

impl<'a> ToCursors for Rule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::AtRule(rule) => ToCursors::to_cursors(rule, s),
			Self::QualifiedRule(rule) => ToCursors::to_cursors(rule, s),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Rule>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Rule, "body{color:black}");
	}
}
