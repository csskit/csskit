use proc_macro2::{Ident, Span};
use quote::format_ident;
use syn::{Fields, GenericArgument, Index, Member, PathArguments, Type};

/// A normalised view of a single struct or variant field.
#[derive(Debug)]
pub struct FieldView<'a> {
	/// Binding identifier for use in generated code (`v0`, `v1`, or the
	/// field's own name for named fields).
	pub binding: Ident,
	/// Member accessor for `self.#member` expressions (`0`, `1`, or the name).
	pub member: Member,
	/// The field's type with any leading `&` reference stripped.
	pub ty: &'a Type,
	/// `true` when `ty` is `Option<T>`.
	pub is_option: bool,
}

/// Extension trait on `syn::Fields` to build field views.
pub trait FieldsExt {
	fn views(&self) -> Vec<FieldView<'_>>;
}

impl FieldsExt for Fields {
	fn views(&self) -> Vec<FieldView<'_>> {
		self.iter()
			.enumerate()
			.map(|(i, field)| {
				let binding = field.ident.clone().unwrap_or_else(|| format_ident!("v{}", i));
				let member: Member = match &field.ident {
					Some(name) => Member::Named(name.clone()),
					None => Member::Unnamed(Index { index: i as u32, span: Span::call_site() }),
				};
				let ty: &Type = match &field.ty {
					Type::Reference(r) => r.elem.as_ref(),
					t => t,
				};
				let is_option = option_inner(ty).is_some();
				FieldView { binding, member, ty, is_option }
			})
			.collect()
	}
}

/// If `ty` is `Option<T>`, return `Some(&T)`. Otherwise `None`.
pub(crate) fn option_inner(ty: &Type) -> Option<&Type> {
	if let Type::Path(path) = ty
		&& let Some(seg) = path.path.segments.last()
		&& seg.ident == "Option"
		&& let PathArguments::AngleBracketed(args) = &seg.arguments
		&& let Some(GenericArgument::Type(inner)) = args.args.first()
	{
		return Some(inner);
	}
	None
}
