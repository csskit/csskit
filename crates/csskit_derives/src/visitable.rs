use crate::{FieldsExt, WhereCollector, field_view::Arm};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DeriveInput, Error, Fields, Ident, Meta, Result,
	parse::{Parse, ParseStream},
	parse_quote,
	token::SelfValue,
};

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

/// Returns true if `#[derive(... FeatureMetadata ...)]` is present on the type,
/// indicating `visit_feature`/`exit_feature` calls should be emitted in `accept()`.
fn has_feature_metadata(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| {
		if !attr.path().is_ident("derive") {
			return false;
		}
		let Meta::List(list) = &attr.meta else { return false };
		list.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
			.map(|paths| paths.iter().any(|p| p.is_ident("FeatureMetadata")))
			.unwrap_or(false)
	})
}

/// A `match` over every arm which calls `visit` on each field that is not skipped.
fn visit_children(arms: &[Arm], visit: impl Fn(&Ident) -> TokenStream) -> TokenStream {
	let arms = arms.iter().map(|arm| {
		let skip_arm = VisitStyle::from(arm.attrs) == VisitStyle::Skip;
		let visited: Vec<bool> = arm
			.fields
			.iter()
			.map(|field| !skip_arm && VisitStyle::from(field.attrs.as_slice()) != VisitStyle::Skip)
			.collect();
		let pattern = arm.pattern(|i, view| visited[i].then(|| view.binding.clone()));
		let calls = arm
			.fields
			.views()
			.into_iter()
			.zip(&visited)
			.filter(|(_, visited)| **visited)
			.map(|(view, _)| visit(&view.binding));
		quote! { #pattern => { #(#calls;)* } }
	});
	quote! { match self { #(#arms)* } }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if let Data::Struct(s) = &input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive Visitable on this struct"));
	}
	let arms = Arm::all(&input)?;

	let style = VisitStyle::from(input.attrs.as_slice());
	let ident = &input.ident;
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();
	let visit_method = format_ident!("visit_{}", ident.to_string().to_snake_case());
	let exit_method = format_ident!("exit_{}", ident.to_string().to_snake_case());

	let mut wc = WhereCollector::new();
	let (children_mut, children) = if style.visit_children() {
		for arm in &arms {
			if VisitStyle::from(arm.attrs) != VisitStyle::Skip {
				for field in
					arm.fields.iter().filter(|field| VisitStyle::from(field.attrs.as_slice()) != VisitStyle::Skip)
				{
					wc.add(&field.ty);
				}
			}
		}
		(
			visit_children(&arms, |binding| quote! { #binding.accept_mut(v) }),
			visit_children(&arms, |binding| quote! { visit_flow::try_visit!(#binding.accept(v)) }),
		)
	} else {
		(quote! {}, quote! {})
	};

	let (visit_feature, exit_feature) = if has_feature_metadata(&input.attrs) {
		(quote! { v.visit_feature(self); }, quote! { v.exit_feature(self); })
	} else {
		(quote! {}, quote! {})
	};

	let (accept_mut_body, accept_body) = if style.visit_self() {
		(
			quote! {
				v.#visit_method(self);
				#children_mut
				v.#exit_method(self);
			},
			quote! {
				let __node = crate::QueryableNode::visit_node(self);
				if let visit_flow::VisitAction::SkipChildren = visit_flow::try_visit!(v.consider_node(__node)) {
					return <visit_flow::VisitFlow as visit_flow::VisitFlowExt>::DESCEND;
				}
				#visit_feature
				if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.enter_node(__node)) {
					if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.#visit_method(self)) {
						#children
					}
					visit_flow::try_visit!(v.#exit_method(self));
				}
				visit_flow::try_visit!(v.exit_node(__node));
				#exit_feature
			},
		)
	} else {
		(children_mut, children)
	};

	let mut_where_clause = wc.extend_where_clause(&input.generics, parse_quote! { crate::VisitableMut });
	// Any type parameter that must be `Visitable` for this impl's fields must also satisfy
	// `ToSpan` and `NodeWithMetadata<CssMetadata>`. This is required directly when this node is
	// itself queryable (`accept()`'s body calls `QueryableNode::visit_node(self)`, and
	// `QueryableNode: ToSpan + NodeWithMetadata<CssMetadata>`, see visit/mod.rs) - but it's also
	// required transitively whenever a *field*'s type is queryable over the same `T` (that
	// field's own `Visitable` impl already carries this same requirement on `T`, so callers like
	// this one, invoking `field.accept(v)`, must prove it too). Since field queryability isn't
	// visible from here, bundle all three bounds together unconditionally for every `T` that
	// needs `Visitable` at all.
	let where_clause = wc.extend_where_clause(
		&input.generics,
		parse_quote! { crate::Visitable + ::css_parse::ToSpan + css_parse::NodeWithMetadata<crate::CssMetadata> },
	);

	let queryable_impl = (style.visit_self() && !has_queryable_skip(&input.attrs)).then(|| {
		quote! {
			#[automatically_derived]
			impl #impl_generics crate::QueryableNode for #ident #type_generics #where_clause {
				const NODE_ID: crate::NodeId = crate::NodeId::#ident;
			}
		}
	});

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics crate::VisitableMut for #ident #type_generics #mut_where_clause {
			fn accept_mut<__V: crate::VisitMut>(&mut self, v: &mut __V) {
				use crate::VisitableMut;
				#accept_mut_body
			}
		}

		#[automatically_derived]
		impl #impl_generics crate::Visitable for #ident #type_generics #where_clause {
			fn accept<__V: crate::Visit>(&self, v: &mut __V) -> visit_flow::VisitFlow {
				use crate::Visitable;
				#accept_body
				<visit_flow::VisitFlow as visit_flow::VisitFlowExt>::DESCEND
			}
		}

		#queryable_impl
	})
}
