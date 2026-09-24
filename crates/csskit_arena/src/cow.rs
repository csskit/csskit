use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::boxed::Box;

/// A Cow-like string that supports custom allocators
pub enum CowStr<'a, A: Allocator = Global> {
	Borrowed(&'a str),
	Owned(Box<str, A>),
}

impl<'a, A: Allocator> CowStr<'a, A> {
	pub fn as_str(&self) -> &str {
		match self {
			CowStr::Borrowed(s) => s,
			CowStr::Owned(b) => b,
		}
	}
}

impl<'a, A: Allocator> core::ops::Deref for CowStr<'a, A> {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl<'a, A: Allocator> AsRef<str> for CowStr<'a, A> {
	fn as_ref(&self) -> &str {
		self.as_str()
	}
}

impl<'a, A: Allocator> PartialEq<&str> for CowStr<'a, A> {
	fn eq(&self, other: &&str) -> bool {
		self.as_str() == *other
	}
}

impl<'a, A: Allocator> PartialEq<CowStr<'a, A>> for &str {
	fn eq(&self, other: &CowStr<'a, A>) -> bool {
		*self == other.as_str()
	}
}

impl<'a, A: Allocator> PartialEq<String> for CowStr<'a, A> {
	fn eq(&self, other: &String) -> bool {
		self.as_str() == other
	}
}

impl<'a, A: Allocator> PartialEq<CowStr<'a, A>> for String {
	fn eq(&self, other: &CowStr<'a, A>) -> bool {
		self == other.as_str()
	}
}

impl<'a, A: Allocator> core::fmt::Debug for CowStr<'a, A> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Debug::fmt(self.as_str(), f)
	}
}

impl<'a, A: Allocator> core::fmt::Display for CowStr<'a, A> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Display::fmt(self.as_str(), f)
	}
}

impl<'a, A: Allocator> From<&'a str> for CowStr<'a, A> {
	fn from(s: &'a str) -> Self {
		CowStr::Borrowed(s)
	}
}

impl<'a, A: Allocator> From<Box<str, A>> for CowStr<'a, A> {
	fn from(b: Box<str, A>) -> Self {
		CowStr::Owned(b)
	}
}
