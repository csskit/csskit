use darling::FromMeta;
use quote::{ToTokens, format_ident};
use syn::{
	Ident, Meta, MetaNameValue, Result, Token, TypePath,
	parse::{Parse, ParseStream},
	parse2,
};

/// A `|`-separated list, e.g. `Elements | Text | None`.
///
/// Used for `applies_to`, `longhands`, `box_side`, `box_portion`, `node_kinds`,
/// `used_at_rules`, `vendor_prefixes`, `declaration_kinds`, `property_kinds`.
#[derive(Debug, Clone, Default)]
pub struct PipeList<T>(pub Vec<T>);

impl<T: Parse> FromMeta for PipeList<T> {
	fn from_meta(meta: &Meta) -> darling::Result<Self> {
		match meta {
			Meta::NameValue(nv) => {
				let ts = nv.value.to_token_stream();
				let items = parse2::<PipeListParsed<T>>(ts)
					.map_err(|e| darling::Error::custom(e.to_string()).with_span(&nv.value))?;
				Ok(Self(items.0))
			}
			_ => Err(darling::Error::custom("expected `key = A | B | C`").with_span(meta)),
		}
	}
}

struct PipeListParsed<T>(Vec<T>);

impl<T: Parse> Parse for PipeListParsed<T> {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut items = vec![input.parse::<T>()?];
		while input.parse::<Token![|]>().is_ok() {
			items.push(input.parse::<T>()?);
		}
		Ok(Self(items))
	}
}

/// Parse a `Type::Path` from a `NameValue` meta value, returning `(first_ident, last_ident)`.
fn parse_nameval_path_idents(nv: &MetaNameValue) -> darling::Result<(Ident, Ident)> {
	let ts = nv.value.to_token_stream();
	let type_path: TypePath = parse2(ts).map_err(|e| darling::Error::custom(e.to_string()).with_span(&nv.value))?;
	let first = type_path
		.path
		.segments
		.first()
		.ok_or_else(|| darling::Error::custom("expected Type::Variant path").with_span(&nv.value))?
		.ident
		.clone();
	let last = type_path
		.path
		.segments
		.last()
		.ok_or_else(|| darling::Error::custom("expected Type::Variant path").with_span(&nv.value))?
		.ident
		.clone();
	Ok((first, last))
}

/// Parsed from `#[parse(stop = Kind::Variant)]` or `#[parse(stop = KindSet::Variant)]`.
///
/// Stores the prefix (`Kind` or `KindSet`) and the variant ident separately.
#[derive(Debug, Clone)]
pub struct StopArg {
	pub prefix: Ident,
	pub variant: Ident,
}

impl FromMeta for StopArg {
	fn from_meta(meta: &Meta) -> darling::Result<Self> {
		match meta {
			Meta::NameValue(nv) => {
				let (first, last) = parse_nameval_path_idents(nv)?;
				if first != "Kind" && first != "KindSet" {
					return Err(darling::Error::custom("stop must use the Kind or KindSet type").with_span(&first));
				}
				Ok(Self { prefix: first, variant: last })
			}
			_ => Err(darling::Error::custom("expected `stop = Kind::Variant`").with_span(meta)),
		}
	}
}

/// Parsed from `#[parse(state = State::Variant)]`.
///
/// Stores just the variant ident (the `State::` prefix is validated then discarded).
#[derive(Debug, Clone)]
pub struct StateArg(pub Ident);

impl FromMeta for StateArg {
	fn from_meta(meta: &Meta) -> darling::Result<Self> {
		match meta {
			Meta::NameValue(nv) => {
				let (first, last) = parse_nameval_path_idents(nv)?;
				if first != "State" {
					return Err(darling::Error::custom(format!("state must use the State type, saw {:?}", first))
						.with_span(&first));
				}
				Ok(Self(last))
			}
			_ => Err(darling::Error::custom("expected `state = State::Variant`").with_span(meta)),
		}
	}
}

/// Parsed from `#[declaration_metadata(inherits)]` (bare → `True`)
/// or `#[declaration_metadata(inherits = Ident)]`.
#[derive(Debug, Clone)]
pub struct InheritsArg(pub Ident);

impl FromMeta for InheritsArg {
	/// Bare `inherits` keyword treated as `inherits = True`.
	fn from_word() -> darling::Result<Self> {
		Ok(Self(format_ident!("True")))
	}

	fn from_meta(meta: &Meta) -> darling::Result<Self> {
		match meta {
			Meta::Path(_) => Self::from_word(),
			Meta::NameValue(nv) => {
				let ts = nv.value.to_token_stream();
				let ident: Ident =
					parse2(ts).map_err(|e| darling::Error::custom(e.to_string()).with_span(&nv.value))?;
				Ok(Self(ident))
			}
			_ => Err(darling::Error::custom("expected bare `inherits` or `inherits = Ident`").with_span(meta)),
		}
	}
}
