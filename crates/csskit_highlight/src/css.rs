use css_ast::{Class, Declaration, DeclarationValue, Id, MediaRule, PropertyRule, PseudoClass, StyleRule, Tag, Visit};
use css_lexer::ToSpan;

use crate::{SemanticKind, SemanticModifier, TokenHighlighter};

impl Visit for TokenHighlighter {
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

	fn visit_style_rule<'a>(&mut self, rule: &StyleRule<'a>) {
		self.insert(rule.0.block.open_curly.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		if let Some(close) = rule.0.block.close_curly {
			self.insert(close.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
		}
	}

	fn visit_declaration<'a, T: DeclarationValue<'a>>(&mut self, property: &Declaration<'a, T>) {
		let span = property.name.to_span();
		let mut modifier = SemanticModifier::none();
		if property.value.is_unknown() {
			modifier |= SemanticModifier::Unknown;
		}
		if property.name.is_dashed_ident() {
			modifier |= SemanticModifier::Custom;
		}
		self.insert(span, SemanticKind::Declaration, modifier);
		self.insert(property.colon.to_span(), SemanticKind::Punctuation, SemanticModifier::none());
	}

	fn visit_property_rule<'a>(&mut self, property: &PropertyRule<'a>) {
		self.insert(property.0.prelude.to_span(), SemanticKind::Declaration, SemanticModifier::Custom);
	}

	fn visit_class(&mut self, class: &Class) {
		self.insert(class.to_span(), SemanticKind::Class, SemanticModifier::none());
	}

	fn visit_id(&mut self, id: &Id) {
		self.insert(id.to_span(), SemanticKind::Id, SemanticModifier::none());
	}

	fn visit_media_rule<'a>(&mut self, _media: &MediaRule<'a>) {
		// For now, let's not highlight the entire rule - let the inner elements be highlighted
		// We'll need to find a different way to highlight just the @media keyword
		// self.insert(media.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}
}
