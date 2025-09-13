use heck::ToKebabCase;
use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, LitStr, Variant};

fn extract_atom_string(attrs: &[Attribute]) -> Option<String> {
	for attr in attrs {
		if attr.path().is_ident("atom") {
			if let Ok(list) = attr.meta.require_list() {
				if let Ok(lit_str) = syn::parse2::<LitStr>(list.tokens.clone()) {
					return Some(lit_str.value());
				}
			}
		}
	}
	None
}

fn has_attribute(attrs: &[Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| attr.path().is_ident(name))
}

// Convert string bytes to u64 key (little-endian, case-insensitive)
fn string_to_u64_key(s: &str) -> u64 {
	let mut key = 0u64;
	for (i, &byte) in s.as_bytes().iter().enumerate() {
		if i >= 8 {
			break;
		}
		let normalized = byte.to_ascii_lowercase();
		key |= (normalized as u64) << (i * 8);
	}
	key
}

// Convert string bytes to u128 key (little-endian, case-insensitive)
fn string_to_u128_key(s: &str) -> u128 {
	let mut key = 0u128;
	for (i, &byte) in s.as_bytes().iter().enumerate() {
		if i >= 16 {
			break;
		}
		let normalized = byte.to_ascii_lowercase();
		key |= (normalized as u128) << (i * 8);
	}
	key
}

// Convert string bytes to multiple u128 keys for very long strings
fn string_to_multi_u128_keys(s: &str) -> Vec<u128> {
	let bytes = s.as_bytes();
	let mut keys = Vec::new();

	for chunk_start in (0..bytes.len()).step_by(16) {
		let mut key = 0u128;
		let chunk_end = (chunk_start + 16).min(bytes.len());

		for (i, &byte) in bytes[chunk_start..chunk_end].iter().enumerate() {
			let normalized = byte.to_ascii_lowercase();
			key |= (normalized as u128) << (i * 8);
		}
		keys.push(key);
	}

	keys
}

fn generate_u64_lookup_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
) -> TokenStream2 {
	let lookup_entries: Vec<_> = group
		.iter()
		.map(|(ident, string)| {
			let key = string_to_u64_key(string);
			quote! { #key => Self::#ident }
		})
		.collect();

	quote! {
		fn #fn_name(b: &[u8]) -> Self {
			if b.len() != #len {
				return Self::#default_variant;
			}
			let mut key = 0u64;
			for (i, &byte) in b.iter().enumerate() {
				let normalized = byte.to_ascii_lowercase();
				key |= (normalized as u64) << (i * 8);
			}
			match key {
				#(#lookup_entries,)*
				_ => Self::#default_variant,
			}
		}
	}
}

fn generate_u128_lookup_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
) -> TokenStream2 {
	let lookup_entries: Vec<_> = group
		.iter()
		.map(|(ident, string)| {
			let key = string_to_u128_key(string);
			quote! { #key => Self::#ident }
		})
		.collect();

	quote! {
		fn #fn_name(b: &[u8]) -> Self {
			if b.len() != #len {
				return Self::#default_variant;
			}
			let mut key = 0u128;
			for (i, &byte) in b.iter().enumerate() {
				let normalized = byte.to_ascii_lowercase();
				key |= (normalized as u128) << (i * 8);
			}
			match key {
				#(#lookup_entries,)*
				_ => Self::#default_variant,
			}
		}
	}
}

