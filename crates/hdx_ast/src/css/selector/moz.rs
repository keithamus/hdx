use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{diagnostics, todo, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::functional_pseudo_class::DirValue;

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#pseudo-elements_and_pseudo-classes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum MozPseudoElement {
	AnonymousBlock(T![::], T![Ident]),
	AnonymousItem(T![::], T![Ident]),
	AnonymousPositionedBlock(T![::], T![Ident]),
	BlockInsideInlineWrapper(T![::], T![Ident]),
	BlockRubyContent(T![::], T![Ident]),
	ButtonContent(T![::], T![Ident]),
	Canvas(T![::], T![Ident]),
	CellContent(T![::], T![Ident]),
	ColorSwatch(T![::], T![Ident]),
	ColumnContent(T![::], T![Ident]),
	ColumnSet(T![::], T![Ident]),
	ColumnSpanWrapper(T![::], T![Ident]),
	DropdownList(T![::], T![Ident]),
	FieldsetContent(T![::], T![Ident]),
	FirstLetterContinuation(T![::], T![Ident]),
	FocusInner(T![::], T![Ident]),
	FocusOuter(T![::], T![Ident]),
	FramesetBlank(T![::], T![Ident]),
	HframesetBorder(T![::], T![Ident]),
	HtmlCanvasContent(T![::], T![Ident]),
	InlineTable(T![::], T![Ident]),
	LineFrame(T![::], T![Ident]),
	ListBullet(T![::], T![Ident]),
	ListNumber(T![::], T![Ident]),
	MathmlAnonymousBlock(T![::], T![Ident]),
	NumberSpinBox(T![::], T![Ident]),
	NumberSpinDown(T![::], T![Ident]),
	NumberSpinUp(T![::], T![Ident]),
	OofPlaceholder(T![::], T![Ident]),
	Page(T![::], T![Ident]),
	PageBreak(T![::], T![Ident]),
	PageContent(T![::], T![Ident]),
	PageSequence(T![::], T![Ident]),
	Pagebreak(T![::], T![Ident]),
	Pagecontent(T![::], T![Ident]),
	Placeholder(T![::], T![Ident]),
	PrintedSheet(T![::], T![Ident]),
	ProgressBar(T![::], T![Ident]),
	RangeProgress(T![::], T![Ident]),
	RangeThumb(T![::], T![Ident]),
	RangeTrack(T![::], T![Ident]),
	Reveal(T![::], T![Ident]),
	Ruby(T![::], T![Ident]),
	RubyBase(T![::], T![Ident]),
	RubyBaseContainer(T![::], T![Ident]),
	RubyText(T![::], T![Ident]),
	RubyTextContainer(T![::], T![Ident]),
	ScrolledCanvas(T![::], T![Ident]),
	ScrolledContent(T![::], T![Ident]),
	ScrolledPageSequence(T![::], T![Ident]),
	SearchClearButton(T![::], T![Ident]),
	Selection(T![::], T![Ident]),
	SvgForeignContent(T![::], T![Ident]),
	SvgMarkerAnonChild(T![::], T![Ident]),
	SvgMarkerOuterSvgAnonChild(T![::], T![Ident]),
	SvgText(T![::], T![Ident]),
	Table(T![::], T![Ident]),
	TableCell(T![::], T![Ident]),
	TableColumn(T![::], T![Ident]),
	TableColumnGroup(T![::], T![Ident]),
	TableOuter(T![::], T![Ident]),
	TableRow(T![::], T![Ident]),
	TableRowGroup(T![::], T![Ident]),
	TableWrapper(T![::], T![Ident]),
	TextControlEditingRoot(T![::], T![Ident]),
	TextControlPreview(T![::], T![Ident]),
	TreeCell(T![::], T![Ident]),
	TreeCheckbox(T![::], T![Ident]),
	TreeDropFeedback(T![::], T![Ident]),
	TreeIndentation(T![::], T![Ident]),
	TreeSeparator(T![::], T![Ident]),
	VframesetBorder(T![::], T![Ident]),
	Viewport(T![::], T![Ident]),
	ViewportScroll(T![::], T![Ident]),
}

