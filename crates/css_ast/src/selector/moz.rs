use css_lexer::{Cursor, KindSet};
use css_parse::{
	diagnostics, function_set, pseudo_class, pseudo_element, CursorSink, Parse, Parser, Result as ParserResult,
	ToCursors, T,
};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

use super::functional_pseudo_class::DirValue;

#[visit]
pseudo_element!(
	// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes
	MozPseudoElement {
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

impl<'a> Visitable<'a> for MozPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_moz_pseudo_element(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum MozFunctionalPseudoElement {
	TreeCell(()),
	TreeCellText(()),
	TreeCheckbox(()),
	TreeColumn(()),
	TreeDropFeedback(()),
	TreeImage(()),
	TreeIndentation(()),
	TreeLine(()),
	TreeRow(()),
	TreeSeparator(()),
	TreeTwisty(()),
}

function_set!(MozFunctionalPseudoElementKeyword {
	TreeCell: "-moz-tree-cell",
	TreeCellText: "-moz-tree-cell-text",
	TreeCheckbox: "-moz-tree-checkbox",
	TreeColumn: "-moz-tree-column",
	TreeDropFeedback: "-moz-tree-drop-feedback",
	TreeImage: "-moz-tree-image",
	TreeIndentation: "-moz-tree-indentation",
	TreeLine: "-moz-tree-line",
	TreeRow: "-moz-tree-row",
	TreeSeparator: "-moz-tree-separator",
	TreeTwisty: "-moz-tree-twisty",
});

impl<'a> Parse<'a> for MozFunctionalPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let skip = p.set_skip(KindSet::NONE);
		let keyword = p.parse::<MozFunctionalPseudoElementKeyword>();
		p.set_skip(skip);
		let keyword = keyword?;
		let _c: Cursor = keyword.into();
		todo!()
		// Ok(match keyword {
		// 	MozFunctionalPseudoElementKeyword::TreeCell(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeCellText(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeCheckbox(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeColumn(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeDropFeedback(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeImage(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeIndentation(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeLine(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeRow(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeSeparator(_) => todo!(),
		// 	MozFunctionalPseudoElementKeyword::TreeTwisty(_) => todo!(),
		// })
	}
}

impl<'a> Visitable<'a> for MozFunctionalPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_moz_functional_pseudo_element(self);
	}
}

#[visit]
pseudo_class!(
	// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes
	MozPseudoClass {
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

impl<'a> Visitable<'a> for MozPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_moz_pseudo_class(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> ToCursors for MozFunctionalPseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::LocaleDir(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> Visitable<'a> for MozFunctionalPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_moz_functional_pseudo_class(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct MozLocaleDirFunctionalPseudoClass {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for MozLocaleDirFunctionalPseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colon, s);
		s.append(self.function.into());
		s.append(self.value.into());
		if let Some(open) = self.close {
			s.append(open.into());
		}
	}
}
