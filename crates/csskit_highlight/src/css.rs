use css_ast::{
	Property, PropertyRule, PropertyRuleProperty, PropertyRuleStyleValue, PseudoClass, StyleDeclaration, StyleValue,
	Tag, Visit,
};
use css_lexer::ToSpan;

use crate::{SemanticKind, SemanticModifier, TokenHighlighter};

impl<'a> Visit<'a> for TokenHighlighter {
	fn visit_tag(&mut self, tag: &Tag) {
		let span = tag.to_span();
		let mut modifier = SemanticModifier::none();
		match tag {
			Tag::HtmlNonConforming(_) => {
				modifier |= SemanticModifier::Deprecated;
			}
			Tag::Html(_) => {}
			Tag::HtmlNonStandard(_) => {
				modifier |= SemanticModifier::Experimental;
			}
			Tag::Svg(_) => {}
			Tag::Mathml(_) => {}
			Tag::CustomElement(_) => {
				modifier |= SemanticModifier::Custom;
			}
			Tag::Unknown(_) => {
				modifier |= SemanticModifier::Unknown;
			}
		}
		self.insert(span, SemanticKind::Tag, modifier);
	}

	fn visit_pseudo_class(&mut self, class: &PseudoClass) {
		let span = class.to_span();
		let mut modifier = SemanticModifier::none();
		match class {
			PseudoClass::Webkit(_) | PseudoClass::Moz(_) | PseudoClass::O(_) | PseudoClass::Ms(_) => {
				modifier |= SemanticModifier::Deprecated;
			}
			_ => {}
		}
		self.insert(span, SemanticKind::PseudoClass, modifier);
	}

	fn visit_style_declaration(&mut self, rule: &StyleDeclaration<'a>) {
		self.insert(rule.open.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		if let Some(close) = rule.close {
			self.insert(close.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		}
	}

	fn visit_property(&mut self, property: &Property<'a>) {
		let span = property.name.to_span();
		let mut modifier = SemanticModifier::none();
		if matches!(&property.value, StyleValue::Unknown(_)) {
			modifier |= SemanticModifier::Unknown;
		}
		if property.name.is_dashed_ident() {
			modifier |= SemanticModifier::Custom;
		}
		self.insert(span, SemanticKind::Declaration, modifier);
		self.insert(property.colon.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
	}

	fn visit_property_rule(&mut self, property: &PropertyRule<'a>) {
		self.insert(property.name.to_span(), SemanticKind::Declaration, SemanticModifier::Custom);
	}

	fn visit_property_rule_property(&mut self, property: &PropertyRuleProperty<'a>) {
		let span = property.name.to_span();
		let mut modifier = SemanticModifier::none();
		if matches!(&property.value, PropertyRuleStyleValue::Unknown(_)) {
			modifier |= SemanticModifier::Unknown;
		}
		if property.name.is_dashed_ident() {
			modifier |= SemanticModifier::Custom;
		}
		self.insert(span, SemanticKind::Declaration, modifier);
		self.insert(property.colon.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
	}
}
