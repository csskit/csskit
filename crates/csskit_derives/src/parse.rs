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

trait Plan {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream;
}

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
		let atom = self.atom.as_ref().expect("atom_match_arm called without atom");
		let atom_path = atom.path();
		let atom_set = atom.first_segment();
		let var = &self.var;
		let inner_ty = self.ty.unpack_option();
		quote! {
			if #var.is_none() && p.peek::<#inner_ty>() && p.to_atom::<#atom_set>(c) == #atom_path {
				#var = Some(p.parse::<#inner_ty>()?);
				continue;
			}
		}
	}

	/// Emits a simple parse-if-peek for a single atom field.
	fn atom_parse_if_peek(&self) -> TokenStream {
		let atom = self.atom.as_ref().expect("atom_parse_if_peek called without atom");
		let atom_path = atom.path();
		let atom_set = atom.first_segment();
		let var = &self.var;
		let inner_ty = self.ty.unpack_option();
		quote! {
			if p.peek::<#inner_ty>()
				&& p.to_atom::<#atom_set>(c) == #atom_path
			{
				#var = Some(p.parse::<#inner_ty>()?);
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
			quote! { p.peek::<#peek_ty>() && p.to_atom::<#atom_set>(c) == #atom_path }
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

struct SequentialPlan<'a> {
	fields: &'a [Field],
	members: &'a [TokenStream],
	post_parse_steps: &'a TokenStream,
	constructor: TokenStream,
}

impl<'a> Plan for SequentialPlan<'a> {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream {
		let parse_steps = self.fields.iter().map(|f| f.parse_tokens(wc));
		let vars = self.fields.iter().map(|f| &f.var);
		let members = self.members;
		let constructor = &self.constructor;
		let post = self.post_parse_steps;
		quote! {
			#( #parse_steps )*
			#post
			return Ok(#constructor { #(#members: #vars),* });
		}
	}
}

struct MustOccurPlan<'a> {
	fields: &'a [Field],
	members: &'a [TokenStream],
	post_parse_steps: &'a TokenStream,
	parse_mode: FieldParseMode,
	constructor: TokenStream,
	hoisted: &'a [&'a Ident],
}

impl<'a> Plan for MustOccurPlan<'a> {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream {
		let MustOccurPlan { fields, members, post_parse_steps, parse_mode, constructor, hoisted } = self;
		let bindings: TokenStream = fields.iter().filter(|f| !hoisted.contains(&&f.var)).map(Field::binding).collect();

		let parse_steps: TokenStream = fields
			.iter()
			.map(|f| if f.atom.is_some() { f.atom_match_arm() } else { f.parse_normal_tokens(*parse_mode, wc) })
			.collect();

		// Add where bounds for atom fields (atom_match_arm doesn't call wc.add)
		for f in fields.iter().filter(|f| f.atom.is_some()) {
			wc.add(&f.ty.unpack_option());
		}

		let all_checks: Vec<_> = fields.iter().map(Field::none_check).collect();

		let required_checks: Vec<_> = fields.iter().filter(|f| !f.ty.is_option()).map(Field::none_check).collect();
		let (occurance_cond, assignments): (TokenStream, Vec<TokenStream>) = match parse_mode {
			FieldParseMode::Sequential => unreachable!(),
			FieldParseMode::OneMustOccur => {
				let vars = fields.iter().map(|f| &f.var);
				(quote! { #(#all_checks)&&* }, vars.map(|v| quote! { #v }).collect())
			}
			FieldParseMode::AllMustOccur => {
				let cond = if required_checks.is_empty() {
					quote! { #(#all_checks)&&* }
				} else {
					quote! { #(#required_checks)||* }
				};
				let assignments = fields
					.iter()
					.map(|f| {
						let v = &f.var;
						if f.ty.is_option() {
							quote! { #v }
						} else {
							quote! { #v.unwrap() }
						}
					})
					.collect();
				(cond, assignments)
			}
		};

		let unexpected = unexpected_at_c();
		quote! {
			#bindings
			loop {
				let c = p.peek_n(1);
				#parse_steps
				break;
			}
			#post_parse_steps
			if #occurance_cond {
				let c = p.peek_n(1);
				#unexpected
			}
			return Ok(#constructor { #(#members: #assignments),* });
		}
	}
}

struct VariantPlan {
	ident: Ident,
	first_type: Type,
	effective_atom: Option<Atom>,
	parse_mode: FieldParseMode,
	fields: Vec<Field>,
	members: Vec<TokenStream>,
	post_parse_steps: TokenStream,
}

impl VariantPlan {
	fn new(variant: &Variant, post_parse_steps: &TokenStream) -> Result<Self> {
		let ident = variant.ident.clone();
		let parse_mode = ParseArg::from_attributes(&variant.attrs)?.parse_mode();
		let variant_atom = extract_atom(&variant.attrs)?;
		let fields = Field::from_fields(&variant.fields)?;
		let first_type = fields
			.first()
			.map(|f| f.ty.clone())
			.ok_or_else(|| Error::new(ident.span(), "enum variant must have at least one field"))?;
		let members = members_tokens(&variant.fields);

		// Prefer variant-level atom; fall back to first field's atom except for
		// must-occur variants with all-optional fields.
		let all_optional = fields.iter().all(|f| f.ty.is_option());
		let effective_atom = if variant_atom.is_some() {
			variant_atom
		} else if (parse_mode == FieldParseMode::OneMustOccur || parse_mode == FieldParseMode::AllMustOccur)
			&& all_optional
		{
			None
		} else {
			fields.first().and_then(|f| f.atom.clone())
		};

		Ok(Self {
			ident,
			first_type,
			effective_atom,
			parse_mode,
			fields,
			members,
			post_parse_steps: post_parse_steps.clone(),
		})
	}

	/// Render the body for this variant, consuming where-bounds into `wc`.
	/// Picks `SequentialPlan` or `MustOccurPlan` based on `parse_mode`.
	fn body(&self, wc: &mut WhereCollector) -> TokenStream {
		let ident = &self.ident;
		let constructor = quote! { Self::#ident };
		if self.parse_mode == FieldParseMode::Sequential {
			SequentialPlan {
				fields: &self.fields,
				members: &self.members,
				post_parse_steps: &self.post_parse_steps,
				constructor,
			}
			.render(wc)
		} else {
			MustOccurPlan {
				fields: &self.fields,
				members: &self.members,
				post_parse_steps: &self.post_parse_steps,
				parse_mode: self.parse_mode,
				constructor,
				hoisted: &[],
			}
			.render(wc)
		}
	}

	fn discriminator(&self, shared_atom_paths: &[String], hoisted_type_var_names: &[&Ident]) -> TokenStream {
		let all_optional = self.fields.iter().all(|f| f.ty.is_option());
		let is_all_optional_must_occur =
			matches!(self.parse_mode, FieldParseMode::OneMustOccur | FieldParseMode::AllMustOccur) && all_optional;
		if is_all_optional_must_occur {
			let peeks: Vec<TokenStream> = self
				.fields
				.iter()
				.filter(|f| {
					f.atom_path_string().is_none_or(|ap| !shared_atom_paths.contains(&ap))
						&& !hoisted_type_var_names.contains(&&f.var)
				})
				.map(|f| f.peek_tokens())
				.collect();
			if !peeks.is_empty() {
				return quote! { #(#peeks)||* };
			}
		}

		let discriminating =
			self.fields.iter().find(|f| f.atom_path_string().is_some_and(|ap| !shared_atom_paths.contains(&ap)));
		if let Some(field) = discriminating {
			return field.peek_tokens();
		}

		if let Some(field) = self.fields.iter().find(|f| f.atom.is_some()) {
			return field.peek_tokens();
		}

		let peek_types: Vec<Type> = self
			.fields
			.iter()
			.filter(|f| !hoisted_type_var_names.contains(&&f.var))
			.scan(true, |still_optional, f| {
				if !*still_optional {
					return None;
				}
				if !f.ty.is_option() {
					*still_optional = false;
				}
				Some(f.ty.unpack_option())
			})
			.collect();
		let type_checks: Vec<TokenStream> = peek_types.iter().map(|t| quote! { p.peek::<#t>() }).collect();
		if type_checks.len() == 1 {
			type_checks.into_iter().next().expect("len checked")
		} else if type_checks.is_empty() {
			quote! { false }
		} else {
			quote! { #(#type_checks)||* }
		}
	}

	fn body_with_hoisted(&self, hoisted: &[&Ident], where_collector: &mut WhereCollector) -> TokenStream {
		let ident = &self.ident;
		MustOccurPlan {
			fields: &self.fields,
			members: &self.members,
			post_parse_steps: &self.post_parse_steps,
			parse_mode: FieldParseMode::OneMustOccur,
			constructor: quote! { Self::#ident },
			hoisted,
		}
		.render(where_collector)
	}

	/// Emit only the occurrence check + return for hoisted-only fallback.
	/// Used when all variant discriminants failed but hoisted shared fields
	/// may have been consumed.
	fn return_with_hoisted_only(&self, hoisted: &[&Ident]) -> TokenStream {
		let ident = &self.ident;
		let members = &self.members;
		let post_parse_steps = &self.post_parse_steps;
		let unexpected = unexpected_at_c();
		let none_checks: Vec<_> =
			self.fields.iter().filter(|f| hoisted.contains(&&f.var)).map(Field::none_check).collect();
		let occurance_cond = if none_checks.is_empty() {
			quote! { true }
		} else {
			quote! { #(#none_checks)&&* }
		};
		let assignments: Vec<TokenStream> = self
			.fields
			.iter()
			.map(|f| {
				if hoisted.contains(&&f.var) {
					let v = &f.var;
					quote! { #v }
				} else {
					quote! { None }
				}
			})
			.collect();
		quote! {
			#post_parse_steps
			if #occurance_cond {
				let c = p.peek_n(1);
				#unexpected
			}
			return Ok(Self::#ident { #(#members: #assignments),* });
		}
	}
}

/// Top-level enum-variant group: dispatches between `AtomDispatchPlan` (variants
/// with effective atoms on a shared first-field type) and `PeekedFallbackPlan`
/// (variants with no atom, fall back to peek-based discrimination).
enum EnumGroupPlan<'a> {
	AtomDispatch(AtomDispatchPlan<'a>),
	PeekedFallback(PeekedFallbackPlan<'a>),
}

impl<'a> EnumGroupPlan<'a> {
	fn new(no_atom: bool, variants: Vec<&'a VariantPlan>, position: &Position) -> Self {
		if no_atom {
			Self::PeekedFallback(PeekedFallbackPlan::new(variants, position))
		} else {
			let first_type = variants[0].first_type.clone();
			Self::AtomDispatch(AtomDispatchPlan::new(first_type, variants, position))
		}
	}
}

impl<'a> Plan for EnumGroupPlan<'a> {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream {
		match self {
			Self::AtomDispatch(p) => p.render(wc),
			Self::PeekedFallback(p) => p.render(wc),
		}
	}
}

/// Plan for a group of variants dispatched by atom on a shared first-field type.
///
/// All variants in the group have an effective atom; the type is peeked first,
/// then `to_atom` selects the branch.  When `is_last`, unknown atoms and a
/// missing peek both produce an error; otherwise they fall through silently.
struct AtomDispatchPlan<'a> {
	first_type: Type,
	variants: Vec<&'a VariantPlan>,
	is_last: bool,
}

impl<'a> AtomDispatchPlan<'a> {
	fn new(first_type: Type, variants: Vec<&'a VariantPlan>, position: &Position) -> Self {
		Self { first_type, variants, is_last: matches!(position, Position::Last | Position::Only) }
	}
}

impl<'a> Plan for AtomDispatchPlan<'a> {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream {
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
				let body = v.body(wc);
				quote! { #atom_path => { #body }, }
			})
			.collect();

		let unknown_atom_arm = if self.is_last {
			quote! { _ => { return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?; } }
		} else {
			quote! { _ => {} }
		};
		let else_branch = if self.is_last {
			let unexpected = unexpected_at_next();
			quote! { else { #unexpected } }
		} else {
			TokenStream::new()
		};
		quote! {
			if p.peek::<#ty>() {
				let c = p.peek_n(1);
				match #extract_atom {
					#atom_arms
					#unknown_atom_arm
				}
			} #else_branch
		}
	}
}

/// Plan for a group of variants dispatched by type-peek (no effective atom on
/// the group key).  Handles three sub-cases in priority order:
///
/// 1. Single variant, last group, no hoisting — emit body directly.
/// 2. Shared-prefix variants — delegate to `SharedPrefixPlan`.
/// 3. General case — emit one `if type_check { body }` block per variant,
///    with optional hoisted bindings and trailing fallback.
struct PeekedFallbackPlan<'a> {
	variants: Vec<&'a VariantPlan>,
	is_last: bool,
}

impl<'a> PeekedFallbackPlan<'a> {
	fn new(variants: Vec<&'a VariantPlan>, position: &Position) -> Self {
		Self { variants, is_last: matches!(position, Position::Last | Position::Only) }
	}
}

impl<'a> Plan for PeekedFallbackPlan<'a> {
	fn render(&self, where_collector: &mut WhereCollector) -> TokenStream {
		let hoist = SharedAtomHoistPlan::new(&self.variants);

		// Single variant, last group, no hoisting — emit body directly.
		if self.is_last && self.variants.len() == 1 && hoist.shared_atom_paths.is_empty() {
			return self.variants[0].body(where_collector);
		}

		// Shared prefix across all variants — consume prefix first, then atom-dispatch.
		if let Some(p) = SharedPrefixPlan::try_new(&self.variants, self.is_last) {
			return p.render(where_collector);
		}

		if let Some(p) = SharedAllMustOccurPlan::try_new(&self.variants, self.is_last) {
			return p.render(where_collector);
		}

		// General case: per-variant if-blocks with optional hoisting.
		MultiVariantBlocksPlan { variants: &self.variants, is_last: self.is_last, hoist: &hoist }
			.render(where_collector)
	}
}

/// General `PeekedFallback` rendering: per-variant `if discriminator { body }`
/// blocks with optional shared-atom hoisting and a fallback at the end.
struct MultiVariantBlocksPlan<'a, 'b> {
	variants: &'b [&'a VariantPlan],
	is_last: bool,
	hoist: &'b SharedAtomHoistPlan<'a>,
}

impl<'a, 'b> Plan for MultiVariantBlocksPlan<'a, 'b> {
	fn render(&self, where_collector: &mut WhereCollector) -> TokenStream {
		let hoisted_bindings = self.hoist.hoisted_bindings();
		let hoisted_preloop = self.hoist.hoisted_preloop(where_collector);
		let all_hoisted_var_names = self.hoist.all_hoisted_var_names();

		let variant_blocks: TokenStream = self
			.variants
			.iter()
			.map(|variant| {
				let type_check =
					variant.discriminator(&self.hoist.shared_atom_paths, &self.hoist.hoisted_type_var_names());
				let final_step = if all_hoisted_var_names.is_empty() {
					variant.body(where_collector)
				} else {
					variant.body_with_hoisted(&all_hoisted_var_names, where_collector)
				};
				quote! { if #type_check { #final_step } }
			})
			.collect();

		let fallback = if all_hoisted_var_names.is_empty() {
			let unexpected = unexpected_at_next();
			quote! { #unexpected }
		} else {
			self.variants[0].return_with_hoisted_only(&all_hoisted_var_names)
		};

		let trailing = if self.is_last {
			quote! { #fallback }
		} else {
			TokenStream::new()
		};

		let needs_c = !self.hoist.shared_fields.is_empty()
			|| self.variants.iter().any(|v| {
				let shared = &self.hoist.shared_atom_paths;
				let hoisted = self.hoist.hoisted_type_var_names();
				v.fields.iter().any(|f| {
					f.atom.is_some()
						&& f.atom_path_string().is_none_or(|ap| !shared.contains(&ap))
						&& !hoisted.contains(&&f.var)
				})
			});
		let peek_binding = if needs_c {
			quote! { let c = p.peek_n(1); }
		} else {
			quote! {}
		};

		quote! {
			#hoisted_bindings
			#hoisted_preloop
			#peek_binding
			#variant_blocks
			#trailing
		}
	}
}

/// Hoisting plan for sibling variants in a `PeekedFallback` group that share atom fields.
///
/// Hoists two kinds of shared fields:
/// - Atom fields (e.g. `balance` in `[ wrap | wrap-reverse ] || balance`): declared at
///   group level and consumed in a pre-check before the variant discriminator.
/// - Non-atom `Option<Type>` fields shared across all variants (e.g. `alignment_baseline`
///   in `[ first | last ] || <'alignment-baseline'>`): declared at group level and
///   consumed in a pre-loop before the variant discriminator, enabling parse of input
///   that starts with those types without a leading keyword.
///
/// Per-variant bodies reference the hoisted bindings instead of redeclaring them.
struct SharedAtomHoistPlan<'a> {
	shared_atom_paths: Vec<String>,
	shared_fields: Vec<&'a Field>,
	shared_type_fields: Vec<&'a Field>,
}

impl<'a> SharedAtomHoistPlan<'a> {
	fn new(variants: &[&'a VariantPlan]) -> Self {
		if variants.len() <= 1 {
			return Self { shared_atom_paths: Vec::new(), shared_fields: Vec::new(), shared_type_fields: Vec::new() };
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

		let shared_type_fields: Vec<&Field> = if variants.iter().all(|v| v.parse_mode == FieldParseMode::OneMustOccur) {
			variants[0]
				.fields
				.iter()
				.filter(|f| f.atom.is_none() && f.ty.is_option())
				.filter(|f| {
					let var_str = f.var.to_string();
					let ty = &f.ty;
					let ty_str = quote!(#ty).to_string();
					variants[1..].iter().all(|v| {
						v.fields.iter().any(|vf| {
							let vf_ty = &vf.ty;
							vf.atom.is_none() && vf.var == var_str && quote!(#vf_ty).to_string() == ty_str
						})
					})
				})
				.collect()
		} else {
			Vec::new()
		};

		Self { shared_atom_paths, shared_fields, shared_type_fields }
	}

	fn hoisted_type_var_names(&self) -> Vec<&Ident> {
		self.shared_type_fields.iter().map(|f| &f.var).collect()
	}

	fn all_hoisted_var_names(&self) -> Vec<&Ident> {
		self.shared_fields.iter().chain(self.shared_type_fields.iter()).map(|f| &f.var).collect()
	}

	fn hoisted_bindings(&self) -> TokenStream {
		self.shared_fields.iter().chain(self.shared_type_fields.iter()).map(|f| f.binding()).collect()
	}

	fn hoisted_preloop(&self, wc: &mut WhereCollector) -> TokenStream {
		let atom_preloop = match self.shared_fields.as_slice() {
			[] => TokenStream::new(),
			[field] => {
				let inner = field.atom_parse_if_peek();
				quote! { { let c = p.peek_n(1); #inner } }
			}
			_ => {
				let parse_steps: TokenStream = self.shared_fields.iter().map(|f| f.atom_match_arm()).collect();
				quote! {
					loop {
						let c = p.peek_n(1);
						#parse_steps
						break;
					}
				}
			}
		};

		let type_preloop = if self.shared_type_fields.is_empty() {
			TokenStream::new()
		} else {
			let parse_steps: TokenStream = self
				.shared_type_fields
				.iter()
				.map(|f| f.parse_normal_tokens(FieldParseMode::OneMustOccur, wc))
				.collect();
			quote! {
				loop {
					let c = p.peek_n(1);
					#parse_steps
					break;
				}
			}
		};

		quote! { #atom_preloop #type_preloop }
	}
}

/// Shared-prefix plan: variants in a `PeekedFallback` group all start with the
/// same non-atom required field type, differing only by a subsequent atom field
/// (or having no second field at all — the bare fallback).
///
/// e.g. `none | <custom-ident> [element | content]?` generates:
///   `None(Ident)`
///   `CustomIdentElement(CustomIdent, #[atom(Element)] Ident)`
///   `CustomIdentContent(CustomIdent, #[atom(Content)] Ident)`
///   `CustomIdent(CustomIdent)`
///
/// This should consume `CustomIdent` first, then atom-dispatch on next token.
struct SharedPrefixPlan<'a> {
	prefix_ty: Type,
	/// Variants that have a second field with a distinguishing atom.
	atom_variants: Vec<&'a VariantPlan>,
	/// Variant with only the prefix field (no second field), if any.
	bare_variant: Option<&'a VariantPlan>,
	is_last: bool,
}

impl<'a> SharedPrefixPlan<'a> {
	fn try_new(variants: &[&'a VariantPlan], is_last: bool) -> Option<Self> {
		if variants.len() < 2 {
			return None;
		}
		// All variants must start with the same non-atom first field type.
		let first_field = variants[0].fields.first()?;
		if first_field.atom.is_some() {
			return None;
		}
		let first_ty = &first_field.ty;
		let prefix_ty_str = quote!(#first_ty).to_string();
		let all_share_prefix = variants.iter().all(|v| {
			v.fields.first().is_some_and(|f| {
				let ty = &f.ty;
				f.atom.is_none() && quote!(#ty).to_string() == prefix_ty_str
			})
		});
		if !all_share_prefix {
			return None;
		}
		let mut atom_variants = Vec::new();
		let mut bare_variant = None;
		for v in variants {
			match v.fields.as_slice() {
				[_prefix] => {
					if bare_variant.is_some() {
						return None; // two bare variants — ambiguous
					}
					bare_variant = Some(*v);
				}
				[_prefix, second] if second.atom.is_some() => {
					atom_variants.push(*v);
				}
				_ => return None, // more complex structure — don't handle here
			}
		}
		// Need at least one atom variant for this to be useful.
		if atom_variants.is_empty() {
			return None;
		}
		Some(Self { prefix_ty: first_field.ty.clone(), atom_variants, bare_variant, is_last })
	}
}

impl<'a> Plan for SharedPrefixPlan<'a> {
	fn render(&self, where_collector: &mut WhereCollector) -> TokenStream {
		let is_last = self.is_last;
		let prefix_ty = &self.prefix_ty;
		where_collector.add(prefix_ty);
		let atom_set = self.atom_variants[0].fields[1]
			.atom
			.as_ref()
			.expect("atom_variants guaranteed to have atom on field 1")
			.first_segment();

		let atom_arms: TokenStream = self
			.atom_variants
			.iter()
			.map(|v| {
				let atom_path = v.fields[1].atom.as_ref().expect("checked above").path();
				let ident = &v.ident;
				let second_ty = &v.fields[1].ty;
				where_collector.add(second_ty);
				quote! {
					#atom_path => {
						let v1 = p.parse::<#second_ty>()?;
						return Ok(Self::#ident(v0, v1));
					}
				}
			})
			.collect();

		let default_arm: TokenStream = if let Some(bare) = self.bare_variant {
			let ident = &bare.ident;
			quote! { _ => return Ok(Self::#ident(v0)), }
		} else if is_last {
			let unexpected = unexpected_at_c();
			quote! { _ => { let c = p.peek_n(1); #unexpected } }
		} else {
			quote! { _ => {} }
		};

		let else_branch = if is_last {
			quote! { else { return Err(crate::Diagnostic::new(p.peek_n(1), crate::Diagnostic::unexpected))?; } }
		} else {
			TokenStream::new()
		};
		quote! {
			if p.peek::<#prefix_ty>() {
				let v0 = p.parse::<#prefix_ty>()?;
				let c = p.peek_n(1);
				match p.to_atom::<#atom_set>(c) {
					#atom_arms
					#default_arm
				}
			} #else_branch
		}
	}
}

/// Plan for sibling `AllMustOccur` variants that all have all-optional fields.
///
/// A single shared loop consumes every atom (and non-atom type) from every sibling
/// variant.  After the loop the filled fields determine which variant to construct.
///
/// Example: `[ filled | open ] || [ dot | sesame ]` generates four variants
/// `FilledDot`, `FilledSesame`, `OpenDot`, `OpenSesame` whose atom sets overlap.
/// Running separate per-variant loops would leave unconsumed tokens; the shared loop
/// avoids that by accepting all atoms and deciding the constructor at the end.
struct SharedAllMustOccurPlan<'a> {
	variants: Vec<&'a VariantPlan>,
	is_last: bool,
}

impl<'a> SharedAllMustOccurPlan<'a> {
	/// Returns `Some` iff all variants are `AllMustOccur` with all-optional fields
	/// and there is at least one shared atom between any two variants (i.e. the
	/// per-variant discriminators would overlap and the shared-loop approach is needed).
	fn try_new(variants: &[&'a VariantPlan], is_last: bool) -> Option<Self> {
		if variants.len() < 2 {
			return None;
		}
		let all_qualify = variants
			.iter()
			.all(|v| v.parse_mode == FieldParseMode::AllMustOccur && v.fields.iter().all(|f| f.ty.is_option()));
		if !all_qualify {
			return None;
		}
		let has_overlap = variants.iter().enumerate().any(|(i, v)| {
			let atoms_i: Vec<String> = v.fields.iter().filter_map(Field::atom_path_string).collect();
			variants.iter().enumerate().any(|(j, w)| {
				i != j && w.fields.iter().filter_map(Field::atom_path_string).any(|ap| atoms_i.contains(&ap))
			})
		});
		if !has_overlap {
			return None;
		}
		Some(Self { variants: variants.to_vec(), is_last })
	}
}

impl<'a> Plan for SharedAllMustOccurPlan<'a> {
	fn render(&self, wc: &mut WhereCollector) -> TokenStream {
		let mut seen_vars: Vec<String> = Vec::new();
		let mut all_fields: Vec<(&Field, usize)> = Vec::new();
		for (vi, v) in self.variants.iter().enumerate() {
			for f in &v.fields {
				let key = f.var.to_string();
				if !seen_vars.contains(&key) {
					seen_vars.push(key);
					all_fields.push((f, vi));
				}
			}
		}
		let atom_counts: Vec<(String, usize)> = {
			let mut counts: Vec<(String, usize)> = Vec::new();
			for v in &self.variants {
				for f in &v.fields {
					if let Some(ap) = Field::atom_path_string(f) {
						if let Some(entry) = counts.iter_mut().find(|(k, _)| k == &ap) {
							entry.1 += 1;
						} else {
							counts.push((ap, 1));
						}
					}
				}
			}
			counts
		};
		let is_shared_atom = |f: &Field| -> bool {
			match Field::atom_path_string(f) {
				Some(ap) => atom_counts.iter().find(|(k, _)| k == &ap).is_some_and(|(_, c)| *c > 1),
				None => false,
			}
		};
		let bindings: TokenStream = all_fields.iter().map(|(f, _)| f.binding()).collect();
		for (f, _) in all_fields.iter().filter(|(f, _)| f.atom.is_some()) {
			wc.add(&f.ty.unpack_option());
		}
		let parse_steps: TokenStream = all_fields
			.iter()
			.map(|(f, vi)| {
				if let Some(atom) = f.atom.as_ref().filter(|_| !is_shared_atom(f)) {
					let sentinel = vi + 1;
					let var = &f.var;
					let ty = &f.ty.unpack_option();
					let atom_path = atom.path();
					let atom_set = atom.first_segment();
					quote! {
						if (#var.is_none() && _alternative == 0 && p.peek::<#ty>() && p.to_atom::<#atom_set>(c) == #atom_path) {
							_alternative = #sentinel;
							#var = Some(p.parse::<#ty>()?);
							continue;
						}
					}
				} else if f.atom.is_some() {
					f.atom_match_arm()
				} else {
					f.parse_normal_tokens(FieldParseMode::AllMustOccur, wc)
				}
			})
			.collect();

		let all_none_checks: Vec<TokenStream> = all_fields.iter().map(|(f, _)| f.none_check()).collect();
		let nothing_parsed_check = quote! { #(#all_none_checks)&&* };
		let match_arms: TokenStream = self
			.variants
			.iter()
			.enumerate()
			.map(|(i, v)| {
				let sentinel = i + 1;
				let ident = &v.ident;
				let members = &v.members;
				let assignments: Vec<TokenStream> = v
					.fields
					.iter()
					.map(|f| {
						let v2 = &f.var;
						quote! { #v2 }
					})
					.collect();
				quote! { #sentinel => return Ok(Self::#ident { #(#members: #assignments),* }), }
			})
			.collect();
		let first_ident = &self.variants[0].ident;
		let first_members = &self.variants[0].members;
		let first_assignments: Vec<TokenStream> = self.variants[0]
			.fields
			.iter()
			.map(|f| {
				let v = &f.var;
				quote! { #v }
			})
			.collect();

		let unexpected = unexpected_at_next();
		let construct = quote! {
			match _alternative {
				#match_arms
				_ => return Ok(Self::#first_ident { #(#first_members: #first_assignments),* }),
			}
		};

		let trailing = if self.is_last {
			quote! {
				if #nothing_parsed_check {
					#unexpected
				}
				#construct
			}
		} else {
			quote! {
				if !#nothing_parsed_check {
					#construct
				}
			}
		};

		quote! {
			#bindings
			let mut _alternative: usize = 0;
			loop {
				let c = p.peek_n(1);
				#parse_steps
				break;
			}
			#trailing
		}
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
		SequentialPlan { fields: &fields, members: &members, post_parse_steps, constructor: quote! { Self } }
			.render(where_collector)
	} else {
		MustOccurPlan {
			fields: &fields,
			members: &members,
			post_parse_steps,
			parse_mode,
			constructor: quote! { Self },
			hoisted: &[],
		}
		.render(where_collector)
	})
}

fn derive_enum_body(
	data: &DataEnum,
	post_parse_steps: &TokenStream,
	where_collector: &mut WhereCollector,
) -> Result<TokenStream> {
	let plans: Vec<VariantPlan> =
		data.variants.iter().map(|v| VariantPlan::new(v, post_parse_steps)).collect::<Result<_>>()?;

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
			Ok(EnumGroupPlan::new(no_atom, variants, &position).render(where_collector))
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
