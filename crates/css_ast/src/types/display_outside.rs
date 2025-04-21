use css_parse::keyword_set;

// https://drafts.csswg.org/css-display-4/#typedef-display-outside
// <display-outside>  = block | inline | run-in
keyword_set!(DisplayOutside { Block: "block", Inline: "inline", RunIn: "run-in" });
