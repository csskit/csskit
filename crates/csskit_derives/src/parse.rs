//! `derive(Parse)`.
//!
//! A struct parses its fields: in order, or for `#[parse(all_must_occur)]` / `one_must_occur` in
//! any order until no field matches. An enum parses as the first variant whose leading tokens
//! match. Variants are keyed by their start, the type and atom of their next field; variants which
//! share a start are told apart after consuming it, on the field after, and a variant with no field
//! left is the fallback for the ones which go on. Variants whose fields come in any order have no
//! single start, so they are tried in turn, rewinding after each failure.

use crate::{
	FieldsExt, WhereCollector,
	attributes::{Atom, FieldParseMode, extract_atom},
	darling_ext::{StateArg, StopArg},
	ensure_lifetime_a,
	field_view::option_inner,
};
use darling::FromAttributes;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Member, Result, Type, parse_quote};

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(parse))]
struct ParseArg {
	pub state: Option<StateArg>,
	pub stop: Option<StopArg>,
	#[darling(default)]
	pub all_must_occur: bool,
	#[darling(default)]
	pub one_must_occur: bool,
}

impl ParseArg {
	fn parse_mode(&self) -> FieldParseMode {
		match (self.all_must_occur, self.one_must_occur) {
			(true, _) => FieldParseMode::AllMustOccur,
			(_, true) => FieldParseMode::OneMustOccur,
			_ => FieldParseMode::Sequential,
		}
	}
}

struct Field {
	var: Ident,
	member: Member,
	/// The type parsed, with any `Option` removed.
	ty: Type,
	optional: bool,
	atom: Option<Atom>,
	state: Option<Ident>,
}

impl Field {
	/// Every field, with `atom` given to the first field when it has none of its own.
	fn all(fields: &Fields, atom: Option<Atom>) -> Result<Vec<Self>> {
		fields
			.views()
			.into_iter()
			.zip(fields.iter())
			.enumerate()
			.map(|(i, (view, field))| {
				let arg = ParseArg::from_attributes(&field.attrs)?;
				Ok(Self {
					var: format_ident!("v{i}"),
					member: view.member,
					ty: option_inner(view.ty).unwrap_or(view.ty).clone(),
					optional: view.is_option,
					atom: extract_atom(&field.attrs)?.or_else(|| if i == 0 { atom.clone() } else { None }),
					state: arg.state.map(|StateArg(ident)| ident),
				})
			})
			.collect()
	}