fn generate_tuple_match_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
) -> TokenStream2 {
	let patterns: Vec<_> = group
		.iter()
		.map(|(ident, string)| {
			let bytes = (0..len).map(|i| {
				let byte = string.as_bytes()[i];
				let lower = byte.to_ascii_lowercase();
				let upper = byte.to_ascii_uppercase();
				quote! { #lower | #upper }
			});
			quote! { (#(#bytes),*) => Self::#ident }
		})
		.collect();

	let parts = (0..len).map(|i| quote! {b[#i]});

	quote! {
		fn #fn_name(b: &[u8]) -> Self {
			if b.len() != #len {
				return Self::#default_variant;
			}
			match (#(#parts),*) {
				#(#patterns,)*
				_ => Self::#default_variant,
			}
		}
	}
}

fn generate_multi_u128_lookup_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
) -> TokenStream2 {
	let num_chunks = len.div_ceil(16); // Ceiling division

	// Generate individual match arms for each string
	let lookup_entries: Vec<_> = group
		.iter()
		.map(|(ident, string)| {
			let keys = string_to_multi_u128_keys(string);

			// Pad with zeros if needed to match the fixed array size
			let mut padded_keys = keys;
			while padded_keys.len() < num_chunks {
				padded_keys.push(0);
			}

			// Generate comparison for each chunk
			let key_checks = padded_keys.iter().enumerate().map(|(chunk_idx, &key)| {
				quote! { chunks[#chunk_idx] == #key }
			});

			quote! {
				if #(#key_checks)&&* {
					return Self::#ident;
				}
			}
		})
		.collect();

	quote! {
		fn #fn_name(b: &[u8]) -> Self {
			if b.len() != #len {
				return Self::#default_variant;
			}

			// Calculate number of chunks needed: ceiling division
			const NUM_CHUNKS: usize = #len.div_ceil(16);

			// Split string into fixed-size array of u128 chunks (allocation-free)
			let mut chunks = [0u128; NUM_CHUNKS];
			for chunk_idx in 0..NUM_CHUNKS {
				let chunk_start = chunk_idx * 16;
				if chunk_start < b.len() {
					let chunk_end = (chunk_start + 16).min(b.len());
					let mut key = 0u128;

					for (i, &byte) in b[chunk_start..chunk_end].iter().enumerate() {
						let normalized = byte.to_ascii_lowercase();
						key |= (normalized as u128) << (i * 8);
					}
					chunks[chunk_idx] = key;
				}
			}

			// Check each pattern
			#(#lookup_entries)*

			Self::#default_variant
		}
	}
}

pub fn generate(_args: TokenStream2, mut input: DeriveInput) -> TokenStream2 {
	let ident = &input.ident;
	let variants = match &mut input.data {
		Data::Enum(enum_data) => &mut enum_data.variants,
		_ => panic!("AtomSet can only be derived for enums"),
	};

	let default_variant = variants
		.iter()
		.find_map(|Variant { ident, attrs, .. }| if has_attribute(attrs, "default") { Some(ident) } else { None })
		.unwrap();

	let variants_and_strings = variants.iter().map(|Variant { ident, attrs, .. }| {
		(
			ident,
			if ident == default_variant {
				String::new()
			} else {
				// Check for #[atom("...")] attribute first, otherwise use kebab-case conversion
				extract_atom_string(attrs).unwrap_or_else(|| ident.to_string().to_kebab_case())
			},
		)
	});

	let to_str_match_arms = variants_and_strings.clone().map(|(ident, string)| quote! { Self::#ident => #string });

	let from_bits_match_arms = variants.iter().filter_map(|Variant { ident, .. }| {
		if ident == default_variant {
			None
		} else {
			Some(quote! { value if value == Self::#ident as u32 => Self::#ident })
		}
	});

	let grouped_variants = variants_and_strings
		.into_iter()
		.sorted_by(|(_, a), (_, b)| Ord::cmp(&a.len(), &b.len()))
		.chunk_by(|(_, string)| string.len() as u32)
		.into_iter()
		.map(|(len, group)| (len, group.collect::<Vec<_>>()))
		.collect::<Vec<_>>();

	let (match_fns, (from_str_match_arms, len_match_arms)): (Vec<_>, (Vec<_>, Vec<_>)) = grouped_variants
		.into_iter()
		.filter_map(|(len, group)| {
			if len == 0 {
				return None;
			}

			let len_usize = len as usize;
			let fn_name = format_ident!("match_str_of_len_{}", len_usize);

			// Choose the appropriate strategy based on string length
			let fn_match = if len_usize <= 5 {
				// Keep tuple matching for short strings (1-5 chars) - already optimized by compiler
				generate_tuple_match_fn(&fn_name, &group, len_usize, default_variant)
			} else if len_usize <= 8 {
				// Use u64 lookup table for strings 6-8 chars
				generate_u64_lookup_fn(&fn_name, &group, len_usize, default_variant)
			} else if len_usize <= 16 {
				// Use u128 lookup table for strings 9-16 chars
				generate_u128_lookup_fn(&fn_name, &group, len_usize, default_variant)
			} else {
				// Use multiple u128 chunks for very long strings (>16)
				generate_multi_u128_lookup_fn(&fn_name, &group, len_usize, default_variant)
			};

			let idents: Vec<_> = group.iter().map(|(ident, _)| ident).collect();
			Some((
				fn_match,
				(
					quote! { #len_usize => Self::#fn_name(b) },
					quote! {
						#(Self::#idents)|* => #len
					},
				),
			))
		})
		.unzip();

	quote! {
		impl #ident {
			#(#match_fns)*
		}

		impl AtomSet for #ident {
			#[inline]
			fn from_str(s: &str) -> Self {
				let b = s.as_bytes();
				match b.len() {
					#(#from_str_match_arms,)*
					_ => Self::#default_variant,
				}
			}

			#[inline]
			fn to_str(self) -> &'static str {
				match self {
					#(#to_str_match_arms,)*
				}
			}

			#[inline]
			fn len(&self) -> u32 {
				match self {
					#(#len_match_arms,)*
					Self::#default_variant => 0,
				}
			}

			#[inline]
			fn from_bits(value: u32) -> Self {
				match value {
					#(#from_bits_match_arms,)*
					_ => Self::#default_variant,
				}
			}

			#[inline]
			fn as_bits(&self) -> u32 {
				*self as u32
			}
		}
	}
}
