use super::prelude::*;
use crate::AssociatedWhitespaceRules;

use super::ComponentValue;

/// <https://drafts.csswg.org/css-syntax-3/#consume-list-of-components>
#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ComponentValues<'a> {
	pub values: Vec<'a, ComponentValue<'a>>,
}

impl<'a> Peek<'a> for ComponentValues<'a> {
	const PEEK_KINDSET: KindSet = ComponentValue::PEEK_KINDSET;
}

impl<'a> Parse<'a> for ComponentValues<'a> {
	// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		let mut values = Vec::new_in(p.alloc());
		let mut last_was_whitespace = false;
		let mut trailing_whitespace = None;

		loop {
			if p.at_end() {
				break;
			}
			if p.next_is_stop() {
				break;
			}
			if let Some(mut value) = p.parse_if_peek::<ComponentValue>()? {
				if let ComponentValue::Delim(d) = value
					&& last_was_whitespace
					&& d.associated_whitespace().contains(AssociatedWhitespaceRules::EnforceAfter)
				{
					let rules = d.associated_whitespace() | AssociatedWhitespaceRules::EnforceBefore;
					value = ComponentValue::Delim(d.with_associated_whitespace(rules))
				}
				// Whitespace at either edge of the list separates nothing - CSS discards it when consuming a
				// declaration value or an at-rule prelude - so it is trivia and a minifier can remove it.
				// Whitespace between two values is grammar (`foo(.a .b)` is not `foo(.a.b)`) and stays significant.
				last_was_whitespace = match value {
					ComponentValue::Whitespace(ws) => {
						if values.is_empty() {
							value = ComponentValue::Whitespace(ws.with_significant_whitespace(false));
						} else if trailing_whitespace.is_none() {
							trailing_whitespace = Some(values.len());
						}
						true
					}
					_ => {
						trailing_whitespace = None;
						false
					}
				};
				values.push(value);
			} else {
				break;
			}
		}

		if let Some(index) = trailing_whitespace {
			for value in &mut values[index..] {
				if let ComponentValue::Whitespace(ws) = value {
					*value = ComponentValue::Whitespace(ws.with_significant_whitespace(false));
				}
			}
		}
		Ok(Self { values })
	}
}

impl<'a, M: NodeMetadata> NodeWithMetadata<M> for ComponentValues<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}

impl<'a> DeclarationValue<'a, ()> for ComponentValues<'a> {
	fn parse_custom_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _name: Cursor) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		Self::parse(p)
	}

	fn is_computed_declaration_value<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		<Self as Peek>::peek(p, c)
	}

	fn parse_computed_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _name: Cursor) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		Self::parse(p)
	}

	fn parse_unknown_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _name: Cursor) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
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

// Implement for ComponentValues - compare sequences, ignoring whitespace
impl<'a> SemanticEq for ComponentValues<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self.values.semantic_eq(&other.values)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EmptyAtomSet, test_helpers::*};

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "body{color:black}");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "body");
	}

	#[test]
	fn test_writes_with_trivia() {
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "/*comment*/foo");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, " /*comment*/ foo");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "/*a*/foo/*b*/bar");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "foo/*comment*/bar");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, " \t foo");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, " /*start*/ foo /*mid*/ bar");
		assert_parse!(EmptyAtomSet::ATOMS, ComponentValues, "/*comment*/foo");
	}
}
