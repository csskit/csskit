use crate::{CssAtomSet, CssDiagnostic, DirValue};
use css_parse::{
	Cursor, Diagnostic, Parse, Parser, Result as ParserResult, T, pseudo_class, pseudo_element, syntax::CommaSeparated,
};
use csskit_derives::{Parse, Peek, SemanticEq, ToCursors, ToSpan};

pseudo_element!(
	/// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum MozPseudoElement {
		AnonymousBlock: CssAtomSet::_MozAnonymousBlock,
		AnonymousItem: CssAtomSet::_MozAnonymousItem,
		AnonymousPositionedBlock: CssAtomSet::_MozAnonymousPositionedBlock,
		BlockInsideInlineWrapper: CssAtomSet::_MozBlockInsideInlineWrapper,
		BlockRubyContent: CssAtomSet::_MozBlockRubyContent,
		ButtonContent: CssAtomSet::_MozButtonContent,
		Canvas: CssAtomSet::_MozCanvas,
		CellContent: CssAtomSet::_MozCellContent,
		ColorSwatch: CssAtomSet::_MozColorSwatch,
		ColumnContent: CssAtomSet::_MozColumnContent,
		ColumnSet: CssAtomSet::_MozColumnSet,
		ColumnSpanWrapper: CssAtomSet::_MozColumnSpanWrapper,
		DropdownList: CssAtomSet::_MozDropdownList,
		FieldsetContent: CssAtomSet::_MozFieldsetContent,
		FirstLetterContinuation: CssAtomSet::_MozFirstLetterContinuation,
		FocusInner: CssAtomSet::_MozFocusInner,
		FocusOuter: CssAtomSet::_MozFocusOuter,
		FramesetBlank: CssAtomSet::_MozFramesetBlank,
		HframesetBorder: CssAtomSet::_MozHframesetBorder,
		HtmlCanvasContent: CssAtomSet::_MozHtmlCanvasContent,
		InlineTable: CssAtomSet::_MozInlineTable,
		LineFrame: CssAtomSet::_MozLineFrame,
		ListBullet: CssAtomSet::_MozListBullet,
		ListNumber: CssAtomSet::_MozListNumber,
		MathmlAnonymousBlock: CssAtomSet::_MozMathmlAnonymousBlock,
		NumberSpinBox: CssAtomSet::_MozNumberSpinBox,
		NumberSpinDown: CssAtomSet::_MozNumberSpinDown,
		NumberSpinUp: CssAtomSet::_MozNumberSpinUp,
		OofPlaceholder: CssAtomSet::_MozOofPlaceholder,
		Page: CssAtomSet::_MozPage,
		PageBreak: CssAtomSet::_MozPageBreak,
		PageContent: CssAtomSet::_MozPageContent,
		PageSequence: CssAtomSet::_MozPageSequence,
		Pagebreak: CssAtomSet::_MozPagebreak,
		Pagecontent: CssAtomSet::_MozPagecontent,
		Placeholder: CssAtomSet::_MozPlaceholder,
		PrintedSheet: CssAtomSet::_MozPrintedSheet,
		ProgressBar: CssAtomSet::_MozProgressBar,
		RangeProgress: CssAtomSet::_MozRangeProgress,
		RangeThumb: CssAtomSet::_MozRangeThumb,
		RangeTrack: CssAtomSet::_MozRangeTrack,
		Reveal: CssAtomSet::_MozReveal,
		Ruby: CssAtomSet::_MozRuby,
		RubyBase: CssAtomSet::_MozRubyBase,
		RubyBaseContainer: CssAtomSet::_MozRubyBaseContainer,
		RubyText: CssAtomSet::_MozRubyText,
		RubyTextContainer: CssAtomSet::_MozRubyTextContainer,
		ScrolledCanvas: CssAtomSet::_MozScrolledCanvas,
		ScrolledContent: CssAtomSet::_MozScrolledContent,
		ScrolledPageSequence: CssAtomSet::_MozScrolledPageSequence,
		SearchClearButton: CssAtomSet::_MozSearchClearButton,
		Selection: CssAtomSet::_MozSelection,
		SvgForeignContent: CssAtomSet::_MozSvgForeignContent,
		SvgMarkerAnonChild: CssAtomSet::_MozSvgMarkerAnonChild,
		SvgMarkerOuterSvgAnonChild: CssAtomSet::_MozSvgMarkerOuterSvgAnonChild,
		SvgText: CssAtomSet::_MozSvgText,
		Table: CssAtomSet::_MozTable,
		TableCell: CssAtomSet::_MozTableCell,
		TableColumn: CssAtomSet::_MozTableColumn,
		TableColumnGroup: CssAtomSet::_MozTableColumnGroup,
		TableOuter: CssAtomSet::_MozTableOuter,
		TableRow: CssAtomSet::_MozTableRow,
		TableRowGroup: CssAtomSet::_MozTableRowGroup,
		TableWrapper: CssAtomSet::_MozTableWrapper,
		TextControlEditingRoot: CssAtomSet::_MozTextControlEditingRoot,
		TextControlPreview: CssAtomSet::_MozTextControlPreview,
		TreeCell: CssAtomSet::_MozTreeCell,
		TreeCheckbox: CssAtomSet::_MozTreeCheckbox,
		TreeDropFeedback: CssAtomSet::_MozTreeDropFeedback,
		TreeIndentation: CssAtomSet::_MozTreeIndentation,
		TreeSeparator: CssAtomSet::_MozTreeSeparator,
		VframesetBorder: CssAtomSet::_MozVframesetBorder,
		Viewport: CssAtomSet::_MozViewport,
		ViewportScroll: CssAtomSet::_MozViewportScroll,
	}
);

