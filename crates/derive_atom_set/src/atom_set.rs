use heck::ToKebabCase;
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
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
	case_sensitive: bool,
) -> TokenStream2 {
	let mut table = vec![quote! { #type_name::#default_variant }; 256];
	for (ident, string) in group {
		if !string.is_empty() {
			let byte = string.as_bytes()[0];
			let (lower_byte, upper_byte) =
				if case_sensitive { (byte, byte) } else { (byte.to_ascii_lowercase(), byte.to_ascii_uppercase()) };

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
const SEED_ATTEMPTS: usize = 10_000;

fn pack(bytes: &[u8], case_sensitive: bool) -> u128 {
	bytes.iter().enumerate().fold(0, |acc, (i, &byte)| {
		acc | ((if case_sensitive { byte } else { byte.to_ascii_lowercase() } as u128) << (i * 8))
	})
}

fn fold_multiply(a: u64, b: u64) -> u64 {
	let product = (a as u128) * (b as u128);
	(product as u64) ^ ((product >> 64) as u64)
}

fn hash_words(words: &[(u64, u64)], (seed, multiplier_seed): (u64, u64)) -> u64 {
	words.iter().fold(seed, |hash, &(low, high)| fold_multiply(low ^ hash, high ^ multiplier_seed))
}

fn splitmix64(state: &mut u64) -> u64 {
	*state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
	let mut mixed = *state;
	mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
	mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
	mixed ^ (mixed >> 31)
}

struct PerfectHash {
	seeds: (u64, u64),
	bucket_bits: u32,
	displacements: Vec<u16>,
	slots: Vec<Option<usize>>,
}

fn build_perfect_hash(keys: &[Vec<(u64, u64)>]) -> PerfectHash {
	let slot_count = (keys.len() + keys.len() / 4 + 1).next_power_of_two();
	let slot_mask = slot_count as u64 - 1;
	let bucket_bits = (keys.len() / 2).next_power_of_two().max(2).trailing_zeros();
	let mut state = 0;
	for _ in 0..SEED_ATTEMPTS {
		let seeds = (splitmix64(&mut state), splitmix64(&mut state));
		let hashes: Vec<u64> = keys.iter().map(|words| hash_words(words, seeds)).collect();
		let mut buckets = vec![Vec::new(); 1 << bucket_bits];
		for (index, hash) in hashes.iter().enumerate() {
			buckets[(hash >> (64 - bucket_bits)) as usize].push(index);
		}
		let mut slots = vec![None; slot_count];
		let mut displacements = vec![0; buckets.len()];
		let placed =
			(0..buckets.len()).sorted_by_key(|&bucket| std::cmp::Reverse(buckets[bucket].len())).all(|bucket| {
				(0..slot_count as u64).any(|displacement| {
					let targets: Vec<usize> = buckets[bucket]
						.iter()
						.map(|&index| ((hashes[index] ^ displacement) & slot_mask) as usize)
						.collect();
					let free = targets.iter().all_unique() && targets.iter().all(|&slot| slots[slot].is_none());
					if free {
						for (&slot, &index) in targets.iter().zip(&buckets[bucket]) {
							slots[slot] = Some(index);
						}
						displacements[bucket] =
							u16::try_from(displacement).expect("AtomSet: perfect hash displacement exceeds u16");
					}
					free
				})
			});
		if placed {
			return PerfectHash { seeds, bucket_bits, displacements, slots };
		}
	}
	panic!("AtomSet: found no perfect hash for {} atoms", keys.len());
}

// The u64/u128 lookups case fold with `b |= (b & 0x40) >> 1`, which also folds the punctuation
// either side of A-Z. A byte is safe for that fold if the set of inputs folding to it is exactly
// the set of its ASCII case variants; atoms holding any other byte match byte-wise instead.
fn is_fold_safe(s: &str) -> bool {
	s.bytes().map(|b| b.to_ascii_lowercase()).all(|atom_byte| {
		(0..=u8::MAX).all(|b| ((b | ((b & 0x40) >> 1)) == atom_byte) == (b.to_ascii_lowercase() == atom_byte))
	})
}

// A byte is safe for the `& 0x40` fold if the set of inputs that fold to it is exactly the set of
// its ASCII case variants. Groups holding any other byte fold with a wider sequence that touches
// only A-Z, and that cannot carry between lanes.
fn generate_fold(fold_safe: bool, case_sensitive: bool, width: usize, var: &proc_macro2::Ident) -> TokenStream2 {
	if case_sensitive {
		return quote! {};
	}
	let repeat =
		|byte: u8| Literal::u128_unsuffixed((0..width).fold(0u128, |acc, i| acc | ((byte as u128) << (i * 8))));
	if fold_safe {
		let ones = repeat(0x40);
		quote! { #var |= (#var & #ones) >> 1; }
	} else {
		let low_bits = repeat(0x7f);
		let ge_a = repeat(0x3f);
		let gt_z = repeat(0x25);
		let high_bits = repeat(0x80);
		quote! {
			let low = #var & #low_bits;
			#var |= ((low + #ge_a) & !(low + #gt_z) & !#var & #high_bits) >> 2;
		}
	}
}

fn generate_lookup_fn(
	fn_name: &proc_macro2::Ident,
	group: &[(&proc_macro2::Ident, String)],
	len: usize,
	default_variant: &proc_macro2::Ident,
	type_name: &proc_macro2::Ident,
	fold_safe: bool,
	case_sensitive: bool,
) -> TokenStream2 {
	let chunk_count = len.div_ceil(16);
	let chunks: Vec<Vec<u128>> = group
		.iter()
		.map(|(_, string)| string.as_bytes().chunks(16).map(|chunk| pack(chunk, case_sensitive)).collect())
		.collect();
	let words: Vec<Vec<(u64, u64)>> = chunks
		.iter()
		.map(|chunks| chunks.iter().map(|&chunk| (chunk as u64, (chunk >> 64) as u64)).collect())
		.collect();
	let PerfectHash { seeds: (seed, multiplier_seed), bucket_bits, displacements, slots } = build_perfect_hash(&words);

	let bucket_count = displacements.len();
	let bucket_shift = 64 - bucket_bits;
	let slot_count = slots.len();
	let slot_mask = slot_count as u64 - 1;
	let atoms = slots.iter().map(|slot| {
		let variant = slot.map_or(default_variant, |index| group[index].0);
		quote! { #type_name::#variant }
	});

	let key = format_ident!("key");
	let chunk = format_ident!("chunk");
	let (key_type, keys, key_computation, hash_inputs): (_, Vec<_>, _, Vec<_>) = if len <= 8 {
		let keys = slots.iter().map(|slot| Literal::u64_suffixed(slot.map_or(0, |index| chunks[index][0] as u64)));
		let load = if len == 8 {
			quote! { let mut key = u64::from_le_bytes(b[..8].try_into().unwrap()); }
		} else {
			quote! {
				let mut bytes = [0u8; 8];
				bytes[..#len].copy_from_slice(&b[..#len]);
				let mut key = u64::from_le_bytes(bytes);
			}
		};
		let fold = generate_fold(fold_safe, case_sensitive, 8, &key);
		(
			quote! { u64 },
			keys.map(|key| quote! { #key }).collect(),
			quote! { #load #fold },
			vec![(quote! { key }, quote! { 0u64 })],
		)
	} else {
		let keys = slots.iter().map(|slot| {
			let chunks = slot.map_or(vec![0; chunk_count], |index| chunks[index].clone());
			let chunks = chunks.into_iter().map(Literal::u128_suffixed);
			quote! { [#(#chunks),*] }
		});
		let loads = (0..chunk_count).map(|index| {
			let start = index * 16;
			let end = (start + 16).min(len);
			let load = if end - start == 16 {
				quote! { let mut chunk = u128::from_le_bytes(b[#start..#end].try_into().unwrap()); }
			} else {
				let width = end - start;
				quote! {
					let mut bytes = [0u8; 16];
					bytes[..#width].copy_from_slice(&b[#start..#end]);
					let mut chunk = u128::from_le_bytes(bytes);
				}
			};
			let fold = generate_fold(fold_safe, case_sensitive, 16, &chunk);
			quote! {{ #load #fold chunk }}
		});
		let hash_inputs =
			(0..chunk_count).map(|index| (quote! { key[#index] as u64 }, quote! { (key[#index] >> 64) as u64 }));
		(quote! { [u128; #chunk_count] }, keys.collect(), quote! { let key = [#(#loads),*]; }, hash_inputs.collect())
	};
	let hash_steps = hash_inputs.iter().map(|(low, high)| {
		quote! {
			let product = ((#low ^ hash) as u128) * ((#high ^ #multiplier_seed) as u128);
			let hash = (product as u64) ^ ((product >> 64) as u64);
		}
	});
	let key_matches = if len <= 8 {
		quote! { KEYS[slot] == key }
	} else {
		let differences = (0..chunk_count).map(|index| quote! { (expected[#index] ^ key[#index]) });
		quote! {{
			let expected = &KEYS[slot];
			(#(#differences)|*) == 0
		}}
	};

	quote! {
		#[inline]
		fn #fn_name(b: &[u8]) -> Self {
			static DISPLACEMENTS: [u16; #bucket_count] = [#(#displacements),*];
			static KEYS: [#key_type; #slot_count] = [#(#keys),*];
			static ATOMS: [#type_name; #slot_count] = [#(#atoms),*];
			#key_computation
			let hash: u64 = #seed;
			#(#hash_steps)*
			let slot = ((hash ^ DISPLACEMENTS[(hash >> #bucket_shift) as usize] as u64) & #slot_mask) as usize;
			if #key_matches { ATOMS[slot] } else { Self::#default_variant }
		}
	}
}

pub fn generate(_args: TokenStream2, mut input: DeriveInput) -> TokenStream2 {
	let ident = &input.ident;
	let mut case_sensitive = false;
	for attr in input.attrs.iter().filter(|attr| attr.path().is_ident("atom_set")) {
		let parsed = attr.parse_nested_meta(|meta| {
			if meta.path.is_ident("case_sensitive") {
				case_sensitive = true;
				Ok(())
			} else {
				Err(meta.error("unsupported atom_set attribute"))
			}
		});
		if let Err(err) = parsed {
			return err.to_compile_error();
		}
	}
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

	let mut seen = std::collections::HashMap::new();
	for (ident, string) in variants_and_strings.clone() {
		let key = if case_sensitive { string.clone() } else { string.to_ascii_lowercase() };
		if let Some(previous) = seen.insert(key, ident) {
			let message = format!("atom {string:?} of {ident} duplicates {previous}");
			return syn::Error::new_spanned(ident, message).to_compile_error();
		}
	}

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
			let fold_safe = group.iter().all(|(_, string)| is_fold_safe(string));

			let fn_match = if len_usize == 1 {
				generate_ascii_table_1char_fn(&fn_name, &group, default_variant, ident, case_sensitive)
			} else {
				generate_lookup_fn(&fn_name, &group, len_usize, default_variant, ident, fold_safe, case_sensitive)
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

		impl ::atom_set::AtomSet for #ident {
			fn from_str(s: &str) -> Self {
				let b = s.as_bytes();
				match b.len() {
					#(#from_str_match_arms,)*
					_ => Self::#default_variant,
				}
			}

			fn to_str(self) -> &'static str {
				match self {
					#(#to_str_match_arms,)*
				}
			}

			fn len(&self) -> u32 {
				match self {
					#(#len_match_arms,)*
					Self::#default_variant => 0,
				}
			}

			fn from_bits(value: u32) -> Self {
				match value {
					#(#from_bits_match_arms,)*
					_ => Self::#default_variant,
				}
			}

			fn as_bits(&self) -> u32 {
				*self as u32
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::is_fold_safe;

	#[test]
	fn fold_safety_follows_the_reachable_byte_set() {
		assert!(is_fold_safe("border-block-end-width"));
		assert!(is_fold_safe("ADD"));
		assert!(is_fold_safe("100%"));
		assert!(is_fold_safe(":"));

		assert!(!is_fold_safe("a_b"));
		assert!(!is_fold_safe("a[b"));
		assert!(!is_fold_safe("a{b"));
		assert!(!is_fold_safe("café"));
	}
}
