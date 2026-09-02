use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Error, Fields, GenericArgument, Index, Member, PathArguments, Result, Type};

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

/// A struct, or one variant of an enum: the path which names it in a pattern, its attributes, and
/// its fields.
pub struct Arm<'a> {
	pub path: TokenStream,
	#[allow(dead_code)]
	pub attrs: &'a [Attribute],
	pub fields: &'a Fields,
}

impl<'a> Arm<'a> {
	/// Every arm of a derive input: one for a struct, one per variant for an enum.
	pub fn all(input: &'a DeriveInput) -> Result<Vec<Self>> {
		match &input.data {
			Data::Union(_) => Err(Error::new(input.ident.span(), "Cannot derive on a Union")),
			Data::Struct(data) => Ok(vec![Self { path: quote! { Self }, attrs: &input.attrs, fields: &data.fields }]),
			Data::Enum(data) => Ok(data
				.variants
				.iter()
				.map(|variant| {
					let ident = &variant.ident;
					Self { path: quote! { Self::#ident }, attrs: &variant.attrs, fields: &variant.fields }
				})
				.collect()),
		}
	}

	/// The pattern which binds each field to the identifier `bind` gives it, and ignores the fields
	/// `bind` gives none.
	pub fn pattern(&self, bind: impl Fn(usize, &FieldView) -> Option<Ident>) -> TokenStream {
		let path = &self.path;
		let views = self.fields.views();
		let bindings = views.iter().enumerate().map(|(i, view)| bind(i, view));
		match self.fields {
			Fields::Unit => quote! { #path },
			Fields::Named(_) => {
				let fields = views.iter().zip(bindings).filter_map(|(view, binding)| {
					let binding = binding?;
					let name = &view.member;
					Some(if view.binding == binding {
						quote! { #binding }
					} else {
						quote! { #name: #binding }
					})
				});
				quote! { #path { #(#fields,)* .. } }
			}
			Fields::Unnamed(_) => {
				let bindings =
					bindings.map(|binding| binding.map_or_else(|| quote! { _ }, |binding| quote! { #binding }));
				quote! { #path(#(#bindings),*) }
			}
		}
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
