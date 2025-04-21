use css_parse::keyword_set;

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-special
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// For the purpose of this specification, they all have the same effect as auto.
// However, the host language may also take these values into account when defining the native appearance of the element.
// <compat-special> = textfield | menulist-button
keyword_set!(CompatSpecial { Textfield: "textfield", MenulistButton: "menulist-button" });
