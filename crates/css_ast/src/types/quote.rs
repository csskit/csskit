use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-content-3/#quote-values>
	///
	/// ```text,ignore
	/// <quote> = open-quote | close-quote | no-open-quote | no-close-quote
	/// ```
	pub enum Quote {
		OpenQuote: "open-quote",
		CloseQuote: "close-quote",
		NoOpenQuote: "no-open-quote",
		NoCloseQuote: "no-close-quote",
	}
);
