use css_parse::{Diagnostic, DiagnosticMeta};

pub trait QueryDiagnostic {
	fn unknown_node_type(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn unknown_functional_pseudo_class(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
	fn nested_has_not_allowed(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta;
}

impl QueryDiagnostic for Diagnostic {
	fn unknown_node_type(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let name = cursor.str_slice(source);
		DiagnosticMeta {
			code: "UnknownNodeType",
			message: format!("unknown node type '{name}'"),
			help: "Use a valid CSS node type name like 'style-rule', 'media-rule', etc.".into(),
			labels: vec![],
		}
	}

	fn unknown_functional_pseudo_class(diagnostic: &Diagnostic, source: &str) -> DiagnosticMeta {
		let cursor = diagnostic.start_cursor;
		let name = cursor.str_slice(source);
		DiagnosticMeta {
			code: "UnknownFunctionalPseudoClass",
			message: format!("unknown functional pseudo-class ':{name}()'"),
			help: "Use a valid functional pseudo-class like :not(), :nth-child(), :property-type(), etc.".into(),
			labels: vec![],
		}
	}

	fn nested_has_not_allowed(_diagnostic: &Diagnostic, _source: &str) -> DiagnosticMeta {
		DiagnosticMeta {
			code: "NestedHasNotAllowed",
			message: "nested :has() is not allowed".into(),
			help: "The CSS :has() pseudo-class cannot be nested inside another :has()".into(),
			labels: vec![],
		}
	}
}
