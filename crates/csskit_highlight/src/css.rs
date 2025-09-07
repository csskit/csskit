use css_ast::{
	Angle, CSSInt, Class, Color, ContainerRule, Declaration, DeclarationValue, DocumentRule, Flex, FontFaceRule, Id,
	KeyframesRule, LayerRule, Length, LengthPercentage, MarginRule, MediaRule, MozDocumentRule, PageRule, PropertyRule,
	PseudoClass, PseudoElement, StyleRule, SupportsRule, Tag, Time, ToChromashift, Url, Visit, WebkitKeyframesRule,
};
use css_lexer::ToSpan;

use crate::{SemanticDecoration, SemanticKind, SemanticModifier, TokenHighlighter};

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
		self.insert(property.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
		self.insert(property.0.prelude.to_span(), SemanticKind::Declaration, SemanticModifier::Custom);
	}

	fn visit_class(&mut self, class: &Class) {
		self.insert(class.to_span(), SemanticKind::Class, SemanticModifier::none());
	}

	fn visit_id(&mut self, id: &Id) {
		self.insert(id.to_span(), SemanticKind::Id, SemanticModifier::none());
	}

	fn visit_media_rule<'a>(&mut self, media: &MediaRule<'a>) {
		self.insert(media.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_keyframes_rule<'a>(&mut self, keyframes: &KeyframesRule<'a>) {
		self.insert(keyframes.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_supports_rule<'a>(&mut self, supports: &SupportsRule<'a>) {
		self.insert(supports.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_font_face_rule<'a>(&mut self, font_face: &FontFaceRule<'a>) {
		self.insert(font_face.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_container_rule<'a>(&mut self, container: &ContainerRule<'a>) {
		self.insert(container.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_page_rule<'a>(&mut self, page: &PageRule<'a>) {
		self.insert(page.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_layer_rule<'a>(&mut self, layer: &LayerRule<'a>) {
		self.insert(layer.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_margin_rule<'a>(&mut self, margin: &MarginRule<'a>) {
		self.insert(margin.0.name.to_span(), SemanticKind::AtKeyword, SemanticModifier::none());
	}

	fn visit_webkit_keyframes_rule<'a>(&mut self, webkit: &WebkitKeyframesRule<'a>) {
		// Strike through the entire deprecated rule
		self.insert(webkit.to_span(), SemanticKind::AtKeyword, SemanticModifier::Deprecated);
	}

	fn visit_document_rule<'a>(&mut self, document: &DocumentRule<'a>) {
		// Strike through the entire deprecated rule
		self.insert(document.to_span(), SemanticKind::AtKeyword, SemanticModifier::Deprecated);
	}

	fn visit_moz_document_rule<'a>(&mut self, moz: &MozDocumentRule<'a>) {
		// Strike through the entire deprecated rule
		self.insert(moz.to_span(), SemanticKind::AtKeyword, SemanticModifier::Deprecated);
	}

	fn visit_color(&mut self, color: &Color) {
		if let Some(bg) = color.to_chromashift() {
			let swatch = SemanticDecoration::BackgroundColor(bg.into());
			self.insert_with_decoration(
				color.to_span(),
				SemanticKind::StyleValueColor,
				SemanticModifier::none(),
				swatch,
			);
		} else {
			self.insert(color.to_span(), SemanticKind::StyleValueColor, SemanticModifier::none());
		}
	}

	fn visit_url(&mut self, url: &Url) {
		self.insert(url.to_span(), SemanticKind::StyleValueUrl, SemanticModifier::none());
	}

	fn visit_pseudo_element(&mut self, element: &PseudoElement) {
		self.insert(element.to_span(), SemanticKind::PseudoElement, SemanticModifier::none());
	}

	// Value types - only add methods that actually exist in the Visit trait
	fn visit_length(&mut self, length: &Length) {
		self.insert(length.to_span(), SemanticKind::StyleValueDimension, SemanticModifier::none());
	}

	fn visit_length_percentage(&mut self, length_pct: &LengthPercentage) {
		self.insert(length_pct.to_span(), SemanticKind::StyleValueDimension, SemanticModifier::none());
	}

	fn visit_angle(&mut self, angle: &Angle) {
		self.insert(angle.to_span(), SemanticKind::StyleValueDimension, SemanticModifier::none());
	}

	fn visit_time(&mut self, time: &Time) {
		self.insert(time.to_span(), SemanticKind::StyleValueDimension, SemanticModifier::none());
	}

	fn visit_css_int(&mut self, int: &CSSInt) {
		self.insert(int.to_span(), SemanticKind::StyleValueNumber, SemanticModifier::none());
	}

	fn visit_flex(&mut self, flex: &Flex) {
		self.insert(flex.to_span(), SemanticKind::StyleValueDimension, SemanticModifier::none());
	}
}
