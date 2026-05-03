mod test_parse;
mod test_peek;
mod test_semantic_eq;
mod test_to_cursors;
mod test_to_span;
mod test_visitable;

#[macro_export]
macro_rules! to_deriveinput {
	( $($tt:tt)+ ) => {
		::syn::parse2::<::syn::DeriveInput>(::quote::quote!{ $($tt)+ }).unwrap()
	}
}
#[cfg(test)]
pub(crate) use to_deriveinput;

macro_rules! assert_derive_snapshot {
	( $derive_fn:path, $data:ident, $name:literal) => {
		let tokens = $derive_fn($data).expect("derive failed");
		let file = ::syn::parse2::<syn::File>(tokens).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}
#[cfg(test)]
pub(crate) use assert_derive_snapshot;
