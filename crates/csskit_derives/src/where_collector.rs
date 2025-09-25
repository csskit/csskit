use std::collections::HashSet;
use syn::{GenericParam, Generics, Type, TypeParam, WhereClause, parse_quote};

/// A collector for tracking which generic type parameters are used during derive macro generation.
/// As the derive macro generates calls, it can report which generics are actually used, and can
/// then expand the collected types into a Where clause.
pub struct WhereCollector(HashSet<String>);

impl WhereCollector {
	pub fn new() -> Self {
		Self(HashSet::new())
	}

	pub fn add(&mut self, ty: &Type) {
		match ty {
			Type::Path(type_path) => {
				if let Some(ident) = type_path.path.get_ident() {
					let ident_str = ident.to_string();
					self.0.insert(ident_str);
				}
				// Also check generic arguments in the path
				for segment in &type_path.path.segments {
					if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
						for arg in &args.args {
							if let syn::GenericArgument::Type(ty) = arg {
								self.add(ty);
							}
						}
					}
				}
			}
			Type::Reference(type_ref) => {
				self.add(&type_ref.elem);
			}
			Type::Tuple(type_tuple) => {
				for elem in &type_tuple.elems {
					self.add(elem);
				}
			}
			Type::Array(type_array) => {
				self.add(&type_array.elem);
			}
			Type::Slice(type_slice) => {
				self.add(&type_slice.elem);
			}
			_ => {}
		}
	}

	pub fn extend_where_clause(&self, generics: &mut Generics, predicate: Type) -> Option<WhereClause> {
		let (_, _, wheres) = generics.split_for_impl();
		if self.0.is_empty() {
			return wheres.cloned();
		}
		let mut wheres = if let Some(wheres) = wheres { wheres.clone() } else { parse_quote!(where) };
		for param in generics.params.iter() {
			if let GenericParam::Type(TypeParam { ident, .. }) = param
				&& self.0.contains(&ident.to_string())
			{
				wheres.predicates.push(parse_quote! { #ident: #predicate });
			}
		}
		Some(wheres)
	}
}
