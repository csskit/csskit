use super::identifier::is_ident_byte;
use crate::simd::*;

#[inline(always)]
fn scan_lanes<S: Simd>(
	simd: S,
	bytes: &[u8],
	stop_bits: impl Fn(u8x16<S>) -> u64,
	stop_byte: impl Fn(u8) -> bool,
	side_bits: impl Fn(u8x16<S>) -> u64,
	side_byte: impl Fn(u8) -> bool,
) -> (usize, bool) {
	const LANES: usize = 16;
	const FULL: u64 = (1 << LANES) - 1;
	let mut side = false;
	let mut i = 0;
	while i + LANES <= bytes.len() {
		let v = u8x16::from_slice(simd, &bytes[i..i + LANES]);
		let stop = stop_bits(v) & FULL;
		let sides = side_bits(v);
		if stop != 0 {
			let at = stop.trailing_zeros();
			side |= (sides & ((1u64 << at) - 1)) != 0;
			return (i + at as usize, side);
		}
		side |= (sides & FULL) != 0;
		i += LANES;
	}
	while i < bytes.len() {
		let b = bytes[i];
		if stop_byte(b) {
			return (i, side);
		}
		side |= side_byte(b);
		i += 1;
	}
	(bytes.len(), side)
}

/// Scan `bytes` from the start for the first byte that is not an ASCII CSS ident byte (`a-z A-Z 0-9 - _`).
///
/// Returns `(offset, contains_upper)` where `offset` is the index of the first non-ident byte (or `bytes.len()`) and
/// `contains_upper` is true if any `A-Z` byte occurred strictly before `offset`.
#[inline]
pub(crate) fn scan_ident(bytes: &[u8]) -> (usize, bool) {
	dispatch!(*LEVEL, simd => scan_ident_impl(simd, bytes))
}

#[inline(always)]
fn scan_ident_impl<S: Simd>(simd: S, bytes: &[u8]) -> (usize, bool) {
	let lc = u8x16::splat(simd, 0x20);
	let a = u8x16::splat(simd, b'a');
	let z = u8x16::splat(simd, b'z');
	let d0 = u8x16::splat(simd, b'0');
	let d9 = u8x16::splat(simd, b'9');
	let ca = u8x16::splat(simd, b'A');
	let cz = u8x16::splat(simd, b'Z');
	let dash = u8x16::splat(simd, b'-');
	let underscore = u8x16::splat(simd, b'_');
	scan_lanes(
		simd,
		bytes,
		|v| {
			let lower = v | lc;
			let is_alpha = lower.simd_ge(a) & lower.simd_le(z);
			let is_digit = v.simd_ge(d0) & v.simd_le(d9);
			let is_ident = is_alpha | is_digit | v.simd_eq(dash) | v.simd_eq(underscore);
			!is_ident.to_bitmask()
		},
		|b| !is_ident_byte(b),
		|v| (v.simd_ge(ca) & v.simd_le(cz)).to_bitmask(),
		|b| b.wrapping_sub(b'A') < 26,
	)
}

/// Scan a string-token body for the first byte needing per-byte handlingw the closing delimiter, `\`, a newline (`\n`
/// `\r` `\x0C`), NUL, or any non-ASCII byte (`>= 0x80`).
#[inline]
pub(crate) fn scan_string(bytes: &[u8], delimiter: u8) -> usize {
	dispatch!(*LEVEL, simd => scan_string_impl(simd, bytes, delimiter))
}

