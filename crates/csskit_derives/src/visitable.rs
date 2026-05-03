use crate::{FieldsExt, WhereCollector};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DeriveInput, Error, Fields, Ident, Meta, Result,
	parse::{Parse, ParseStream},
	parse_quote,
	token::SelfValue,
};
use synstructure::{AddBounds, Structure};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum VisitStyle {
	All,
	Skip,
	OnlySelf,
	#[default]
	OnlyChildren,
}

impl VisitStyle {
	pub fn visit_self(&self) -> bool {
		matches!(self, Self::All | Self::OnlySelf)
	}
	pub fn visit_children(&self) -> bool {
		matches!(self, Self::All | Self::OnlyChildren)
	}
}

impl Parse for VisitStyle {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.parse::<SelfValue>().is_ok() {
			return Ok(Self::OnlySelf);
		}
		match input.parse::<Ident>()? {
			i if i == "all" => Ok(Self::All),
			i if i == "skip" => Ok(Self::Skip),
			i if i == "children" => Ok(Self::OnlyChildren),
			ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
		}
	}
}

impl From<&[Attribute]> for VisitStyle {
	fn from(attrs: &[Attribute]) -> Self {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("visit")) {
			match meta {
				Meta::List(meta) => meta.parse_args::<VisitStyle>().unwrap(),
				_ => Self::All,
			}
		} else {
			Self::default()
		}
	}
}

fn has_queryable_skip(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| {
		if attr.path().is_ident("queryable") {
			match &attr.meta {
				Meta::List(meta) => meta.parse_args::<Ident>().map(|i| i == "skip").unwrap_or(false),
				_ => false,
			}
		} else {
			false
		}
	})
}

fn make_body(s: &Structure, accept: &syn::Ident, wc: &mut WhereCollector) -> TokenStream {
	match &s.ast().data {
		Data::Struct(ds) => {
			let steps: Vec<TokenStream> = ds
				.fields
				.views()
				.into_iter()
				.zip(ds.fields.iter())
				.filter_map(|(view, syn_field)| {
					if VisitStyle::from(syn_field.attrs.as_slice()) == VisitStyle::Skip {
						return None;
					}
					wc.add(&syn_field.ty);
					let m = &view.member;
					Some(quote! { self.#m.#accept(v); })
				})
				.collect();
			quote! { #(#steps)* }
		}
		Data::Enum(_) => {
			let arms: TokenStream = s
				.variants()
				.iter()
				.map(|variant| {
					let var_ident = variant.ast().ident;
					let skip_variant = VisitStyle::from(variant.ast().attrs) == VisitStyle::Skip;
					let bindings: Vec<_> = variant.bindings().iter().collect();
					let named = bindings.first().and_then(|bi| bi.ast().ident.as_ref()).is_some();

					let (patterns, calls): (Vec<TokenStream>, Vec<TokenStream>) = bindings
						.iter()
						.map(|bi| {
							let skip_field =
								skip_variant || VisitStyle::from(bi.ast().attrs.as_slice()) == VisitStyle::Skip;
							let binding = &bi.binding;
							if named {
								let field_name = bi.ast().ident.as_ref().unwrap();
								if skip_field {
									(quote! { #field_name: _ }, quote! {})
								} else {
									wc.add(&bi.ast().ty);
									(quote! { #field_name: #binding }, quote! { #binding.#accept(v) })
								}
							} else if skip_field {
								(quote! { _ }, quote! {})
							} else {
								wc.add(&bi.ast().ty);
								(quote! { #binding }, quote! { #binding.#accept(v) })
							}
						})
						.unzip();

					let pattern = if bindings.is_empty() {
						quote! { Self::#var_ident }
					} else if named {
						quote! { Self::#var_ident { #(#patterns),* } }
					} else {
						quote! { Self::#var_ident(#(#patterns),*) }
					};
					quote! { #pattern => { #(#calls;)* }, }
				})
				.collect();
			quote! { match self { #arms } }
		}
		Data::Union(_) => unreachable!("checked above"),
	}
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if matches!(input.data, Data::Union(_)) {
		return Err(Error::new(input.ident.span(), "Cannot derive Visitable on a Union"));
	}
	if let Data::Struct(ref s) = input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive Visitable on this struct"));
	}

	let style: VisitStyle = VisitStyle::from(input.attrs.as_slice());
	let is_queryable = style.visit_self();
	let ident = &input.ident;
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	let (visit, exit) = if style.visit_self() {
		let visit_method = format_ident!("visit_{}", ident.to_string().to_snake_case());
		let exit_method = format_ident!("exit_{}", ident.to_string().to_snake_case());
		(quote! { v.#visit_method(self); }, quote! { v.#exit_method(self); })
	} else {
		(quote! {}, quote! {})
	};

	let (visit_queryable, exit_queryable) = if is_queryable {
		(quote! { v.visit_queryable_node(self); }, quote! { v.exit_queryable_node(self); })
	} else {
		(quote! {}, quote! {})
	};

	let mut s = Structure::try_new(&input)?;
	s.add_bounds(AddBounds::None);

	let mut wc = WhereCollector::new();

	let (body_mut, body) = if style.visit_children() {
		let accept_mut = format_ident!("accept_mut");
		let accept = format_ident!("accept");
		let b_mut = make_body(&s, &accept_mut, &mut wc);
		let b = make_body(&s, &accept, &mut wc);
		(b_mut, b)
	} else {
		(quote! {}, quote! {})
	};

	let where_clause = wc.extend_where_clause(&input.generics, parse_quote! { crate::Visitable });
	let mut_where_clause = wc.extend_where_clause(&input.generics, parse_quote! { crate::VisitableMut });

	let skip_queryable = has_queryable_skip(&input.attrs);

	let queryable_impl = if style.visit_self() && !skip_queryable {
		quote! {
			#[automatically_derived]
			impl #impl_generics crate::QueryableNode for #ident #type_generics #where_clause {
				const NODE_ID: crate::NodeId = crate::NodeId::#ident;
			}
		}
	} else {
		quote! {}
	};

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics crate::VisitableMut for #ident #type_generics #mut_where_clause {
			fn accept_mut<V: crate::VisitMut>(&mut self, v: &mut V) {
				use crate::VisitableMut;
				#visit
				#body_mut
				#exit
			}
		}

		#[automatically_derived]
		impl #impl_generics crate::Visitable for #ident #type_generics #where_clause {
			fn accept<V: crate::Visit>(&self, v: &mut V) {
				use crate::Visitable;
				#visit_queryable
				#visit
				#body
				#exit
				#exit_queryable
			}
		}

		#queryable_impl
	})
}
