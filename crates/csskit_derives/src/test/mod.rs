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
