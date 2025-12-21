use css_ast::visit::NodeId;
use css_parse::AtomSet;

use crate::CsskitAtomSet;

#[derive(Debug, Clone)]
pub struct QuerySelectorList {
	pub selectors: Vec<QuerySelector>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuerySelector {
	pub parts: Vec<QuerySelectorPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuerySelectorPart {
	Simple(QuerySimpleSelector),
	Combinator(QueryCombinator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuerySimpleSelector {
	pub node_type: Option<NodeId>,
	pub attributes: Vec<QueryAttribute>,
	pub pseudo_classes: Vec<QueryPseudo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryAttribute {
	Name(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryCombinator {
	Descendant,
	Child,
	NextSibling,
	SubsequentSibling,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryPseudo {
	Important,
	Custom,
	Prefixed(Option<String>),
	Unknown,
	Computed,
	Shorthand,
	Longhand,
	PropertyType(CsskitAtomSet),
	Empty,
	Nested,
	Root,
	FirstChild,
	LastChild,
	OnlyChild,
	NthChild(NthValue),
	NthLastChild(NthValue),
	FirstOfType,
	LastOfType,
	OnlyOfType,
	NthOfType(NthValue),
	NthLastOfType(NthValue),
	Not(Box<QuerySelector>),
	AtRule,
	Rule,
	Function,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NthValue {
	Odd,
	Even,
	Index(i32),
	AnPlusB(i32, i32),
}

impl NthValue {
	pub fn matches(&self, index: i32) -> bool {
		match self {
			Self::Odd => index % 2 == 1,
			Self::Even => index % 2 == 0,
			Self::Index(n) => index == *n,
			Self::AnPlusB(a, b) => {
				if *a == 0 {
					index == *b
				} else {
					let diff = index - b;
					diff % a == 0 && diff / a >= 0
				}
			}
		}
	}
}

impl QuerySelectorList {
	pub fn parse(input: &str) -> Result<Self, SelectorParseError> {
		SelectorParser::new(input).parse_list()
	}
}

impl QuerySelector {
	pub fn parse(input: &str) -> Result<Self, SelectorParseError> {
		SelectorParser::new(input).parse_selector()
	}
}

#[derive(Debug)]
pub struct SelectorParseError {
	pub message: String,
	pub offset: usize,
}

impl std::fmt::Display for SelectorParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} at offset {}", self.message, self.offset)
	}
}

impl std::error::Error for SelectorParseError {}

struct SelectorParser<'a> {
	input: &'a str,
	pos: usize,
}

impl<'a> SelectorParser<'a> {
	fn new(input: &'a str) -> Self {
		Self { input, pos: 0 }
	}

	fn parse_list(&mut self) -> Result<QuerySelectorList, SelectorParseError> {
		let mut selectors = vec![self.parse_selector()?];
		self.skip_ws();
		while self.peek() == Some(',') {
			self.advance();
			self.skip_ws();
			selectors.push(self.parse_selector()?);
			self.skip_ws();
		}
		Ok(QuerySelectorList { selectors })
	}

	fn parse_selector(&mut self) -> Result<QuerySelector, SelectorParseError> {
		let mut parts = Vec::new();
		self.skip_ws();

		loop {
			let simple = self.parse_simple_selector()?;
			parts.push(QuerySelectorPart::Simple(simple));
			self.skip_ws();

			if let Some(comb) = self.try_combinator() {
				parts.push(QuerySelectorPart::Combinator(comb));
				self.skip_ws();
			} else if self.peek().is_some_and(|c| c != ',' && c != ')') {
				// Implicit descendant combinator
				parts.push(QuerySelectorPart::Combinator(QueryCombinator::Descendant));
			} else {
				break;
			}
		}

		if parts.is_empty() {
			return Err(self.error("expected selector"));
		}

		Ok(QuerySelector { parts })
	}

	fn parse_simple_selector(&mut self) -> Result<QuerySimpleSelector, SelectorParseError> {
		let (node_type, is_universal) = self.parse_type_selector()?;
		let mut attributes = Vec::new();
		let mut pseudo_classes = Vec::new();

		loop {
			match self.peek() {
				Some('[') => {
					self.advance();
					attributes.push(self.parse_attribute()?);
				}
				Some(':') => {
					self.advance();
					pseudo_classes.push(self.parse_pseudo_class()?);
				}
				_ => break,
			}
		}

		if node_type.is_none() && !is_universal && attributes.is_empty() && pseudo_classes.is_empty() {
			return Err(self.error("expected type, attribute, or pseudo-class"));
		}

		Ok(QuerySimpleSelector { node_type, attributes, pseudo_classes })
	}

	fn parse_attribute(&mut self) -> Result<QueryAttribute, SelectorParseError> {
		self.skip_ws();
		let attr_start = self.pos;
		while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_') {
			self.advance();
		}
		let attr_name = &self.input[attr_start..self.pos];
		if attr_name.is_empty() {
			return Err(self.error("expected attribute name"));
		}
		self.skip_ws();
		if attr_name != "name" {
			return Err(SelectorParseError {
				message: format!("unknown attribute '{attr_name}', only 'name' is supported"),
				offset: attr_start,
			});
		}
		if self.peek() != Some('=') {
			return Err(self.error("expected '=' after attribute name"));
		}
		self.advance();
		self.skip_ws();
		let value = self.parse_attribute_value()?;
		self.skip_ws();
		if self.peek() != Some(']') {
			return Err(self.error("expected ']'"));
		}
		self.advance();
		Ok(QueryAttribute::Name(value))
	}

	fn parse_attribute_value(&mut self) -> Result<String, SelectorParseError> {
		let quote = self.peek();
		if quote == Some('"') || quote == Some('\'') {
			self.advance();
			let start = self.pos;
			while self.peek().is_some_and(|c| c != quote.unwrap()) {
				self.advance();
			}
			let value = self.input[start..self.pos].to_string();
			if self.peek() != quote {
				return Err(self.error("unterminated string"));
			}
			self.advance();
			Ok(value)
		} else {
			let start = self.pos;
			while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_') {
				self.advance();
			}
			if start == self.pos {
				return Err(self.error("expected attribute value"));
			}
			Ok(self.input[start..self.pos].to_string())
		}
	}

	fn parse_type_selector(&mut self) -> Result<(Option<NodeId>, bool), SelectorParseError> {
		if self.peek() == Some('*') {
			self.advance();
			return Ok((None, true)); // Universal
		}

		let start = self.pos;
		while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_') {
			self.advance();
		}

		if start == self.pos {
			return Ok((None, false));
		}

		let name = &self.input[start..self.pos];
		match NodeId::from_tag_name(name) {
			Some(id) => Ok((Some(id), false)),
			None => Err(SelectorParseError { message: format!("unknown node type '{name}'"), offset: start }),
		}
	}

	fn parse_pseudo_class(&mut self) -> Result<QueryPseudo, SelectorParseError> {
		let start = self.pos;
		while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_') {
			self.advance();
		}

		let name = &self.input[start..self.pos];
		let atom = CsskitAtomSet::from_str(name);

		match atom {
			CsskitAtomSet::Important => Ok(QueryPseudo::Important),
			CsskitAtomSet::Custom => Ok(QueryPseudo::Custom),
			CsskitAtomSet::Unknown => Ok(QueryPseudo::Unknown),
			CsskitAtomSet::Computed => Ok(QueryPseudo::Computed),
			CsskitAtomSet::Shorthand => Ok(QueryPseudo::Shorthand),
			CsskitAtomSet::Longhand => Ok(QueryPseudo::Longhand),
			CsskitAtomSet::Empty => Ok(QueryPseudo::Empty),
			CsskitAtomSet::Nested => Ok(QueryPseudo::Nested),
			CsskitAtomSet::Root => Ok(QueryPseudo::Root),
			CsskitAtomSet::FirstChild => Ok(QueryPseudo::FirstChild),
			CsskitAtomSet::LastChild => Ok(QueryPseudo::LastChild),
			CsskitAtomSet::OnlyChild => Ok(QueryPseudo::OnlyChild),
			CsskitAtomSet::FirstOfType => Ok(QueryPseudo::FirstOfType),
			CsskitAtomSet::LastOfType => Ok(QueryPseudo::LastOfType),
			CsskitAtomSet::OnlyOfType => Ok(QueryPseudo::OnlyOfType),
			CsskitAtomSet::AtRule => Ok(QueryPseudo::AtRule),
			CsskitAtomSet::Rule => Ok(QueryPseudo::Rule),
			CsskitAtomSet::Function => Ok(QueryPseudo::Function),
			CsskitAtomSet::NthChild => Ok(QueryPseudo::NthChild(self.parse_nth_pattern()?)),
			CsskitAtomSet::NthLastChild => Ok(QueryPseudo::NthLastChild(self.parse_nth_pattern()?)),
			CsskitAtomSet::NthOfType => Ok(QueryPseudo::NthOfType(self.parse_nth_pattern()?)),
			CsskitAtomSet::NthLastOfType => Ok(QueryPseudo::NthLastOfType(self.parse_nth_pattern()?)),
			CsskitAtomSet::Prefixed => {
				if self.peek() == Some('(') {
					self.advance();
					let arg_start = self.pos;
					while self.peek().is_some_and(|c| c != ')') {
						self.advance();
					}
					let arg = self.input[arg_start..self.pos].trim().to_string();
					if self.peek() != Some(')') {
						return Err(self.error("expected ')'"));
					}
					self.advance();
					Ok(QueryPseudo::Prefixed(Some(arg)))
				} else {
					Ok(QueryPseudo::Prefixed(None))
				}
			}
			CsskitAtomSet::PropertyType => {
				if self.peek() != Some('(') {
					return Err(self.error("expected '(' after :property-type"));
				}
				self.advance();
				self.skip_ws();
				let arg_start = self.pos;
				while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '-' || c == '_') {
					self.advance();
				}
				let arg = &self.input[arg_start..self.pos];
				if arg.is_empty() {
					return Err(self.error("expected property group name"));
				}
				let group_atom = CsskitAtomSet::from_str(arg);
				self.skip_ws();
				if self.peek() != Some(')') {
					return Err(self.error("expected ')'"));
				}
				self.advance();
				Ok(QueryPseudo::PropertyType(group_atom))
			}
			CsskitAtomSet::Not => {
				if self.peek() != Some('(') {
					return Err(self.error("expected '(' after :not"));
				}
				self.advance();
				self.skip_ws();
				let inner = self.parse_selector()?;
				self.skip_ws();
				if self.peek() != Some(')') {
					return Err(self.error("expected ')'"));
				}
				self.advance();
				Ok(QueryPseudo::Not(Box::new(inner)))
			}
			_ => Err(SelectorParseError { message: format!("unknown pseudo-class ':{name}'"), offset: start }),
		}
	}