#[derive(ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
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

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum MozFunctionalPseudoElementKeyword {
	#[atom(CssAtomSet::_MozTreeCell)]
	TreeCell(T![Function]),
	#[atom(CssAtomSet::_MozTreeCellText)]
	TreeCellText(T![Function]),
	#[atom(CssAtomSet::_MozTreeCheckbox)]
	TreeCheckbox(T![Function]),
	#[atom(CssAtomSet::_MozTreeColumn)]
	TreeColumn(T![Function]),
	#[atom(CssAtomSet::_MozTreeImage)]
	TreeImage(T![Function]),
	#[atom(CssAtomSet::_MozTreeLine)]
	TreeLine(T![Function]),
	#[atom(CssAtomSet::_MozTreeRow)]
	TreeRow(T![Function]),
	#[atom(CssAtomSet::_MozTreeSeparator)]
	TreeSeparator(T![Function]),
	#[atom(CssAtomSet::_MozTreeTwisty)]
	TreeTwisty(T![Function]),
}

impl<'a> Parse<'a> for MozFunctionalPseudoElement<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let colons = p.parse::<T![::]>()?;
		let keyword = p.parse::<MozFunctionalPseudoElementKeyword>()?;
		let items = p.parse::<CommaSeparated<'a, T![Ident]>>()?;
		let close = p.parse::<T![')']>()?;
		Ok(match keyword {
			MozFunctionalPseudoElementKeyword::TreeCell(function) => Self::TreeCell(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeCellText(function) => {
				Self::TreeCellText(colons, function, items, close)
			}
			MozFunctionalPseudoElementKeyword::TreeCheckbox(function) => {
				Self::TreeCheckbox(colons, function, items, close)
			}
			MozFunctionalPseudoElementKeyword::TreeColumn(function) => Self::TreeColumn(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeImage(function) => Self::TreeImage(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeLine(function) => Self::TreeLine(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeRow(function) => Self::TreeRow(colons, function, items, close),
			MozFunctionalPseudoElementKeyword::TreeSeparator(function) => {
				Self::TreeSeparator(colons, function, items, close)
			}
			MozFunctionalPseudoElementKeyword::TreeTwisty(function) => Self::TreeTwisty(colons, function, items, close),
		})
	}
}

pseudo_class!(
	/// <https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes>
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum MozPseudoClass {
		Any: CssAtomSet::_MozAny,
		AnyLink: CssAtomSet::_MozAnyLink,
		Broken: CssAtomSet::_MozBroken,
		DragOver: CssAtomSet::_MozDragOver,
		FirstNode: CssAtomSet::_MozFirstNode,
		FocusRing: CssAtomSet::_MozFocusring,
		FullScreen: CssAtomSet::_MozFullScreen,
		FullScreenAncestor: CssAtomSet::_MozFullScreenAncestor,
		HandlerBlocked: CssAtomSet::_MozHandlerBlocked,
		HandlerCrashed: CssAtomSet::_MozHandlerCrashed,
		HandlerDisabled: CssAtomSet::_MozHandlerDisabled,
		LastNode: CssAtomSet::_MozLastNode,
		Loading: CssAtomSet::_MozLoading,
		LwTheme: CssAtomSet::_MozLwtheme,
		LwThemeBrighttext: CssAtomSet::_MozLwthemeBrighttext,
		LwThemeDarktext: CssAtomSet::_MozLwthemeDarktext,
		NativeAnonymous: CssAtomSet::_MozNativeAnonymous,
		OnlyWhitespace: CssAtomSet::_MozOnlyWhitespace,
		PlaceholderShown: CssAtomSet::_MozPlaceholderShown,
		ReadOnly: CssAtomSet::_MozReadOnly,
		ReadWrite: CssAtomSet::_MozReadWrite,
		SubmitInvalid: CssAtomSet::_MozSubmitInvalid,
		Suppressed: CssAtomSet::_MozSuppressed,
		UiInvalid: CssAtomSet::_MozUiInvalid,
		UiValid: CssAtomSet::_MozUiValid,
		UserDisabled: CssAtomSet::_MozUserDisabled,
		WindowInactive: CssAtomSet::_MozWindowInactive,
	}
);

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum MozFunctionalPseudoClass {
	LocaleDir(MozLocaleDirFunctionalPseudoClass),
}

impl<'a> Parse<'a> for MozFunctionalPseudoClass {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.equals_atom(c, &CssAtomSet::_MozLocaleDir) {
			let value = p.parse::<DirValue>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::LocaleDir(MozLocaleDirFunctionalPseudoClass { colon, function, value, close }))
		} else {
			Err(Diagnostic::new(c, Diagnostic::unexpected_function))?
		}
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct MozLocaleDirFunctionalPseudoClass {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
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
		assert_parse!(CssAtomSet::ATOMS, MozPseudoElement, "::-moz-anonymous-block");
		assert_parse!(CssAtomSet::ATOMS, MozFunctionalPseudoElement, "::-moz-tree-twisty(selected,focus)");
	}
}