	/// `p.parse::<T>()?` under this field's state, if any.
	fn parse(&self, ty: TokenStream) -> TokenStream {
		match &self.state {
			Some(state) => quote! {{
				let old_state = p.set_state(State::#state);
				let result = p.parse::<#ty>()?;
				p.set_state(old_state);
				result
			}},
			None => quote! { p.parse::<#ty>()? },
		}
	}

	/// `let v = ...;` which parses this field next. With `atom_known` the atom has been matched
	/// already and is not checked again.
	fn step(&self, atom_known: bool, wc: &mut WhereCollector) -> TokenStream {
		let Self { var, ty, .. } = self;
		wc.add(ty);
		match (&self.atom, self.optional) {
			(Some(_), _) if atom_known => {
				let parse = self.parse(quote! { #ty });
				quote! { let #var = #parse; }
			}
			(Some(atom), true) => {
				let condition = atom.equals_atom(format_ident!("c"));
				let parse = self.parse(quote! { #ty });
				quote! { let #var = { let c = p.peek_n(1); if #condition { Some(#parse) } else { None } }; }
			}
			(Some(atom), false) => {
				let condition = atom.equals_atom(format_ident!("c"));
				let parse = self.parse(quote! { #ty });
				quote! {
					let #var = {
						let c = p.peek_n(1);
						if #condition { #parse } else { return Err(::css_parse::Diagnostic::new(c, ::css_parse::Diagnostic::unexpected))?; }
					};
				}
			}
			(None, true) => {
				let parse = self.parse(quote! { Option<#ty> });
				quote! { let #var = #parse; }
			}
			(None, false) => {
				let parse = self.parse(quote! { #ty });
				quote! { let #var = #parse; }
			}
		}
	}

	fn binding(&self) -> TokenStream {
		let Self { var, ty, .. } = self;
		quote! { let mut #var: Option<#ty> = None; }
	}

	/// Whether the cursor `c` starts this field.
	fn peek(&self) -> TokenStream {
		let ty = &self.ty;
		match &self.atom {
			Some(atom) => {
				let path = atom.path();
				let set = atom.first_segment();
				quote! { p.peek::<#ty>() && p.to_atom::<#set>(c) == #path }
			}
			None => quote! { p.peek::<#ty>() },
		}
	}

	/// One step of an any-order loop: parse this field if it is not yet seen and starts here.
	fn occur(&self, wc: &mut WhereCollector) -> TokenStream {
		let Self { var, ty, .. } = self;
		wc.add(ty);
		let peek = self.peek();
		quote! {
			if #var.is_none() && #peek {
				#var = Some(p.parse::<#ty>()?);
				continue;
			}
		}
	}
}

struct Variant {
	path: TokenStream,
	mode: FieldParseMode,
	fields: Vec<Field>,
}

impl Variant {
	fn is_ordered(&self) -> bool {
		self.mode == FieldParseMode::Sequential
	}

	fn all_optional(&self) -> bool {
		self.fields.iter().all(|field| field.optional)
	}

	fn construct(&self, post: &TokenStream, value: impl Fn(&Field) -> TokenStream) -> TokenStream {
		let path = &self.path;
		let members = self.fields.iter().map(|field| &field.member);
		let values = self.fields.iter().map(value);
		quote! { #post return Ok(#path { #(#members: #values),* }); }
	}

	/// Parses fields `from` onwards in order, then constructs.
	fn ordered(&self, from: usize, post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
		let steps = self.fields[from..].iter().map(|field| field.step(false, wc));
		let construct = self.construct(post, |field| field.var.to_token_stream());
		quote! { #(#steps)* #construct }
	}

	/// Parses fields in any order until none matches, checks the ones which must occur, then
	/// constructs.
	fn any_order(&self, post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
		let bindings = self.fields.iter().map(Field::binding);
		let steps = self.fields.iter().map(|field| field.occur(wc));
		let none = |field: &Field| {
			let var = &field.var;
			quote! { #var.is_none() }
		};
		let missing = match self.mode {
			FieldParseMode::AllMustOccur if !self.all_optional() => {
				let required = self.fields.iter().filter(|field| !field.optional).map(none);
				quote! { #(#required)||* }
			}
			_ => {
				let all = self.fields.iter().map(none);
				quote! { #(#all)&&* }
			}
		};
		let construct = self.construct(post, |field| {
			let var = &field.var;
			if field.optional {
				quote! { #var }
			} else {
				quote! { #var.unwrap() }
			}
		});
		quote! {
			#(#bindings)*
			loop {
				let c = p.peek_n(1);
				#(#steps)*
				break;
			}
			if #missing {
				let c = p.peek_n(1);
				Err(::css_parse::Diagnostic::new(c, ::css_parse::Diagnostic::unexpected))?
			}
			#construct
		}
	}

	fn body(&self, post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
		if self.is_ordered() { self.ordered(0, post, wc) } else { self.any_order(post, wc) }
	}
}

fn unexpected_at_next() -> TokenStream {
	quote! { return Err(::css_parse::Diagnostic::new(p.peek_n(1), ::css_parse::Diagnostic::unexpected))?; }
}

/// The ordered variants which still compete after `depth` fields, keyed by the start of their next
/// field. Variants keyed on the same type share one peek and are told apart by atom; a variant
/// whose next field is optional has no single start and stands alone.
enum Node<'a> {
	Keyed { ty: &'a Type, set: Option<Ident>, arms: Vec<(Option<Atom>, Vec<&'a Variant>)> },
	Alone(&'a Variant),
}

/// Dispatch between ordered variants once `depth` fields are consumed. Returns or errors unless
/// `depth` is zero, where a miss falls through.
fn dispatch(variants: &[&Variant], depth: usize, post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
	if let [variant] = variants
		&& depth > 0
	{
		return variant.ordered(depth, post, wc);
	}
	let mut nodes: Vec<Node> = vec![];
	let mut done: Option<&Variant> = None;
	for &variant in variants {
		match variant.fields.get(depth) {
			None => done = done.or(Some(variant)),
			Some(field) if field.optional => nodes.push(Node::Alone(variant)),
			Some(field) => {
				let set = field.atom.as_ref().map(Atom::first_segment);
				let keyed = nodes.iter_mut().find_map(|node| match node {
					Node::Keyed { ty, set: seen, arms } if **ty == field.ty && *seen == set => Some(arms),
					_ => None,
				});
				let arms = match keyed {
					Some(arms) => arms,
					None => {
						nodes.push(Node::Keyed { ty: &field.ty, set, arms: vec![] });
						let Some(Node::Keyed { arms, .. }) = nodes.last_mut() else { unreachable!() };
						arms
					}
				};
				match arms.iter_mut().find(|(atom, _)| *atom == field.atom) {
					Some((_, group)) => group.push(variant),
					None => arms.push((field.atom.clone(), vec![variant])),
				}
			}
		}
	}

	// Keywords are tried before open types, which may accept them too, as `<custom-ident>` does.
	nodes.sort_by_key(|node| !matches!(node, Node::Keyed { set: Some(_), .. }));
	let nodes = nodes.iter().map(|node| match node {
		Node::Alone(variant) => {
			let body = variant.ordered(depth, post, wc);
			let rest = &variant.fields[depth..];
			let Some(required) = rest.iter().position(|field| !field.optional) else {
				return body;
			};
			let starts = rest[..=required].iter().map(|field| {
				wc.add(&field.ty);
				field.peek()
			});
			quote! { { let c = p.peek_n(1); if #(#starts)||* { #body } } }
		}
		Node::Keyed { ty, set, arms } => {
			wc.add(ty);
			let mut consume = |group: &[&Variant]| {
				let step = group[0].fields[depth].step(true, wc);
				let rest = dispatch(group, depth + 1, post, wc);
				quote! { #step #rest }
			};
			let plain = arms.iter().find(|(atom, _)| atom.is_none()).map(|(_, group)| consume(group));
			if set.is_none() {
				quote! { if p.peek::<#ty>() { #plain } }
			} else {
				let keyed = arms.iter().filter_map(|(atom, group)| {
					let path = atom.as_ref()?.path();
					let body = consume(group);
					Some(quote! { #path => { #body } })
				});
				quote! {
					if p.peek::<#ty>() {
						let c = p.peek_n(1);
						match p.to_atom::<#set>(c) {
							#(#keyed)*
							_ => { #plain }
						}
					}
				}
			}
		}
	});
	let nodes: Vec<TokenStream> = nodes.collect();
	let miss = match done {
		Some(variant) => variant.construct(post, |field| field.var.to_token_stream()),
		None if depth > 0 => unexpected_at_next(),
		None => quote! {},
	};
	quote! { #(#nodes)* #miss }
}

/// True when `ty` is a substitution value-slot wrapper (`Value<..>` / `CalcableValue<..>`).
fn is_value_slot(ty: &Type) -> bool {
	matches!(ty, Type::Path(path) if path.path.segments.last().is_some_and(|s| s.ident == "Value" || s.ident == "CalcableValue"))
}

/// Runs `body`, which returns or errors, and rewinds to where it began when it errors.
fn attempt(body: TokenStream) -> TokenStream {
	quote! {
		let checkpoint = p.checkpoint();
		if let Ok(v) = (|| -> ::css_parse::Result<Self> { #body })() {
			return Ok(v);
		}
		p.rewind(checkpoint);
	}
}

/// Value slots peek broadly (any substitution or math function), so an enum with several cannot
/// pick the right one for a leading function by peek alone: try each variant which starts with
/// one in turn, rewinding on failure. Typed slots come before the keyword slot, which is one
/// `Value<T![Ident]>` like any other, so `var(--a, small)` lands in the typed slot when there is
/// one.
fn substitution_predispatch(variants: &[Variant], post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
	let slots = variants.iter().filter(|v| v.is_ordered() && v.fields.first().is_some_and(|f| is_value_slot(&f.ty)));
	let (keyword, typed): (Vec<_>, Vec<_>) = slots.partition(|v| v.fields[0].atom.is_some());
	let attempts: Vec<TokenStream> = typed
		.into_iter()
		.chain(keyword.into_iter().take(1))
		.map(|variant| attempt(variant.ordered(0, post, wc)))
		.collect();
	if attempts.is_empty() {
		quote! {}
	} else {
		quote! { if p.peek_n(1) == ::css_parse::Kind::Function { #(#attempts)* } }
	}
}

/// Sibling any-order variants whose fields are all optional and whose atoms overlap, such as
/// `[ filled | open ] || [ dot | sesame ]`, cannot be tried one at a time: the first would stop at
/// a token a sibling accepts and succeed with it unconsumed. One loop accepts every field of every
/// sibling, and the first atom which belongs to one sibling alone decides which is built.
fn shared_loop(variants: &[&Variant], post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
	let mut union: Vec<(&Field, usize)> = vec![];
	let same = |a: &Field, b: &Field| a.atom == b.atom && a.ty == b.ty;
	for (i, variant) in variants.iter().enumerate() {
		for field in &variant.fields {
			if !union.iter().any(|(seen, _)| same(seen, field)) {
				union.push((field, i));
			}
		}
	}
	let var = |field: &Field| {
		let i = union.iter().position(|(seen, _)| same(seen, field)).expect("field is in union");
		format_ident!("u{i}")
	};
	let owners = |field: &Field| variants.iter().filter(|v| v.fields.iter().any(|f| same(f, field))).count();

	let bindings = union.iter().map(|(field, _)| {
		let var = var(field);
		let ty = &field.ty;
		quote! { let mut #var: Option<#ty> = None; }
	});
	let steps = union.iter().map(|(field, i)| {
		wc.add(&field.ty);
		let var = var(field);
		let ty = &field.ty;
		let peek = field.peek();
		let decides = field.atom.is_some() && owners(field) == 1;
		let guard = decides.then(|| quote! { && _alternative == 0 });
		let decide = decides.then(|| {
			let alternative = i + 1;
			quote! { _alternative = #alternative; }
		});
		quote! {
			if #var.is_none() #guard && #peek {
				#decide
				#var = Some(p.parse::<#ty>()?);
				continue;
			}
		}
	});
	let none = union.iter().map(|(field, _)| {
		let var = var(field);
		quote! { #var.is_none() }
	});
	let arms = variants.iter().enumerate().map(|(i, variant)| {
		let alternative = i + 1;
		let construct = variant.construct(post, |field| var(field).to_token_stream());
		quote! { #alternative => { #construct } }
	});
	let first = variants[0].construct(post, |field| var(field).to_token_stream());
	let unexpected = unexpected_at_next();
	quote! {
		#(#bindings)*
		let mut _alternative: usize = 0;
		loop {
			let c = p.peek_n(1);
			#(#steps)*
			break;
		}
		if #(#none)&&* { #unexpected }
		match _alternative {
			#(#arms)*
			_ => { #first }
		}
	}
}

fn enum_body(variants: &[Variant], post: &TokenStream, wc: &mut WhereCollector) -> TokenStream {
	let predispatch = substitution_predispatch(variants, post, wc);
	let ordered: Vec<&Variant> = variants.iter().filter(|v| v.is_ordered()).collect();
	let any_order: Vec<&Variant> = variants.iter().filter(|v| !v.is_ordered()).collect();
	let dispatch = dispatch(&ordered, 0, post, wc);

	let overlapping = any_order.len() > 1
		&& any_order.iter().all(|v| v.all_optional())
		&& any_order.iter().enumerate().any(|(i, v)| {
			any_order.iter().enumerate().any(|(j, w)| {
				i != j && v.fields.iter().any(|f| f.atom.is_some() && w.fields.iter().any(|g| g.atom == f.atom))
			})
		});
	let tail = if overlapping {
		shared_loop(&any_order, post, wc)
	} else if let Some((last, rest)) = any_order.split_last() {
		let attempts: Vec<TokenStream> = rest.iter().map(|variant| attempt(variant.any_order(post, wc))).collect();
		let last = last.any_order(post, wc);
		quote! { #(#attempts)* #last }
	} else {
		unexpected_at_next()
	};
	quote! { #predispatch #dispatch #tail }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut wc = WhereCollector::new();
	let ident = &input.ident;
	let generics = &input.generics;
	let generic_with_a = ensure_lifetime_a(generics);
	let (impl_generics, _, _) = generic_with_a.split_for_impl();
	let (_, type_generics, _) = generics.split_for_impl();

	let mut pre = quote! {};
	let mut post = quote! {};
	let arg = ParseArg::from_attributes(&input.attrs)?;
	if let Some(StateArg(state)) = &arg.state {
		pre = quote! { let state = p.set_state(State::#state); #pre };
		post = quote! { #post p.set_state(state); };
	}
	if let Some(stop) = &arg.stop {
		let variant = &stop.variant;
		let set = if stop.prefix == "Kind" {
			quote! { KindSet::new(&[Kind::#variant]) }
		} else {
			quote! { KindSet::#variant }
		};
		pre = quote! { let stop = p.set_stop(#set); #pre };
		post = quote! { #post p.set_stop(stop); };
	}

	let body = match &input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Parse on a Union")),
		Data::Struct(data) => {
			let variant =
				Variant { path: quote! { Self }, mode: arg.parse_mode(), fields: Field::all(&data.fields, None)? };
			variant.body(&post, &mut wc)
		}
		Data::Enum(data) => {
			let variants = data
				.variants
				.iter()
				.map(|variant| {
					if variant.fields.is_empty() {
						return Err(Error::new(variant.ident.span(), "enum variant must have at least one field"));
					}
					let ident = &variant.ident;
					Ok(Variant {
						path: quote! { Self::#ident },
						mode: ParseArg::from_attributes(&variant.attrs)?.parse_mode(),
						fields: Field::all(&variant.fields, extract_atom(&variant.attrs)?)?,
					})
				})
				.collect::<Result<Vec<_>>>()?;
			enum_body(&variants, &post, &mut wc)
		}
	};

	let where_clause =
		wc.extend_where_clause(generics, parse_quote! { ::css_parse::Parse<'a> + ::css_parse::Peek<'a> });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Parse<'a> for #ident #type_generics #where_clause {
			fn parse<I>(p: &mut css_parse::Parser<'a, I>) -> css_parse::Result<Self>
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
			{
				use css_parse::{Parse, Peek};
				#pre
				#body
			}
		}
	})
}