#[inline(always)]
fn scan_string_impl<S: Simd>(simd: S, bytes: &[u8], delimiter: u8) -> usize {
	let delim = u8x16::splat(simd, delimiter);
	let bs = u8x16::splat(simd, b'\\');
	let lf = u8x16::splat(simd, b'\n');
	let cr = u8x16::splat(simd, b'\r');
	let ff = u8x16::splat(simd, 0x0C);
	let nul = u8x16::splat(simd, 0);
	let hi = u8x16::splat(simd, 0x80);
	scan_lanes(
		simd,
		bytes,
		|v| {
			(v.simd_eq(delim)
				| v.simd_eq(bs)
				| v.simd_eq(lf)
				| v.simd_eq(cr)
				| v.simd_eq(ff)
				| v.simd_eq(nul)
				| v.simd_ge(hi))
			.to_bitmask()
		},
		|b| b == delimiter || b == b'\\' || b == b'\n' || b == b'\r' || b == 0x0C || b == 0 || b >= 0x80,
		|_| 0,
		|_| false,
	)
	.0
}

/// Scan a url-token body for the first byte needing per-byte handling: whitespace/control (`<= 0x20`), a quote, `(`,
/// `)`, `\`, DEL (`0x7F`), or any non-ASCII byte (`>= 0x80`).
#[inline]
pub(crate) fn scan_url(bytes: &[u8]) -> usize {
	dispatch!(*LEVEL, simd => scan_url_impl(simd, bytes))
}

#[inline(always)]
fn scan_url_impl<S: Simd>(simd: S, bytes: &[u8]) -> usize {
	let ctrl = u8x16::splat(simd, 0x20);
	let hi = u8x16::splat(simd, 0x80);
	let squote = u8x16::splat(simd, b'\'');
	let dquote = u8x16::splat(simd, b'"');
	let open = u8x16::splat(simd, b'(');
	let close = u8x16::splat(simd, b')');
	let bs = u8x16::splat(simd, b'\\');
	let del = u8x16::splat(simd, 0x7F);
	scan_lanes(
		simd,
		bytes,
		|v| {
			(v.simd_le(ctrl)
				| v.simd_ge(hi)
				| v.simd_eq(squote)
				| v.simd_eq(dquote)
				| v.simd_eq(open)
				| v.simd_eq(close)
				| v.simd_eq(bs)
				| v.simd_eq(del))
			.to_bitmask()
		},
		|b| b <= 0x20 || b >= 0x80 || b == b'\'' || b == b'"' || b == b'(' || b == b')' || b == b'\\' || b == 0x7F,
		|_| 0,
		|_| false,
	)
	.0
}

/// Scan for the first occurrence of `needle`, or `bytes.len()` if absent.
#[inline]
pub(crate) fn scan_byte(bytes: &[u8], needle: u8) -> usize {
	dispatch!(*LEVEL, simd => scan_byte_impl(simd, bytes, needle))
}

#[inline(always)]
fn scan_byte_impl<S: Simd>(simd: S, bytes: &[u8], needle: u8) -> usize {
	let n = u8x16::splat(simd, needle);
	scan_lanes(simd, bytes, |v| v.simd_eq(n).to_bitmask(), |b| b == needle, |_| 0, |_| false).0
}

/// Scan a bad-url remnant for the first byte needing handling: `)` or `\`.
#[inline]
pub(crate) fn scan_bad_url(bytes: &[u8]) -> usize {
	dispatch!(*LEVEL, simd => scan_bad_url_impl(simd, bytes))
}

#[inline(always)]
fn scan_bad_url_impl<S: Simd>(simd: S, bytes: &[u8]) -> usize {
	let close = u8x16::splat(simd, b')');
	let bs = u8x16::splat(simd, b'\\');
	scan_lanes(
		simd,
		bytes,
		|v| (v.simd_eq(close) | v.simd_eq(bs)).to_bitmask(),
		|b| b == b')' || b == b'\\',
		|_| 0,
		|_| false,
	)
	.0
}

/// Scan a single-line-comment body for its terminator: a newline (`\n` `\r` `\x0C`) or NUL.
#[inline]
pub(crate) fn scan_line_comment(bytes: &[u8]) -> usize {
	dispatch!(*LEVEL, simd => scan_line_comment_impl(simd, bytes))
}

