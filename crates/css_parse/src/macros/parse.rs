/// A macro for easily calling the Parser and entirely parsing a string.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// let bump = Bump::default();
/// parse!(in bump "foo"); // ComponentValues
///
/// parse!(in bump "foo" as T![Ident]);
///
/// let features = Feature::SingleLineComments;
/// parse!(in bump with features "//foo");
///
/// // Or pass a reference to a str
/// let source = "//foo";
/// let features = Feature::SingleLineComments;
/// parse!(in bump with features &source);
/// ```
#[macro_export]
macro_rules! parse {
	(in $bump: ident $(with $features: ident)? &$source_text: ident as $ty: ty) => {
		{
			let mut p = $crate::Parser::new(&$bump, $source_text)$(.with_features($features))?;
			p.parse_entirely::<$ty>()
		}
	};
	(in $bump: ident $(with $features: ident)? $str: literal as $ty: ty) => {
		{
			let source_text = $str;
			parse!(in $bump $(with $features)? &source_text as $ty)
		}
	};
	(in $bump: ident $(with $features: ident)? $str: literal) => {
		{
			use $crate::ComponentValues;
			parse!(in $bump $(with $features)? $str as ComponentValues)
		}
	};
	(in $bump: ident $(with $features: ident)? &$str: ident) => {
		{
			use $crate::ComponentValues;
			parse!(in $bump $(with $features)? &$str as ComponentValues)
		}
	};
}
