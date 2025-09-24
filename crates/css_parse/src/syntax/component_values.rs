use crate::{
	AssociatedWhitespaceRules, Cursor, CursorSink, DeclarationValue, Parse, Parser, Peek, Result, Span, ToCursors,
	ToSpan,
};
use bumpalo::collections::Vec;

use super::ComponentValue;

// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ComponentValues<'a> {
	values: Vec<'a, ComponentValue<'a>>,
}

impl<'a> Peek<'a> for ComponentValues<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		ComponentValue::peek(p, c)
	}
}

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let mut values = Vec::new_in(p.bump());
		let mut last_was_whitespace = false;
		loop {
			if p.at_end() {
				break;
			}
			if p.next_is_stop() {
				break;
			}
			let c = p.peek_n(1);
			if <ComponentValue>::peek(p, c) {
				let mut value = p.parse::<ComponentValue>()?;
				if let ComponentValue::Delim(d) = value
					&& last_was_whitespace
				{
					let rules = d.associated_whitespace() | AssociatedWhitespaceRules::EnforceBefore;
					value = ComponentValue::Delim(d.with_associated_whitespace(rules))
				}
				last_was_whitespace = matches!(value, ComponentValue::Whitespace(_));
				values.push(value);
			} else {
				break;
			}
		}
		Ok(Self { values })
	}
}

impl<'a> DeclarationValue<'a> for ComponentValues<'a> {
	type ComputedValue = ComponentValues<'a>;

	fn is_initial(&self) -> bool {
		false
	}

	fn is_inherit(&self) -> bool {
		false
	}

	fn is_unset(&self) -> bool {
		false
	}

	fn is_revert(&self) -> bool {
		false
	}

	fn is_revert_layer(&self) -> bool {
		false
	}

	fn needs_computing(&self) -> bool {
		false
	}

	fn parse_custom_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		Self::parse(p)
	}

	fn parse_computed_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		Self::parse(p)
	}

	fn parse_unknown_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		Self::parse(p)
	}
}

impl<'a> ToCursors for ComponentValues<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.values, s)
	}
}

impl<'a> ToSpan for ComponentValues<'a> {
	fn to_span(&self) -> Span {
		self.values.to_span()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EmptyAtomSet, test_helpers::*};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ComponentValues>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "body{color:black}");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "body");
	}
}
