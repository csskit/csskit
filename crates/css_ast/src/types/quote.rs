use css_parse::keyword_set;

// https://drafts.csswg.org/css-content-3/#quote-values
// <quote> = open-quote | close-quote | no-open-quote | no-close-quote
keyword_set!(Quote {
	OpenQuote: "open-quote",
	CloseQuote: "close-quote",
	NoOpenQuote: "no-open-quote",
	NoCloseQuote: "no-close-quote",
});