#[inline(always)]
fn scan_line_comment_impl<S: Simd>(simd: S, bytes: &[u8]) -> usize {
	let lf = u8x16::splat(simd, b'\n');
	let cr = u8x16::splat(simd, b'\r');
	let ff = u8x16::splat(simd, 0x0C);
	let nul = u8x16::splat(simd, 0);
	scan_lanes(
		simd,
		bytes,
		|v| (v.simd_eq(lf) | v.simd_eq(cr) | v.simd_eq(ff) | v.simd_eq(nul)).to_bitmask(),
		|b| b == b'\n' || b == b'\r' || b == 0x0C || b == 0,
		|_| 0,
		|_| false,
	)
	.0
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ident() {
		for (s, want) in [
			(&b""[..], (0usize, false)),
			(b"width", (5, false)),
			(b"width:1px", (5, false)),
			(b"color) ", (5, false)),
			(b"has space", (3, false)),
			(b"Background-Color", (16, true)),
			(b"ALLCAPS", (7, true)),
			(b"CAFE\xc3\xa9", (4, true)),
			(b"emoji\xf0\x9f\x98\x80tail", (5, false)),
			(b"abcdefghijklmnoP", (16, true)),
			(b"abcdefghijklmno P", (15, false)),
			(b"abcdefghijklmnop;Q", (16, false)),
			(b"abcdefghijklmnopQRSTUVWXYZ0123456789", (36, true)),
			(b"abcdefghijklmnopqrstuvwxyz0123456;", (33, false)),
		] {
			assert_eq!(scan_ident(s), want, "ident {s:?}");
		}
	}

	#[test]
	fn string() {
		for (s, delim, want) in [
			(&b""[..], b'"', 0usize),
			(b"hello world", b'"', 11),
			(b"quote\"tail", b'"', 5),
			(b"back\\slash", b'"', 4),
			(b"line\nbreak", b'"', 4),
			(b"carriage\rreturn", b'"', 8),
			(b"formfeed\x0cbyte", b'"', 8),
			(b"nul\0byte", b'"', 3),
			(b"unicode\xc3\xa9tail", b'"', 7),
			(b"it's", b'\'', 2),
			(b"it's", b'"', 4),
			(b"0123456789abcde\"", b'"', 15),
			(b"0123456789abcdef\"g", b'"', 16),
			(b"0123456789abcdefghijk", b'"', 21),
		] {
			assert_eq!(scan_string(s, delim), want, "string {s:?} delim {delim:#x}");
		}
	}

	#[test]
	fn url() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"path/to/image.png", 17),
			(b"has space", 3),
			(b"paren)here", 5),
			(b"back\\slash", 4),
			(b"quote'x", 5),
			(b"open(x", 4),
			(b"del\x7fx", 3),
			(b"tab\tbyte", 3),
			(b"unicode\xe2\x82\xactail", 7),
			(b"0123456789abcde)", 15),
			(b"0123456789abcdef)", 16),
		] {
			assert_eq!(scan_url(s), want, "url {s:?}");
		}
	}

	#[test]
	fn byte() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"has*star", 3),
			(b"no star here", 12),
			(b"0123456789abcde*", 15),
			(b"0123456789abcdef*", 16),
		] {
			assert_eq!(scan_byte(s, b'*'), want, "byte {s:?}");
		}
	}

	#[test]
	fn bad_url() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"cleantail", 9),
			(b"close)here", 5),
			(b"back\\slash", 4),
			(b"0123456789abcde)", 15),
			(b"0123456789abcdef)", 16),
		] {
			assert_eq!(scan_bad_url(s), want, "bad_url {s:?}");
		}
	}

	#[test]
	fn line_comment() {
		for (s, want) in [
			(&b""[..], 0usize),
			(b"comment body", 12),
			(b"line\nfeed", 4),
			(b"carriage\rreturn", 8),
			(b"formfeed\x0cbyte", 8),
			(b"nul\0byte", 3),
			(b"0123456789abcde\n", 15),
			(b"0123456789abcdef\n", 16),
		] {
			assert_eq!(scan_line_comment(s), want, "line_comment {s:?}");
		}
	}
}