	fn parse_nth_pattern(&mut self) -> Result<NthValue, SelectorParseError> {
		if self.peek() != Some('(') {
			return Err(self.error("expected '(' after :nth-child"));
		}
		self.advance();
		self.skip_ws();

		let pattern = self.parse_nth_expression()?;

		self.skip_ws();
		if self.peek() != Some(')') {
			return Err(self.error("expected ')' after :nth-child(...)"));
		}
		self.advance();
		Ok(pattern)
	}

	fn parse_nth_expression(&mut self) -> Result<NthValue, SelectorParseError> {
		let start = self.pos;
		while self.peek().is_some_and(|c| c.is_alphabetic()) {
			self.advance();
		}
		let keyword = &self.input[start..self.pos];

		if keyword.eq_ignore_ascii_case("odd") {
			return Ok(NthValue::Odd);
		}
		if keyword.eq_ignore_ascii_case("even") {
			return Ok(NthValue::Even);
		}

		self.pos = start;

		let mut a: i32 = 0;
		let mut b: i32 = 0;
		let mut has_n = false;

		let sign = match self.peek() {
			Some('-') => {
				self.advance();
				-1
			}
			Some('+') => {
				self.advance();
				1
			}
			_ => 1,
		};

		self.skip_ws();

		let num_start = self.pos;
		while self.peek().is_some_and(|c| c.is_ascii_digit()) {
			self.advance();
		}
		let num_str = &self.input[num_start..self.pos];

		if self.peek() == Some('n') || self.peek() == Some('N') {
			has_n = true;
			a = if num_str.is_empty() { sign } else { sign * num_str.parse::<i32>().unwrap_or(1) };
			self.advance();
			self.skip_ws();

			if self.peek() == Some('+') || self.peek() == Some('-') {
				let b_sign = if self.peek() == Some('-') { -1 } else { 1 };
				self.advance();
				self.skip_ws();

				let b_start = self.pos;
				while self.peek().is_some_and(|c| c.is_ascii_digit()) {
					self.advance();
				}
				let b_str = &self.input[b_start..self.pos];
				if !b_str.is_empty() {
					b = b_sign * b_str.parse::<i32>().unwrap_or(0);
				}
			}
		} else if !num_str.is_empty() {
			b = sign * num_str.parse::<i32>().unwrap_or(0);
		} else {
			return Err(self.error("expected number or 'n' in :nth-child()"));
		}

		if has_n { Ok(NthValue::AnPlusB(a, b)) } else { Ok(NthValue::Index(b)) }
	}