impl<'a> Parse<'a> for MozPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colons = p.parse::<T![::]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colons = colons?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-moz-anonymous-block") => Self::AnonymousBlock(colons, ident),
			atom!("-moz-anonymous-item") => Self::AnonymousItem(colons, ident),
			atom!("-moz-anonymous-positioned-block") => Self::AnonymousPositionedBlock(colons, ident),
			atom!("-moz-block-inside-inline-wrapper") => Self::BlockInsideInlineWrapper(colons, ident),
			atom!("-moz-block-ruby-content") => Self::BlockRubyContent(colons, ident),
			atom!("-moz-button-content") => Self::ButtonContent(colons, ident),
			atom!("-moz-canvas") => Self::Canvas(colons, ident),
			atom!("-moz-cell-content") => Self::CellContent(colons, ident),
			atom!("-moz-color-swatch") => Self::ColorSwatch(colons, ident),
			atom!("-moz-column-content") => Self::ColumnContent(colons, ident),
			atom!("-moz-column-set") => Self::ColumnSet(colons, ident),
			atom!("-moz-column-span-wrapper") => Self::ColumnSpanWrapper(colons, ident),
			atom!("-moz-dropdown-list") => Self::DropdownList(colons, ident),
			atom!("-moz-fieldset-content") => Self::FieldsetContent(colons, ident),
			atom!("-moz-first-letter-continuation") => Self::FirstLetterContinuation(colons, ident),
			atom!("-moz-focus-inner") => Self::FocusInner(colons, ident),
			atom!("-moz-focus-outer") => Self::FocusOuter(colons, ident),
			atom!("-moz-frameset-blank") => Self::FramesetBlank(colons, ident),
			atom!("-moz-hframeset-border") => Self::HframesetBorder(colons, ident),
			atom!("-moz-html-canvas-content") => Self::HtmlCanvasContent(colons, ident),
			atom!("-moz-inline-table") => Self::InlineTable(colons, ident),
			atom!("-moz-line-frame") => Self::LineFrame(colons, ident),
			atom!("-moz-list-bullet") => Self::ListBullet(colons, ident),
			atom!("-moz-list-number") => Self::ListNumber(colons, ident),
			atom!("-moz-mathml-anonymous-block") => Self::MathmlAnonymousBlock(colons, ident),
			atom!("-moz-number-spin-box") => Self::NumberSpinBox(colons, ident),
			atom!("-moz-number-spin-down") => Self::NumberSpinDown(colons, ident),
			atom!("-moz-number-spin-up") => Self::NumberSpinUp(colons, ident),
			atom!("-moz-oof-placeholder") => Self::OofPlaceholder(colons, ident),
			atom!("-moz-page") => Self::Page(colons, ident),
			atom!("-moz-page-break") => Self::PageBreak(colons, ident),
			atom!("-moz-page-content") => Self::PageContent(colons, ident),
			atom!("-moz-page-sequence") => Self::PageSequence(colons, ident),
			atom!("-moz-pagebreak") => Self::Pagebreak(colons, ident),
			atom!("-moz-pagecontent") => Self::Pagecontent(colons, ident),
			atom!("-moz-placeholder") => Self::Placeholder(colons, ident),
			atom!("-moz-printed-sheet") => Self::PrintedSheet(colons, ident),
			atom!("-moz-progress-bar") => Self::ProgressBar(colons, ident),
			atom!("-moz-range-progress") => Self::RangeProgress(colons, ident),
			atom!("-moz-range-thumb") => Self::RangeThumb(colons, ident),
			atom!("-moz-range-track") => Self::RangeTrack(colons, ident),
			atom!("-moz-reveal") => Self::Reveal(colons, ident),
			atom!("-moz-ruby") => Self::Ruby(colons, ident),
			atom!("-moz-ruby-base") => Self::RubyBase(colons, ident),
			atom!("-moz-ruby-base-container") => Self::RubyBaseContainer(colons, ident),
			atom!("-moz-ruby-text") => Self::RubyText(colons, ident),
			atom!("-moz-ruby-text-container") => Self::RubyTextContainer(colons, ident),
			atom!("-moz-scrolled-canvas") => Self::ScrolledCanvas(colons, ident),
			atom!("-moz-scrolled-content") => Self::ScrolledContent(colons, ident),
			atom!("-moz-scrolled-page-sequence") => Self::ScrolledPageSequence(colons, ident),
			atom!("-moz-search-clear-button") => Self::SearchClearButton(colons, ident),
			atom!("-moz-selection") => Self::Selection(colons, ident),
			atom!("-moz-svg-foreign-content") => Self::SvgForeignContent(colons, ident),
			atom!("-moz-svg-marker-anon-child") => Self::SvgMarkerAnonChild(colons, ident),
			atom!("-moz-svg-marker-outer-svg-anon-child") => Self::SvgMarkerOuterSvgAnonChild(colons, ident),
			atom!("-moz-svg-text") => Self::SvgText(colons, ident),
			atom!("-moz-table") => Self::Table(colons, ident),
			atom!("-moz-table-cell") => Self::TableCell(colons, ident),
			atom!("-moz-table-column") => Self::TableColumn(colons, ident),
			atom!("-moz-table-column-group") => Self::TableColumnGroup(colons, ident),
			atom!("-moz-table-outer") => Self::TableOuter(colons, ident),
			atom!("-moz-table-row") => Self::TableRow(colons, ident),
			atom!("-moz-table-row-group") => Self::TableRowGroup(colons, ident),
			atom!("-moz-table-wrapper") => Self::TableWrapper(colons, ident),
			atom!("-moz-text-control-editing-root") => Self::TextControlEditingRoot(colons, ident),
			atom!("-moz-text-control-preview") => Self::TextControlPreview(colons, ident),
			atom!("-moz-tree-cell") => Self::TreeCell(colons, ident),
			atom!("-moz-tree-checkbox") => Self::TreeCheckbox(colons, ident),
			atom!("-moz-tree-drop-feedback") => Self::TreeDropFeedback(colons, ident),
			atom!("-moz-tree-indentation") => Self::TreeIndentation(colons, ident),
			atom!("-moz-tree-separator") => Self::TreeSeparator(colons, ident),
			atom!("-moz-vframeset-border") => Self::VframesetBorder(colons, ident),
			atom!("-moz-viewport") => Self::Viewport(colons, ident),
			atom!("-moz-viewport-scroll") => Self::ViewportScroll(colons, ident),
			atom => Err(diagnostics::UnexpectedPseudoElement(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors<'a> for MozPseudoElement {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::AnonymousBlock(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::AnonymousItem(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::AnonymousPositionedBlock(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::BlockInsideInlineWrapper(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::BlockRubyContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ButtonContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Canvas(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::CellContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ColorSwatch(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ColumnContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ColumnSet(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ColumnSpanWrapper(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::DropdownList(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FieldsetContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FirstLetterContinuation(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FocusInner(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FocusOuter(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FramesetBlank(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::HframesetBorder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::HtmlCanvasContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::InlineTable(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::LineFrame(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ListBullet(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ListNumber(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::MathmlAnonymousBlock(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::NumberSpinBox(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::NumberSpinDown(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::NumberSpinUp(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::OofPlaceholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Page(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::PageBreak(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::PageContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::PageSequence(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Pagebreak(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Pagecontent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Placeholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::PrintedSheet(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ProgressBar(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RangeProgress(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RangeThumb(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RangeTrack(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Reveal(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Ruby(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RubyBase(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RubyBaseContainer(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RubyText(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::RubyTextContainer(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrolledCanvas(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrolledContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrolledPageSequence(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SearchClearButton(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Selection(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SvgForeignContent(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SvgMarkerAnonChild(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SvgMarkerOuterSvgAnonChild(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SvgText(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Table(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableCell(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableColumn(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableColumnGroup(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableOuter(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableRow(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableRowGroup(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TableWrapper(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TextControlEditingRoot(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TextControlPreview(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TreeCell(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TreeCheckbox(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TreeDropFeedback(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TreeIndentation(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TreeSeparator(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::VframesetBorder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Viewport(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ViewportScroll(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
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

impl<'a> Parse<'a> for MozFunctionalPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let _colons = p.parse::<T![::]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-moz-tree-cell") => todo!(p),
			atom!("-moz-tree-cell-text") => todo!(p),
			atom!("-moz-tree-checkbox") => todo!(p),
			atom!("-moz-tree-column") => todo!(p),
			atom!("-moz-tree-drop-feedback") => todo!(p),
			atom!("-moz-tree-image") => todo!(p),
			atom!("-moz-tree-indentation") => todo!(p),
			atom!("-moz-tree-line") => todo!(p),
			atom!("-moz-tree-row") => todo!(p),
			atom!("-moz-tree-separator") => todo!(p),
			atom!("-moz-tree-twisty") => todo!(p),
			atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
		})
	}
}

