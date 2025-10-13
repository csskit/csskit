use heck::ToKebabCase;
use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, LitStr, Variant};

fn extract_atom_string(attrs: &[Attribute]) -> Option<String> {
	for attr in attrs {
		if attr.path().is_ident("atom")
			&& let Ok(list) = attr.meta.require_list()
			&& let Ok(lit_str) = syn::parse2::<LitStr>(list.tokens.clone())
		{
			return Some(lit_str.value());
		}
	}
	None
}

fn has_attribute(attrs: &[Attribute], name: &str) -> bool {
	attrs.iter().any(|attr| attr.path().is_ident(name))
}

// For single character idents a lookup table is the most efficient;
// we only need to store a 256 item table, and direct lookup will be
// fast, even if the data is sparse it's not too much additional
// memory footprint.
fn generate_ascii_table_1char_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	default_variant: &proc_macro2::Ident,
	type_name: &proc_macro2::Ident,
) -> TokenStream2 {
	let mut table = vec![quote! { #type_name::#default_variant }; 256];
	for (ident, string) in group {
		if !string.is_empty() {
			let byte = string.as_bytes()[0];
			let lower_byte = byte.to_ascii_lowercase();
			let upper_byte = byte.to_ascii_uppercase();

			table[lower_byte as usize] = quote! { #type_name::#ident };
			if lower_byte != upper_byte {
				table[upper_byte as usize] = quote! { #type_name::#ident };
			}
		}
	}
	quote! {
		#[inline(always)]
		fn #fn_name(b: &[u8]) -> Self {
			const TABLE: [#type_name; 256] = [ #(#table),* ];
			TABLE[b[0] as usize]
		}
	}
}

fn string_to_multi_u128_keys(s: &str) -> Vec<u128> {
	let bytes = s.as_bytes();
	let mut keys = Vec::new();
	for chunk_start in (0..bytes.len()).step_by(16) {
		let chunk_end = (chunk_start + 16).min(bytes.len());
		let mut buf = [0u8; 16];
		for (i, &byte) in bytes[chunk_start..chunk_end].iter().enumerate() {
			buf[i] = byte.to_ascii_lowercase();
		}
		keys.push(u128::from_le_bytes(buf));
	}

	keys
}

fn generate_u64_lookup_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
) -> TokenStream2 {
	let keys: Vec<u64> = group
		.iter()
		.map(|(_, s)| {
			s.bytes().take(8).enumerate().fold(0, |acc, (i, b)| acc | ((b.to_ascii_lowercase() as u64) << (i * 8)))
		})
		.collect();
	let variants: Vec<_> = group.iter().map(|(ident, _)| ident).collect();

	// For lengths less than 8, we need padding
	let key_computation = if len == 8 {
		quote! {
			let mut key = u64::from_le_bytes(b[..8].try_into().unwrap());
		}
	} else {
		quote! {
			let mut bytes = [0u8; 8];
			bytes[..#len].copy_from_slice(&b[..#len]);
			let mut key = u64::from_le_bytes(bytes);
		}
	};

	quote! {
	#[inline]
		fn #fn_name(b: &[u8]) -> Self {
			#key_computation
			key |= (key & 0x4040404040404040) >> 1;
			match key {
				#( #keys => Self::#variants, )*
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
	let keys: Vec<u128> = group
		.iter()
		.map(|(_, s)| {
			s.bytes().take(16).enumerate().fold(0, |acc, (i, b)| acc | ((b.to_ascii_lowercase() as u128) << (i * 8)))
		})
		.collect();
	let variants: Vec<_> = group.iter().map(|(ident, _)| ident).collect();

	// For lengths less than 16, we need padding
	let key_computation = if len == 16 {
		quote! {
			let mut key = u128::from_le_bytes(b[..16].try_into().unwrap());
		}
	} else {
		quote! {
			let mut bytes = [0u8; 16];
			bytes[..#len].copy_from_slice(&b[..#len]);
			let mut key = u128::from_le_bytes(bytes);
		}
	};

	quote! {
	#[inline]
		fn #fn_name(b: &[u8]) -> Self {
			#key_computation
			key |= (key & 0x40404040404040404040404040404040) >> 1;
			match key {
				#( #keys => Self::#variants, )*
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
	let num_chunks = len.div_ceil(16);

	// Generate individual match arms for each string
	let lookup_entries: Vec<_> = group
		.iter()
		.map(|(ident, string)| {
			let keys = string_to_multi_u128_keys(string);
			let mut padded_keys = keys;
			while padded_keys.len() < num_chunks {
				padded_keys.push(0);
			}
			let key_checks = padded_keys.iter().enumerate().map(|(chunk_idx, &key)| {
				let chunk_var = quote::format_ident!("chunk_{}", chunk_idx);
				quote! { #chunk_var == #key }
			});
			quote! {
				if #(#key_checks)&&* {
					return Self::#ident;
				}
			}
		})
		.collect();

	let chunk_computations: Vec<_> = (0..num_chunks)
		.map(|chunk_idx| {
			let chunk_start = chunk_idx * 16;
			let chunk_end = std::cmp::min(chunk_start + 16, len);
			let bytes_in_chunk = chunk_end - chunk_start;
			let chunk_var = quote::format_ident!("chunk_{}", chunk_idx);

			if bytes_in_chunk == 0 {
				quote! { let #chunk_var = 0u128; }
			} else {
				// Need to normalize to lowercase
				quote! {
					let mut buf = [0u8; 16];
					for i in 0..#bytes_in_chunk {
						buf[i] = b[#chunk_start + i].to_ascii_lowercase();
					}
					let #chunk_var = u128::from_le_bytes(buf);
				}
			}
		})
		.collect();

	quote! {
		#[inline]
		fn #fn_name(b: &[u8]) -> Self {
			#(#chunk_computations)*
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
			let fn_match = if len_usize == 1 {
				// Use ASCII table lookup for 1-char strings
				generate_ascii_table_1char_fn(&fn_name, &group, default_variant, ident)
			} else if len_usize <= 8 {
				// Use u64 lookup table for strings 4-8 chars
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
