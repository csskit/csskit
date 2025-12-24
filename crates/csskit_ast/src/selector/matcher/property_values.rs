use crate::QueryAttribute;
use css_ast::visit::QueryableNode;
use css_ast::{AttributeOperator, CssAtomSet, PropertyKind};
use css_parse::{AtomSet, Cursor};

/// Stores queryable property values extracted from a node.
#[derive(Clone, Copy, Default)]
pub(crate) struct PropertyValues {
	/// The `name` property value (for declarations, named at-rules, functions).
	pub name: Option<Cursor>,
}

impl PropertyValues {
	#[inline]
	pub(crate) fn from_node<T: QueryableNode>(node: &T) -> Self {
		Self { name: node.get_property(PropertyKind::Name) }
	}

	#[inline]
	pub(crate) fn get(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => self.name,
			_ => None,
		}
	}

	/// Match an attribute selector against property values.
	/// Returns true if the attribute matches.
	pub(crate) fn matches_attribute(&self, attr: &QueryAttribute, query_str: &str, source: &str) -> bool {
		let Some(property_kind) = attr.property_kind() else {
			return false;
		};
		let Some(cursor) = self.get(property_kind) else {
			return false;
		};
		let Some(expected_value) = attr.attr_value(query_str) else {
			return true;
		};
		let actual_value = cursor.str_slice(source);
		let Some(operator) = attr.operator() else {
			return true;
		};
		let actual = actual_value.as_bytes();
		let expected = expected_value.as_bytes();
		match operator {
			AttributeOperator::Exact(_) => {
				let expected_atom = CssAtomSet::from_str(expected_value);
				if expected_atom != CssAtomSet::_None {
					return CssAtomSet::from_bits(cursor.atom_bits()) == expected_atom;
				}
				actual.eq_ignore_ascii_case(expected)
			}
			AttributeOperator::SpaceList(_) => {
				!expected.is_empty()
					&& actual_value.split_ascii_whitespace().any(|word| word.as_bytes().eq_ignore_ascii_case(expected))
			}
			AttributeOperator::LangPrefix(_) => {
				expected.is_empty()
					|| actual.eq_ignore_ascii_case(expected)
					|| (actual.len() > expected.len()
						&& actual.get(expected.len()) == Some(&b'-')
						&& actual.get(..expected.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(expected)))
			}
			AttributeOperator::Prefix(_) => {
				expected.is_empty()
					|| actual.get(..expected.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(expected))
			}
			AttributeOperator::Suffix(_) => {
				expected.is_empty()
					|| actual
						.len()
						.checked_sub(expected.len())
						.and_then(|start| actual.get(start..))
						.is_some_and(|suffix| suffix.eq_ignore_ascii_case(expected))
			}
			AttributeOperator::Contains(_) => {
				expected.is_empty()
					|| actual.windows(expected.len()).any(|window| window.eq_ignore_ascii_case(expected))
			}
		}
	}

	/// Extract vendor prefix from property name (e.g., "-webkit-transform" becomes "webkit").
	/// Returns None for non-prefixed or custom properties (--foo).
	pub(crate) fn vendor_prefix<'a>(&self, source: &'a str) -> Option<&'a str> {
		let cursor = self.get(PropertyKind::Name)?;
		let name: &str = cursor.str_slice(source);
		if !name.starts_with('-') {
			return None;
		}
		let end = name[1..].find('-')?;
		if end == 0 {
			return None; // Excludes custom properties (--foo)
		}
		Some(&name[1..1 + end])
	}
}