// https://searchfox.org/mozilla-central/source/xpcom/ds/StaticAtoms.py#2502
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum MozPseudoClass {
	Any(T![:], T![Ident]),
	AnyLink(T![:], T![Ident]),
	Broken(T![:], T![Ident]),
	DragOver(T![:], T![Ident]),
	FirstNode(T![:], T![Ident]),
	FocusRing(T![:], T![Ident]),
	FullScreen(T![:], T![Ident]),
	FullScreenAncestor(T![:], T![Ident]),
	HandlerBlocked(T![:], T![Ident]),
	HandlerCrashed(T![:], T![Ident]),
	HandlerDisabled(T![:], T![Ident]),
	LastNode(T![:], T![Ident]),
	Loading(T![:], T![Ident]),
	LwTheme(T![:], T![Ident]),
	LwThemeBrighttext(T![:], T![Ident]),
	LwThemeDarktext(T![:], T![Ident]),
	NativeAnonymous(T![:], T![Ident]),
	OnlyWhitespace(T![:], T![Ident]),
	PlaceholderShown(T![:], T![Ident]),
	ReadOnly(T![:], T![Ident]),
	ReadWrite(T![:], T![Ident]),
	SubmitInvalid(T![:], T![Ident]),
	Suppressed(T![:], T![Ident]),
	UiInvalid(T![:], T![Ident]),
	UiValid(T![:], T![Ident]),
	UserDisabled(T![:], T![Ident]),
	WindowInactive(T![:], T![Ident]),
}

