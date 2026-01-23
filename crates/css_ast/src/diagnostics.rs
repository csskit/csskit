use css_parse::{Diagnostic, DiagnosticMeta, SourceCursor};

pub trait CssDiagnostic {
	fn unimplemented(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unexpected_pseudo_class(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unexpected_pseudo_element(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unexpected_at_rule(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unexpected_function(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn expected_unsigned(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn number_out_of_bounds(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn expected_int(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unexpected_zero(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn reserved_keyframe_name(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn non_negative(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn positive(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
}

impl CssDiagnostic for Diagnostic {
	fn unimplemented(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "Unimplemented",
			message: "This cannot yet be parsed by the parser :(".into(),
			help: "This feature needs to be implemented within csskit. This file won't parse without it.".into(),
			labels: vec![],
		}
	}

	fn unexpected_pseudo_class(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "UnexpectedPseudo",
			message: format!("Unexpected pseudo selector ':{}'", SourceCursor::from(diagnostic.start_cursor, source)),
			help: "This isn't a valid psuedo selector for this rule.".into(),
			labels: vec![],
		}
	}

	fn unexpected_pseudo_element(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let text = if start + len <= source.len() { &source[start..start + len] } else { "<unknown>" };
		DiagnosticMeta {
			code: "UnexpectedPseudoElement",
			message: format!("Unexpected pseudo element '::{text}'"),
			help: "This isn't a valid psuedo selector for this rule.".into(),
			labels: vec![],
		}
	}

	fn unexpected_at_rule(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let text = if start + len <= source.len() { &source[start..start + len] } else { "<unknown>" };
		DiagnosticMeta {
			code: "UnexpectedAtRule",
			message: format!("Unexpected at rule '@{text}'"),
			help: "This isn't a recognisable at-rule here. If the rule is valid, it might not be allowed here.".into(),
			labels: vec![],
		}
	}

	fn unexpected_function(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let text = if start + len <= source.len() { &source[start..start + len] } else { "<unknown>" };
		DiagnosticMeta {
			code: "UnexpectedFunction",
			message: format!("Unexpected function '{text}'()"),
			help: "A function with this name wasn't expected in this position.".into(),
			labels: vec![],
		}
	}

	fn expected_unsigned(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "ExpectedUnsigned",
			message: format!("Expected an unsigned number but saw `{}`", diagnostic.start_cursor.token().value()),
			help: "This number cannot have a + or a -".into(),
			labels: vec![],
		}
	}

	fn number_out_of_bounds(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "NumberOutOfBounds",
			message: "This number is out of bounds.".into(),
			help: "This needs to be within the valid range.".into(),
			labels: vec![],
		}
	}

	fn expected_int(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "ExpectedInt",
			message: "This value isn't allowed to have a fraction, it must be a whole number.".into(),
			help: "Try using a whole number instead".into(),
			labels: vec![],
		}
	}

	fn unexpected_zero(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "ExpectedZero",
			message: "This number must not be 0.".into(),
			help: "Try replacing it with a positive or negative number".into(),
			labels: vec![],
		}
	}

	fn reserved_keyframe_name(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let start = cursor.offset().0 as usize;
		let len = cursor.token().len() as usize;
		let text = if start + len <= source.len() { &source[start..start + len] } else { "<unknown>" };
		DiagnosticMeta {
			code: "ReservedKeyframeName",
			message: format!("{text} cannot be used as a keyframe name, as it's a reserved word."),
			help: "Rename it, or try wrapping it in quotes".into(),
			labels: vec![],
		}
	}

	fn non_negative(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "NonNegative",
			message: format!("Value must be non-negative, but saw `{}`", diagnostic.start_cursor.token().value()),
			help: "This property requires a value >= 0. Use a non-negative value.".into(),
			labels: vec![],
		}
	}

	fn positive(diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "Positive",
			message: format!("Value must be positive, but saw `{}`", diagnostic.start_cursor.token().value()),
			help: "This property requires a value > 0. Use a positive value.".into(),
			labels: vec![],
		}
	}
}
