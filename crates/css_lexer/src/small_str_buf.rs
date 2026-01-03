use std::fmt::{Error, Result, Write};

#[derive(Debug)]
pub(crate) struct SmallStrBuf<const N: usize>(u8, [u8; N]);

impl<const N: usize> SmallStrBuf<N> {
	pub const fn new() -> Self {
		Self(0, [b'-'; N])
	}

	#[inline]
	pub fn append(&mut self, c: char) {
		let n = self.0 as usize;
		let char_len = c.len_utf8();
		if n + char_len <= N {
			c.encode_utf8(&mut self.1[n..]);
		}
		self.0 += char_len as u8;
	}

	#[inline]
	pub const fn over_capacity(&self) -> bool {
		self.0 >= N as u8
	}

	#[inline]
	pub fn as_str(&self) -> Option<&str> {
		if self.over_capacity() {
			None
		} else {
			// SAFETY: We only append valid UTF-8 chars, so this is always valid
			Some(unsafe { str::from_utf8_unchecked(&self.1[0..self.0 as usize]) })
		}
	}
}

impl<const N: usize> Write for SmallStrBuf<N> {
	fn write_str(&mut self, s: &str) -> Result {
		let b = s.as_bytes();
		let n = b.len() as u8;
		if (self.0 + n) as usize > N {
			return Err(Error);
		}
		self.1[self.0 as usize..(self.0 + n) as usize].copy_from_slice(b);
		self.0 += n;
		Ok(())
	}
}
