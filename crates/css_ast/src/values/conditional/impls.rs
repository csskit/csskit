#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<ContainerTypeStyleValue>(), 36);
		assert_eq!(std::mem::size_of::<ContainerNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<ContainerStyleValue>(), 80);
	}

	#[test]
	fn test_container_type() {
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "size");
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "inline-size");
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "scroll-state");
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "size scroll-state");
		assert_parse!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "inline-size scroll-state");
		assert_parse_error!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, ContainerTypeStyleValue, "auto");
	}

	#[test]
	fn test_container() {
		assert_parse!(CssAtomSet::ATOMS, ContainerStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainerStyleValue, "sidebar");
		assert_parse!(CssAtomSet::ATOMS, ContainerStyleValue, "sidebar / size");
		assert_parse!(CssAtomSet::ATOMS, ContainerStyleValue, "none / inline-size");
		assert_parse_error!(CssAtomSet::ATOMS, ContainerStyleValue, "1px");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "a");
		assert_parse!(CssAtomSet::ATOMS, ContainerNameStyleValue, "a b c");
	}
}
