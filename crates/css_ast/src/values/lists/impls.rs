#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CounterIncrementStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<CounterResetStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<CounterSetStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ListStyleImageStyleValue>(), 128);
		assert_eq!(std::mem::size_of::<ListStylePositionStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ListStyleStyleValue>(), 216);
		assert_eq!(std::mem::size_of::<ListStyleTypeStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<MarkerSideStyleValue>(), 16);
	}

	#[test]
	fn test_counter_reset() {
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "my-counter");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "my-counter 5");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "a 1 b 2");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "reversed(foo)");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "reversed(foo) 3");
		assert_parse!(CssAtomSet::ATOMS, CounterResetStyleValue, "my-counter reversed(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, CounterResetStyleValue, "123");
	}

	#[test]
	fn test_counter_increment_parses() {
		assert_parse!(CssAtomSet::ATOMS, CounterIncrementStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CounterIncrementStyleValue, "my-counter");
		assert_parse!(CssAtomSet::ATOMS, CounterIncrementStyleValue, "my-counter 2");
		assert_parse!(CssAtomSet::ATOMS, CounterIncrementStyleValue, "a b c");
	}

	#[test]
	fn test_counter_increment_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CounterIncrementStyleValue, "123");
	}

	#[test]
	fn test_counter_set_parses() {
		assert_parse!(CssAtomSet::ATOMS, CounterSetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CounterSetStyleValue, "foo");
		assert_parse!(CssAtomSet::ATOMS, CounterSetStyleValue, "foo 5");
		assert_parse!(CssAtomSet::ATOMS, CounterSetStyleValue, "a 1 b 2");
	}

	#[test]
	fn test_counter_set_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CounterSetStyleValue, "123");
	}
}
