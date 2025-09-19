use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-display-4/#typedef-display-outside>
	///
	/// ```text,ignore
	/// <display-outside>  = block | inline | run-in
	/// ```
	pub enum DisplayOutside {
		Block: "block",
		Inline: "inline",
		RunIn: "run-in"
	}
);
