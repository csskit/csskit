use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-special>
	///
	/// These values exist for compatibility of content developed for earlier non-standard versions of this property.
	/// For the purpose of this specification, they all have the same effect as auto.
	/// However, the host language may also take these values into account when defining the native appearance of the element.
	///
	/// ```text,ignore
	/// <compat-special> = textfield | menulist-button
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum CompatSpecial {
		Textfield: "textfield",
		MenulistButton: "menulist-button"
	}
);
