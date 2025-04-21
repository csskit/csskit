use css_parse::keyword_set;

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-auto
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// They all have the same effect as auto.
// <compat-auto> = searchfield | textarea | checkbox | radio | menulist | listbox | meter | progress-bar | button
keyword_set!(CompatAuto {
	Searchfield: "searchfield",
	Textarea: "textarea",
	Checkbox: "checkbox",
	Radio: "radio",
	Menulist: "menulist",
	Listbox: "listbox",
	Meter: "meter",
	ProgressBar: "progress-bar",
	Button: "button",
});