	fn try_combinator(&mut self) -> Option<QueryCombinator> {
		match self.peek()? {
			'>' => {
				self.advance();
				Some(QueryCombinator::Child)
			}
			'+' => {
				self.advance();
				Some(QueryCombinator::NextSibling)
			}
			'~' => {
				self.advance();
				Some(QueryCombinator::SubsequentSibling)
			}
			_ => None,
		}
	}

	fn skip_ws(&mut self) {
		while self.peek().is_some_and(|c| c.is_whitespace()) {
			self.advance();
		}
	}

	fn peek(&self) -> Option<char> {
		self.input[self.pos..].chars().next()
	}

	fn advance(&mut self) {
		if let Some(c) = self.peek() {
			self.pos += c.len_utf8();
		}
	}

	fn error(&self, msg: &str) -> SelectorParseError {
		SelectorParseError { message: msg.to_string(), offset: self.pos }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_simple_type() {
		let sel = QuerySelector::parse("style-rule").unwrap();
		assert_eq!(sel.parts.len(), 1);
	}

	#[test]
	fn parse_universal() {
		let sel = QuerySelector::parse("*").unwrap();
		assert_eq!(sel.parts.len(), 1);
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert!(s.node_type.is_none());
		} else {
			panic!("expected simple selector");
		}
	}

