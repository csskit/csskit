use crate::{
	FieldsExt, WhereCollector,
	attributes::{Atom, FieldParseMode, extract_atom},
	darling_ext::{StateArg, StopArg},
	ensure_lifetime_a,
	field_view::option_inner,
};
use darling::FromAttributes;
use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result, Type, Variant, parse_quote};

trait TypeIsOption {
	fn is_option(&self) -> bool;
	fn unpack_option(&self) -> Self;
}

impl TypeIsOption for Type {
	fn is_option(&self) -> bool {
		option_inner(self).is_some()
	}

	fn unpack_option(&self) -> Self {
		option_inner(self).cloned().unwrap_or_else(|| self.clone())
	}
}

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

#[derive(Debug)]
struct Field {
	var: Ident,
	ty: Type,
	arg: ParseArg,
	atom: Option<Atom>,
}

impl Field {
	fn parse_tokens(&self, wc: &mut WhereCollector) -> TokenStream {
		if self.atom.is_some() {
			self.parse_keyword_tokens(wc)
		} else {
			self.parse_normal_tokens(FieldParseMode::Sequential, wc)
		}
	}

	fn parse_keyword_tokens(&self, wc: &mut WhereCollector) -> TokenStream {
		let Field { var, ty, atom, .. } = self;
		let atom = atom.as_ref().expect("parse_keyword_tokens called without atom");
		let condition = atom.equals_atom(format_ident!("c"));
		let inner_ty = ty.unpack_option();
		wc.add(&inner_ty);
		if ty.is_option() {
			quote! {
				let #var = {
					let c = p.peek_n(1);
					if #condition {
						Some(p.parse::<#inner_ty>()?)
					} else {
						None
					}
				};
			}
		} else {
			quote! {
				let #var = {
					let c = p.peek_n(1);
					if #condition {
						p.parse::<#inner_ty>()?
					} else {
						return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
					}
				};
			}
		}
	}

	fn parse_normal_tokens(&self, parse_mode: FieldParseMode, wc: &mut WhereCollector) -> TokenStream {
		let Field { var, ty, arg, .. } = self;
		match parse_mode {
			FieldParseMode::Sequential => {
				wc.add(ty);
				if let Some(crate::darling_ext::StateArg(state_ident)) = &arg.state {
					quote! {
						let #var = {
							let old_state = p.set_state(State::#state_ident);
							let result = p.parse::<#ty>()?;
							p.set_state(old_state);
							result
						};
					}
				} else {
					quote! { let #var = p.parse::<#ty>()?; }
				}
			}
			FieldParseMode::AllMustOccur | FieldParseMode::OneMustOccur => {
				let inner_ty = ty.unpack_option();
				wc.add(&inner_ty);
				quote! {
					if #var.is_none() && <#inner_ty>::peek(p, c) {
						#var = Some(p.parse::<#inner_ty>()?);
						continue;
					}
				}
			}
		}
	}

	/// Emits the atom-conditional match arm used in must-occur loops.
	fn atom_match_arm(&self) -> TokenStream {
		let atom_path = self.atom.as_ref().expect("atom_match_arm called without atom").path();
		let var = &self.var;
		let inner_ty = self.ty.unpack_option();
		quote! {
			if #var.is_none() && atom == #atom_path {
				#var = Some(p.parse::<#inner_ty>()?);
				continue;
			}
		}
	}

	fn binding(&self) -> TokenStream {
		let Field { var, ty, .. } = self;
		if ty.is_option() {
			quote! { let mut #var: #ty = None; }
		} else {
			quote! { let mut #var: Option<#ty> = None; }
		}
	}

	fn none_check(&self) -> TokenStream {
		let v = &self.var;
		quote! { #v.is_none() }
	}

	fn from_fields(fields: &Fields) -> Result<Vec<Self>> {
		fields
			.views()
			.into_iter()
			.zip(fields.iter())
			.map(|(view, syn_field)| {
				Ok(Self {
					var: view.binding,
					ty: view.ty.clone(),
					arg: ParseArg::from_attributes(&syn_field.attrs)?,
					atom: extract_atom(&syn_field.attrs)?,
				})
			})
			.collect()
	}

	fn atom_path_string(&self) -> Option<String> {
		self.atom.as_ref().map(|a| quote!(#a).to_string())
	}

	fn peek_tokens(&self) -> TokenStream {
		let peek_ty = if self.ty.is_option() { self.ty.unpack_option() } else { self.ty.clone() };
		if let Some(atom) = &self.atom {
			let atom_path = atom.path();
			let atom_set = atom.first_segment();
			quote! { p.peek::<#peek_ty>() && p.to_atom::<#atom_set>(p.peek_n(1)) == #atom_path }
		} else {
			quote! { p.peek::<#peek_ty>() }
		}
	}
}

fn members_tokens(fields: &Fields) -> Vec<TokenStream> {
	fields.members().map(|m| quote! { #m }).collect()
}

fn unexpected_at_next() -> TokenStream {
	quote! { return Err(crate::Diagnostic::new(p.peek_n(1), crate::Diagnostic::unexpected))?; }
}

fn unexpected_at_c() -> TokenStream {
	quote! { Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))? }
}

fn emit_peek_loop(atom_binding: TokenStream, parse_steps: TokenStream) -> TokenStream {
	quote! {
		loop {
			let c = p.peek_n(1);
			#atom_binding
			#parse_steps
			break;
		}
	}
}

fn generate_sequential_parsing(
	fields: &[Field],
	members: &[TokenStream],
	post_parse_steps: &TokenStream,
	constructor: TokenStream,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	let parse_steps = fields.iter().map(|f| f.parse_tokens(where_collector));
	let vars = fields.iter().map(|f| &f.var);
	quote! {
		#( #parse_steps )*
		#post_parse_steps
		return Ok(#constructor { #(#members: #vars),* });
	}
}

fn generate_must_occur_parsing(
	fields: &[Field],
	members: &[TokenStream],
	post_parse_steps: &TokenStream,
	parse_mode: FieldParseMode,
	constructor: TokenStream,
	where_collector: &mut WhereCollector,
	hoisted: &[&Ident],
) -> TokenStream {
	let bindings: TokenStream = fields.iter().filter(|f| !hoisted.contains(&&f.var)).map(Field::binding).collect();
	let any_atom = fields.iter().find_map(|f| f.atom.as_ref());
	let atom_binding = Atom::opt_binding_block(any_atom);

	let parse_steps: TokenStream = fields
		.iter()
		.map(|f| if f.atom.is_some() { f.atom_match_arm() } else { f.parse_normal_tokens(parse_mode, where_collector) })
		.collect();

	// Add where bounds for atom fields (atom_match_arm doesn't call wc.add)
	for f in fields.iter().filter(|f| f.atom.is_some()) {
		where_collector.add(&f.ty.unpack_option());
	}

	let vars = fields.iter().map(|f| &f.var);
	let checks: Vec<_> = fields.iter().map(Field::none_check).collect();
	let (occurance_cond, assignments): (TokenStream, Vec<TokenStream>) = match parse_mode {
		FieldParseMode::Sequential => unreachable!(),
		FieldParseMode::OneMustOccur => (quote! { #(#checks)&&* }, vars.map(|v| quote! { #v }).collect()),
		FieldParseMode::AllMustOccur => (quote! { #(#checks)||* }, vars.map(|v| quote! { #v.unwrap() }).collect()),
	};

	let peek_loop = emit_peek_loop(atom_binding, parse_steps);
	let unexpected = unexpected_at_c();
	quote! {
		#bindings
		#peek_loop
		#post_parse_steps
		if #occurance_cond {
			let c = p.peek_n(1);
			#unexpected
		}
		return Ok(#constructor { #(#members: #assignments),* });
	}
}

struct VariantPlan {
	ident: Ident,
	first_type: Type,
	effective_atom: Option<Atom>,
	body: TokenStream,
	fields: Vec<Field>,
	members: Vec<TokenStream>,
	post_parse_steps: TokenStream,
}

impl VariantPlan {
	fn new(variant: &Variant, post_parse_steps: &TokenStream, where_collector: &mut WhereCollector) -> Result<Self> {
		let ident = variant.ident.clone();
		let parse_mode = ParseArg::from_attributes(&variant.attrs)?.parse_mode();
		let variant_atom = extract_atom(&variant.attrs)?;
		let fields = Field::from_fields(&variant.fields)?;
		let first_type = fields
			.first()
			.map(|f| f.ty.clone())
			.ok_or_else(|| Error::new(ident.span(), "enum variant must have at least one field"))?;
		let members = members_tokens(&variant.fields);

		let body = if parse_mode == FieldParseMode::Sequential {
			generate_sequential_parsing(&fields, &members, post_parse_steps, quote! { Self::#ident }, where_collector)
		} else {
			generate_must_occur_parsing(
				&fields,
				&members,
				post_parse_steps,
				parse_mode,
				quote! { Self::#ident },
				where_collector,
				&[],
			)
		};

		// Prefer variant-level atom; fall back to first field's atom except
		// one_must_occur variants with all-optional fields dispatch by type only.
		let effective_atom = if variant_atom.is_some() {
			variant_atom
		} else if parse_mode == FieldParseMode::OneMustOccur && fields.iter().all(|f| f.ty.is_option()) {
			None
		} else {
			match variant.fields.iter().next() {
				Some(f) => extract_atom(&f.attrs)?,
				None => None,
			}
		};

		Ok(Self {
			ident,
			first_type,
			effective_atom,
			body,
			fields,
			members,
			post_parse_steps: post_parse_steps.clone(),
		})
	}

	fn discriminator(&self, shared_atom_paths: &[String]) -> TokenStream {
		let discriminating =
			self.fields.iter().find(|f| f.atom_path_string().is_some_and(|ap| !shared_atom_paths.contains(&ap)));
		if let Some(field) = discriminating {
			return field.peek_tokens();
		}

		if let Some(field) = self.fields.iter().find(|f| f.atom.is_some()) {
			return field.peek_tokens();
		}

		let mut peek_types = Vec::new();
		for f in &self.fields {
			let peek_ty = if f.ty.is_option() { f.ty.unpack_option() } else { f.ty.clone() };
			peek_types.push(peek_ty);
			if !f.ty.is_option() {
				break;
			}
		}
		let type_checks: Vec<TokenStream> = peek_types.iter().map(|t| quote! { p.peek::<#t>() }).collect();
		if type_checks.len() == 1 {
			type_checks.into_iter().next().expect("len checked")
		} else {
			quote! { #(#type_checks)||* }
		}
	}

	fn body_with_hoisted(&self, hoisted: &[&Ident], where_collector: &mut WhereCollector) -> TokenStream {
		let ident = &self.ident;
		generate_must_occur_parsing(
			&self.fields,
			&self.members,
			&self.post_parse_steps,
			FieldParseMode::OneMustOccur,
			quote! { Self::#ident },
			where_collector,
			hoisted,
		)
	}
}

enum GroupKind {
	AtomDispatch,
	PeekedFallback,
}

struct EnumVariantGroup<'a> {
	first_type: Type,
	kind: GroupKind,
	variants: Vec<&'a VariantPlan>,
	position: Position,
}

impl<'a> EnumVariantGroup<'a> {
	fn render(&self, where_collector: &mut WhereCollector) -> TokenStream {
		match self.kind {
			GroupKind::AtomDispatch => self.render_atom_dispatch(),
			GroupKind::PeekedFallback => self.render_peeked_fallback(where_collector),
		}
	}

	fn render_atom_dispatch(&self) -> TokenStream {
		let ty = &self.first_type;
		let extract_atom: TokenStream = self
			.variants
			.first()
			.iter()
			.flat_map(|v| v.effective_atom.as_ref())
			.map(|atom| atom.to_atom(format_ident!("c")))
			.collect();

		let atom_arms: TokenStream = self
			.variants
			.iter()
			.map(|v| {
				let atom_path = v.effective_atom.as_ref().expect("AtomDispatch variant must have atom").path();
				let body = &v.body;
				quote! { #atom_path => { #body }, }
			})
			.collect();

		let is_last = matches!(self.position, Position::Last | Position::Only);
		let unexpected = unexpected_at_next();
		if is_last {
			quote! {
				if p.peek::<#ty>() {
					let c = p.peek_n(1);
					match #extract_atom {
						#atom_arms
						_ => {
							return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
						}
					}
				} else {
					#unexpected
				}
			}
		} else {
			quote! {
				if p.peek::<#ty>() {
					let c = p.peek_n(1);
					match #extract_atom {
						#atom_arms
						_ => {}
					}
				}
			}
		}
	}

	fn render_peeked_fallback(&self, where_collector: &mut WhereCollector) -> TokenStream {
		let last_idx = self.variants.len() - 1;
		let is_outer_last = matches!(self.position, Position::Last | Position::Only);

		let plan = OneMustOccurPlan::for_group(&self.variants);
		let hoisted_bindings = plan.hoisted_bindings();
		let hoisted_preloop = plan.hoisted_preloop();
		let hoisted_var_names = plan.hoisted_var_names();

		let variant_blocks: TokenStream = self
			.variants
			.iter()
			.enumerate()
			.map(|(idx, variant)| {
				let is_last_variant = is_outer_last && idx == last_idx;

				// Single trailing variant with no shared hoisting: emit
				// body directly, no peek check needed.
				if is_last_variant && idx == 0 && plan.shared_atom_paths.is_empty() {
					let body = &variant.body;
					return quote! { { #body } };
				}

				let type_check = variant.discriminator(&plan.shared_atom_paths);
				let final_step = if hoisted_var_names.is_empty() {
					variant.body.clone()
				} else {
					variant.body_with_hoisted(&hoisted_var_names, where_collector)
				};

				if is_last_variant {
					let unexpected = unexpected_at_next();
					quote! {
						if #type_check { #final_step }
						else { #unexpected }
					}
				} else {
					quote! { if #type_check { #final_step } }
				}
			})
			.collect();

		quote! {
			#hoisted_bindings
			#hoisted_preloop
			#variant_blocks
		}
	}
}

/// Hoisting plan for `one_must_occur` sibling variants that share atom fields.
///
/// When variants share a field with the same atom (e.g. `balance` in
/// `[ wrap | wrap-reverse ] || balance`), we hoist it: declare the binding at
/// group level and run a pre-loop to consume it before the variant discriminator.
/// Per-variant bodies then reference the hoisted bindings instead of redeclaring them.
struct OneMustOccurPlan<'a> {
	shared_atom_paths: Vec<String>,
	shared_fields: Vec<&'a Field>,
}

impl<'a> OneMustOccurPlan<'a> {
	fn for_group(variants: &[&'a VariantPlan]) -> Self {
		if variants.len() <= 1 {
			return Self { shared_atom_paths: Vec::new(), shared_fields: Vec::new() };
		}

		let first_atoms: Vec<String> = variants[0].fields.iter().filter_map(Field::atom_path_string).collect();
		let shared_atom_paths: Vec<String> = first_atoms
			.into_iter()
			.filter(|ap| {
				variants[1..].iter().all(|v| v.fields.iter().any(|f| f.atom_path_string().as_deref() == Some(ap)))
			})
			.collect();

		let shared_fields: Vec<&Field> = variants[0]
			.fields
			.iter()
			.filter(|f| f.atom_path_string().is_some_and(|ap| shared_atom_paths.contains(&ap)))
			.collect();

		Self { shared_atom_paths, shared_fields }
	}

	fn shared_atom(&self) -> Option<&Atom> {
		self.shared_fields.first().and_then(|f| f.atom.as_ref())
	}

	fn hoisted_var_names(&self) -> Vec<&Ident> {
		self.shared_fields.iter().map(|f| &f.var).collect()
	}

	fn hoisted_bindings(&self) -> TokenStream {
		self.shared_fields.iter().map(|f| f.binding()).collect()
	}

	fn hoisted_preloop(&self) -> TokenStream {
		let Some(atom) = self.shared_atom() else {
			return TokenStream::new();
		};
		let parse_steps: TokenStream = self.shared_fields.iter().map(|f| f.atom_match_arm()).collect();
		let atom_binding = atom.binding_block();
		emit_peek_loop(atom_binding, parse_steps)
	}
}

fn derive_struct_body(
	data: &DataStruct,
	parse_mode: FieldParseMode,
	post_parse_steps: &TokenStream,
	where_collector: &mut WhereCollector,
) -> Result<TokenStream> {
	let fields = Field::from_fields(&data.fields)?;
	let members = members_tokens(&data.fields);
	Ok(if parse_mode == FieldParseMode::Sequential {
		generate_sequential_parsing(&fields, &members, post_parse_steps, quote! { Self }, where_collector)
	} else {
		generate_must_occur_parsing(
			&fields,
			&members,
			post_parse_steps,
			parse_mode,
			quote! { Self },
			where_collector,
			&[],
		)
	})
}

fn derive_enum_body(
	data: &DataEnum,
	post_parse_steps: &TokenStream,
	where_collector: &mut WhereCollector,
) -> Result<TokenStream> {
	let plans: Vec<VariantPlan> =
		data.variants.iter().map(|v| VariantPlan::new(v, post_parse_steps, where_collector)).collect::<Result<_>>()?;

	// Group by (first-field type, has-effective-atom). No-atom variants get their
	// own peeked-fallback group, after any atom-dispatch group for the same type.
	let grouped = plans
		.iter()
		.sorted_by_key(|p| {
			let ty = &p.first_type;
			(quote!(#ty).to_string(), p.effective_atom.is_none())
		})
		.chunk_by(|p| {
			let ty = &p.first_type;
			(quote!(#ty).to_string(), p.effective_atom.is_none())
		});

	let ts = grouped
		.into_iter()
		.with_position()
		.map(|(position, ((_type_str, no_atom), group))| {
			let variants: Vec<&VariantPlan> = group.collect();
			let first_type = variants[0].first_type.clone();
			let kind = if no_atom { GroupKind::PeekedFallback } else { GroupKind::AtomDispatch };
			let group = EnumVariantGroup { first_type, kind, variants, position };
			Ok(group.render(where_collector))
		})
		.collect::<Result<TokenStream>>()?;
	Ok(ts)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let generic_with_a = ensure_lifetime_a(generics);
	let (impl_generics, _, _) = generic_with_a.split_for_impl();
	let (_, type_generics, _) = generics.split_for_impl();

	let mut pre_parse_steps = quote! {};
	let mut post_parse_steps = quote! {};
	let parse_arg = ParseArg::from_attributes(&input.attrs)?;
	let parse_mode = parse_arg.parse_mode();
	if let Some(crate::darling_ext::StateArg(state_ident)) = parse_arg.state {
		pre_parse_steps = quote! {
			let state = p.set_state(State::#state_ident);
			#pre_parse_steps
		};
		post_parse_steps = quote! {
			#post_parse_steps
			p.set_state(state);
		};
	}
	if let Some(stop) = parse_arg.stop {
		let kind_ident = &stop.variant;
		pre_parse_steps = if stop.prefix == "Kind" {
			quote! {
				let stop = p.set_stop(KindSet::new(&[Kind::#kind_ident]));
				#pre_parse_steps
			}
		} else {
			quote! {
				let stop = p.set_stop(KindSet::#kind_ident);
				#pre_parse_steps
			}
		};
		post_parse_steps = quote! {
			#post_parse_steps
			p.set_stop(stop);
		};
	}

	let body = match &input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Parse on a Union")),
		Data::Struct(data) => derive_struct_body(data, parse_mode, &post_parse_steps, &mut where_collector)?,
		Data::Enum(data) => derive_enum_body(data, &post_parse_steps, &mut where_collector)?,
	};

	let generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&generics, parse_quote! { ::css_parse::Parse<'a> });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Parse<'a> for #ident #type_generics #where_clause {
			fn parse<I>(p: &mut css_parse::Parser<'a, I>) -> css_parse::Result<Self>
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
			{
				use css_parse::{Parse, Peek};
				#pre_parse_steps
				#body
			}
		}
	})
}
