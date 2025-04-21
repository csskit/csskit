use css_parse::keyword_set;

// https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment
// <attachment> = scroll | fixed | local
keyword_set!(Attachment { Scroll: "scroll", Fixed: "fixed", Local: "local" });