	#[test]
	fn parse_pseudo_class() {
		let sel = QuerySelector::parse("*:important").unwrap();
		assert_eq!(sel.parts.len(), 1);
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert_eq!(s.pseudo_classes, vec![QueryPseudo::Important]);
		}
	}

	#[test]
	fn parse_descendant() {
		let sel = QuerySelector::parse("style-rule *:important").unwrap();
		assert_eq!(sel.parts.len(), 3); // type, combinator, simple
	}

	#[test]
	fn parse_child() {
		let sel = QuerySelector::parse("style-rule > *:important").unwrap();
		assert_eq!(sel.parts.len(), 3);
		assert!(matches!(sel.parts[1], QuerySelectorPart::Combinator(QueryCombinator::Child)));
	}

	#[test]
	fn parse_list() {
		let list = QuerySelectorList::parse("style-rule, media-rule").unwrap();
		assert_eq!(list.selectors.len(), 2);
	}

	#[test]
	fn parse_not() {
		let sel = QuerySelector::parse("*:not(:important)").unwrap();
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert!(matches!(&s.pseudo_classes[0], QueryPseudo::Not(_)));
		}
	}

	#[test]
	fn parse_attribute_selector() {
		let sel = QuerySelector::parse("[name=color]").unwrap();
		assert_eq!(sel.parts.len(), 1);
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert_eq!(s.attributes.len(), 1);
			assert!(matches!(&s.attributes[0], QueryAttribute::Name(n) if n == "color"));
		}
	}

	#[test]
	fn parse_attribute_selector_quoted() {
		let sel = QuerySelector::parse("[name='background-color']").unwrap();
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert!(matches!(&s.attributes[0], QueryAttribute::Name(n) if n == "background-color"));
		}
	}

	#[test]
	fn parse_attribute_selector_double_quoted() {
		let sel = QuerySelector::parse("[name=\"margin-top\"]").unwrap();
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert!(matches!(&s.attributes[0], QueryAttribute::Name(n) if n == "margin-top"));
		}
	}

	#[test]
	fn parse_attribute_unknown_attr() {
		let result = QuerySelector::parse("[foo=bar]");
		assert!(result.is_err());
	}

	#[test]
	fn parse_universal_with_attribute() {
		let sel = QuerySelector::parse("*[name=color]").unwrap();
		if let QuerySelectorPart::Simple(s) = &sel.parts[0] {
			assert!(s.node_type.is_none());
			assert_eq!(s.attributes.len(), 1);
		}
	}
}
