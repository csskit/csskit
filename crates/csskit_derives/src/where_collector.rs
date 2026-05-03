use std::collections::HashSet;
use syn::{
	GenericParam, Generics, Type, TypeParam, TypePath, WhereClause, parse_quote,
	visit::{Visit, visit_type_path},
};

pub struct WhereCollector(HashSet<String>);

struct TypeParamCollector<'a>(&'a mut HashSet<String>);

impl<'ast> Visit<'ast> for TypeParamCollector<'_> {
	fn visit_type_path(&mut self, ty: &'ast TypePath) {
		if let Some(ident) = ty.path.get_ident() {
			self.0.insert(ident.to_string());
		}
		visit_type_path(self, ty);
	}
}

impl WhereCollector {
	pub fn new() -> Self {
		Self(HashSet::new())
	}

	pub fn add(&mut self, ty: &Type) {
		TypeParamCollector(&mut self.0).visit_type(ty);
	}

	pub fn extend_where_clause(&self, generics: &Generics, predicate: Type) -> Option<WhereClause> {
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