impl<'a> Parse<'a> for MozPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-moz-any") => Self::Any(colon, ident),
			atom!("-moz-any-link") => Self::AnyLink(colon, ident),
			atom!("-moz-broken") => Self::Broken(colon, ident),
			atom!("-moz-drag-over") => Self::DragOver(colon, ident),
			atom!("-moz-first-node") => Self::FirstNode(colon, ident),
			atom!("-moz-focusring") => Self::FocusRing(colon, ident),
			atom!("-moz-full-screen") => Self::FullScreen(colon, ident),
			atom!("-moz-full-screen-ancestor") => Self::FullScreenAncestor(colon, ident),
			atom!("-moz-handler-blocked") => Self::HandlerBlocked(colon, ident),
			atom!("-moz-handler-crashed") => Self::HandlerCrashed(colon, ident),
			atom!("-moz-handler-disabled") => Self::HandlerDisabled(colon, ident),
			atom!("-moz-last-node") => Self::LastNode(colon, ident),
			atom!("-moz-loading") => Self::Loading(colon, ident),
			atom!("-moz-lwtheme") => Self::LwTheme(colon, ident),
			atom!("-moz-lwtheme-brighttext") => Self::LwThemeBrighttext(colon, ident),
			atom!("-moz-lwtheme-darktext") => Self::LwThemeDarktext(colon, ident),
			atom!("-moz-native-anonymous") => Self::NativeAnonymous(colon, ident),
			atom!("-moz-only-whitespace") => Self::OnlyWhitespace(colon, ident),
			atom!("-moz-placeholder-shown") => Self::PlaceholderShown(colon, ident),
			atom!("-moz-read-only") => Self::ReadOnly(colon, ident),
			atom!("-moz-read-write") => Self::ReadWrite(colon, ident),
			atom!("-moz-submit-invalid") => Self::SubmitInvalid(colon, ident),
			atom!("-moz-suppressed") => Self::Suppressed(colon, ident),
			atom!("-moz-ui-invalid") => Self::UiInvalid(colon, ident),
			atom!("-moz-ui-valid") => Self::UiValid(colon, ident),
			atom!("-moz-user-disabled") => Self::UserDisabled(colon, ident),
			atom!("-moz-window-inactive") => Self::WindowInactive(colon, ident),
			atom => Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors<'a> for MozPseudoClass {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Any(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::AnyLink(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Broken(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::DragOver(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FirstNode(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FocusRing(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FullScreen(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FullScreenAncestor(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::HandlerBlocked(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::HandlerCrashed(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::HandlerDisabled(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LastNode(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Loading(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LwTheme(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LwThemeBrighttext(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::LwThemeDarktext(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::NativeAnonymous(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::OnlyWhitespace(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::PlaceholderShown(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::ReadOnly(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::ReadWrite(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::SubmitInvalid(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Suppressed(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::UiInvalid(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::UiValid(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::UserDisabled(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::WindowInactive(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum MozFunctionalPseudoClass {
	LocaleDir(MozLocaleDirFunctionalPseudoClass),
}

impl<'a> Parse<'a> for MozFunctionalPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		match p.parse_atom_lower(c) {
			atom!("-moz-locale-dir") => {
				let value = p.parse::<DirValue>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::LocaleDir(MozLocaleDirFunctionalPseudoClass { colon, function, value, close }))
			}
			atom => Err(diagnostics::UnexpectedFunction(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors<'a> for MozFunctionalPseudoClass {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::LocaleDir(c) => ToCursors::to_cursors(c, s),
		}
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

impl<'a> ToCursors<'a> for MozLocaleDirFunctionalPseudoClass {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		ToCursors::to_cursors(&self.colon, s);
		s.append(self.function.into());
		s.append(self.value.into());
		if let Some(open) = self.close {
			s.append(open.into());
		}
	}
}
