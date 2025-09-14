use syn::{Attribute, ExprRange, Meta};

/// Extract #[in_range(...)] attribute from a field
pub fn extract_in_range(attrs: &[Attribute]) -> Option<ExprRange> {
	if let Some(Attribute { meta, .. }) = attrs.iter().find(|a| a.path().is_ident("in_range")) {
		match meta {
			Meta::List(meta) => {
				let range = meta.parse_args::<syn::Expr>().unwrap();
				if let syn::Expr::Range(range_expr) = range {
					Some(range_expr)
				} else {
					panic!("Expected range expression in #[in_range(...)]");
				}
			}
			_ => panic!("could not parse in_range meta"),
		}
	} else {
		None
	}
}
