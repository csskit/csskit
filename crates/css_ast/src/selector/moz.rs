use css_parse::{
	Build, Cursor, Parse, Parser, Result as ParserResult, T, diagnostics, function_set, pseudo_class, pseudo_element,
	syntax::CommaSeparated,
};
use csskit_derives::{ToCursors, Visitable};

use super::functional_pseudo_class::DirValue;

pseudo_element!(
	/// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes
	#[derive(Visitable)]
	#[visit(self)]
	pub enum MozPseudoElement {
		AnonymousBlock: "-moz-anonymous-block",
		AnonymousItem: "-moz-anonymous-item",
		AnonymousPositionedBlock: "-moz-anonymous-positioned-block",
		BlockInsideInlineWrapper: "-moz-block-inside-inline-wrapper",
		BlockRubyContent: "-moz-block-ruby-content",
		ButtonContent: "-moz-button-content",
		Canvas: "-moz-canvas",
		CellContent: "-moz-cell-content",
		ColorSwatch: "-moz-color-swatch",
		ColumnContent: "-moz-column-content",
		ColumnSet: "-moz-column-set",
		ColumnSpanWrapper: "-moz-column-span-wrapper",
		DropdownList: "-moz-dropdown-list",
		FieldsetContent: "-moz-fieldset-content",
		FirstLetterContinuation: "-moz-first-letter-continuation",
		FocusInner: "-moz-focus-inner",
		FocusOuter: "-moz-focus-outer",
		FramesetBlank: "-moz-frameset-blank",
		HframesetBorder: "-moz-hframeset-border",
		HtmlCanvasContent: "-moz-html-canvas-content",
		InlineTable: "-moz-inline-table",
		LineFrame: "-moz-line-frame",
		ListBullet: "-moz-list-bullet",
		ListNumber: "-moz-list-number",
		MathmlAnonymousBlock: "-moz-mathml-anonymous-block",
		NumberSpinBox: "-moz-number-spin-box",
		NumberSpinDown: "-moz-number-spin-down",
		NumberSpinUp: "-moz-number-spin-up",
		OofPlaceholder: "-moz-oof-placeholder",
		Page: "-moz-page",
		PageBreak: "-moz-page-break",
		PageContent: "-moz-page-content",
		PageSequence: "-moz-page-sequence",
		Pagebreak: "-moz-pagebreak",
		Pagecontent: "-moz-pagecontent",
		Placeholder: "-moz-placeholder",
		PrintedSheet: "-moz-printed-sheet",
		ProgressBar: "-moz-progress-bar",
		RangeProgress: "-moz-range-progress",
		RangeThumb: "-moz-range-thumb",
		RangeTrack: "-moz-range-track",
		Reveal: "-moz-reveal",
		Ruby: "-moz-ruby",
		RubyBase: "-moz-ruby-base",
		RubyBaseContainer: "-moz-ruby-base-container",
		RubyText: "-moz-ruby-text",
		RubyTextContainer: "-moz-ruby-text-container",
		ScrolledCanvas: "-moz-scrolled-canvas",
		ScrolledContent: "-moz-scrolled-content",
		ScrolledPageSequence: "-moz-scrolled-page-sequence",
		SearchClearButton: "-moz-search-clear-button",
		Selection: "-moz-selection",
		SvgForeignContent: "-moz-svg-foreign-content",
		SvgMarkerAnonChild: "-moz-svg-marker-anon-child",
		SvgMarkerOuterSvgAnonChild: "-moz-svg-marker-outer-svg-anon-child",
		SvgText: "-moz-svg-text",
		Table: "-moz-table",
		TableCell: "-moz-table-cell",
		TableColumn: "-moz-table-column",
		TableColumnGroup: "-moz-table-column-group",
		TableOuter: "-moz-table-outer",
		TableRow: "-moz-table-row",
		TableRowGroup: "-moz-table-row-group",
		TableWrapper: "-moz-table-wrapper",
		TextControlEditingRoot: "-moz-text-control-editing-root",
		TextControlPreview: "-moz-text-control-preview",
		TreeCell: "-moz-tree-cell",
		TreeCheckbox: "-moz-tree-checkbox",
		TreeDropFeedback: "-moz-tree-drop-feedback",
		TreeIndentation: "-moz-tree-indentation",
		TreeSeparator: "-moz-tree-separator",
		VframesetBorder: "-moz-vframeset-border",
		Viewport: "-moz-viewport",
		ViewportScroll: "-moz-viewport-scroll",
	}
);

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub enum MozFunctionalPseudoElement<'a> {
	TreeCell(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeCellText(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeCheckbox(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeColumn(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeImage(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeLine(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeRow(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeSeparator(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
	TreeTwisty(T![::], T![Function], CommaSeparated<'a, T![Ident]>, T![')']),
}

function_set!(
	pub enum MozFunctionalPseudoElementKeyword {
		TreeCell: "-moz-tree-cell",
		TreeCellText: "-moz-tree-cell-text",
		TreeCheckbox: "-moz-tree-checkbox",
		TreeColumn: "-moz-tree-column",
		TreeImage: "-moz-tree-image",
		TreeLine: "-moz-tree-line",
		TreeRow: "-moz-tree-row",
		TreeSeparator: "-moz-tree-separator",
		TreeTwisty: "-moz-tree-twisty",
	}
);

impl<'a> Parse<'a> for MozFunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let keyword = p.parse::<MozFunctionalPseudoElementKeyword>()?;
		let function = <T![Function]>::build(p, keyword.into());
		let items = p.parse::<CommaSeparated<'a, T![Ident]>>()?;
		let close = p.parse::<T![')']>()?;
		Ok(match keyword {
			MozFunctionalPseudoElementKeyword::TreeCell(_) => Self::TreeCell(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeCellText(_) => Self::TreeCellText(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeCheckbox(_) => Self::TreeCheckbox(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeColumn(_) => Self::TreeColumn(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeImage(_) => Self::TreeImage(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeLine(_) => Self::TreeLine(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeRow(_) => Self::TreeRow(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeSeparator(_) => Self::TreeSeparator(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeTwisty(_) => Self::TreeTwisty(colons, function, items, close),
		})
	}
}

pseudo_class!(
	/// <https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes>
	#[derive(Visitable)]
	#[visit(self)]
	pub enum MozPseudoClass {
		Any: "-moz-any",
		AnyLink: "-moz-any-link",
		Broken: "-moz-broken",
		DragOver: "-moz-drag-over",
		FirstNode: "-moz-first-node",
		FocusRing: "-moz-focusring",
		FullScreen: "-moz-full-screen",
		FullScreenAncestor: "-moz-full-screen-ancestor",
		HandlerBlocked: "-moz-handler-blocked",
		HandlerCrashed: "-moz-handler-crashed",
		HandlerDisabled: "-moz-handler-disabled",
		LastNode: "-moz-last-node",
		Loading: "-moz-loading",
		LwTheme: "-moz-lwtheme",
		LwThemeBrighttext: "-moz-lwtheme-brighttext",
		LwThemeDarktext: "-moz-lwtheme-darktext",
		NativeAnonymous: "-moz-native-anonymous",
		OnlyWhitespace: "-moz-only-whitespace",
		PlaceholderShown: "-moz-placeholder-shown",
		ReadOnly: "-moz-read-only",
		ReadWrite: "-moz-read-write",
		SubmitInvalid: "-moz-submit-invalid",
		Suppressed: "-moz-suppressed",
		UiInvalid: "-moz-ui-invalid",
		UiValid: "-moz-ui-valid",
		UserDisabled: "-moz-user-disabled",
		WindowInactive: "-moz-window-inactive",
	}
);

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum MozFunctionalPseudoClass {
	LocaleDir(MozLocaleDirFunctionalPseudoClass),
}

impl<'a> Parse<'a> for MozFunctionalPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.eq_ignore_ascii_case(c, "-moz-locale-dir") {
			let value = p.parse::<DirValue>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::LocaleDir(MozLocaleDirFunctionalPseudoClass { colon, function, value, close }))
		} else {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
	}
}

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub struct MozLocaleDirFunctionalPseudoClass {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MozPseudoElement>(), 40);
		assert_eq!(std::mem::size_of::<MozFunctionalPseudoElement>(), 88);
		assert_eq!(std::mem::size_of::<MozPseudoClass>(), 28);
		assert_eq!(std::mem::size_of::<MozFunctionalPseudoClass>(), 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MozPseudoElement, "::-moz-anonymous-block");
		assert_parse!(MozFunctionalPseudoElement, "::-moz-tree-twisty(selected,focus)");
	}
}
