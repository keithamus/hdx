use hdx_atom::atom;
use hdx_derive::Atomizable;
use hdx_lexer::Token;
use hdx_parser::{expect, expect_ignore_case, todo, unexpected, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use super::functional_pseudo_class::DirValue;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NonStandardMozPseudoElement {
	#[atomizable("-moz-block-inside-inline-wrapper")]
	BlockInsideInlineWrapper,
	#[atomizable("-moz-button-content")]
	ButtonContent,
	#[atomizable("-moz-canvas")]
	Canvas,
	#[atomizable("-moz-cell-content")]
	CellContent,
	#[atomizable("-moz-color-swatch")]
	ColorSwatch,
	#[atomizable("-moz-column-span-wrapper")]
	ColumnSpanWrapper,
	#[atomizable("-moz-dropdown-list")]
	DropdownList,
	#[atomizable("-moz-fieldset-content")]
	FieldsetContent,
	#[atomizable("-moz-first-letter-continuation")]
	FirstLetterContinuation,
	#[atomizable("-moz-html-canvas-content")]
	HtmlCanvasContent,
	#[atomizable("-moz-inline-table")]
	InlineTable,
	#[atomizable("-moz-line-frame")]
	LineFrame,
	#[atomizable("-moz-mathml-anonymous-block")]
	MathmlAnonymousBlock,
	#[atomizable("-moz-table")]
	Table,
	#[atomizable("-moz-table-cell")]
	TableCell,
	#[atomizable("-moz-table-row")]
	TableRow,
	#[atomizable("-moz-table-row-group")]
	TableRowGroup,
	#[atomizable("-moz-table-wrapper")]
	TableWrapper,
	#[atomizable("-moz-anonymous-item")]
	AnonymousItem,
	#[atomizable("-moz-block-ruby-content")]
	BlockRubyContent,
	#[atomizable("-moz-column-content")]
	ColumnContent,
	#[atomizable("-moz-column-set")]
	ColumnSet,
	#[atomizable("-moz-focus-inner")]
	FocusInner,
	#[atomizable("-moz-frameset-blank")]
	FramesetBlank,
	#[atomizable("-moz-hframeset-border")]
	HframesetBorder,
	#[atomizable("-moz-list-bullet")]
	ListBullet,
	#[atomizable("-moz-list-number")]
	ListNumber,
	#[atomizable("-moz-number-spin-box")]
	NumberSpinBox,
	#[atomizable("-moz-number-spin-down")]
	NumberSpinDown,
	#[atomizable("-moz-number-spin-up")]
	NumberSpinUp,
	#[atomizable("-moz-oof-placeholder")]
	OofPlaceholder,
	#[atomizable("-moz-page")]
	Page,
	#[atomizable("-moz-page-break")]
	PageBreak,
	#[atomizable("-moz-page-content")]
	PageContent,
	#[atomizable("-moz-page-sequence")]
	PageSequence,
	#[atomizable("-moz-printed-sheet")]
	PrintedSheet,
	#[atomizable("-moz-progress-bar")]
	ProgressBar,
	#[atomizable("-moz-range-progress")]
	RangeProgress,
	#[atomizable("-moz-range-thumb")]
	RangeThumb,
	#[atomizable("-moz-range-track")]
	RangeTrack,
	#[atomizable("-moz-reveal")]
	Reveal,
	#[atomizable("-moz-ruby")]
	Ruby,
	#[atomizable("-moz-ruby-base")]
	RubyBase,
	#[atomizable("-moz-ruby-base-container")]
	RubyBaseContainer,
	#[atomizable("-moz-ruby-text")]
	RubyText,
	#[atomizable("-moz-ruby-text-container")]
	RubyTextContainer,
	#[atomizable("-moz-scrolled-content")]
	ScrolledContent,
	#[atomizable("-moz-scrolled-page-sequence")]
	ScrolledPageSequence,
	#[atomizable("-moz-search-clear-button")]
	SearchClearButton,
	#[atomizable("-moz-svg-foreign-content")]
	SvgForeignContent,
	#[atomizable("-moz-svg-marker-anon-child")]
	SvgMarkerAnonChild,
	#[atomizable("-moz-svg-marker-outer-svg-anon-child")]
	SvgMarkerOuterSvgAnonChild,
	#[atomizable("-moz-svg-text")]
	SvgText,
	#[atomizable("-moz-table-column")]
	TableColumn,
	#[atomizable("-moz-table-column-group")]
	TableColumnGroup,
	#[atomizable("-moz-text-control-editing-root")]
	TextControlEditingRoot,
	#[atomizable("-moz-text-control-preview")]
	TextControlPreview,
	#[atomizable("-moz-tree-cell")]
	TreeCell,
	#[atomizable("-moz-tree-checkbox")]
	TreeCheckbox,
	#[atomizable("-moz-tree-drop-feedback")]
	TreeDropFeedback,
	#[atomizable("-moz-tree-indentation")]
	TreeIndentation,
	#[atomizable("-moz-tree-separator")]
	TreeSeparator,
	#[atomizable("-moz-vframeset-border")]
	VframesetBorder,
	#[atomizable("-moz-viewport")]
	Viewport,
	#[atomizable("-moz-viewport-scroll")]
	ViewportScroll,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NonStandardMozFunctionalPseudoElement {
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

impl<'a> Parse<'a> for NonStandardMozFunctionalPseudoElement {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Function(_):
			atom!("-moz-tree-cell") => todo!(parser),
			atom!("-moz-tree-cell-text") => todo!(parser),
			atom!("-moz-tree-checkbox") => todo!(parser),
			atom!("-moz-tree-column") => todo!(parser),
			atom!("-moz-tree-drop-feedback") => todo!(parser),
			atom!("-moz-tree-image") => todo!(parser),
			atom!("-moz-tree-indentation") => todo!(parser),
			atom!("-moz-tree-line") => todo!(parser),
			atom!("-moz-tree-row") => todo!(parser),
			atom!("-moz-tree-separator") => todo!(parser),
			atom!("-moz-tree-twisty") => todo!(parser),
		}
	}
}

impl<'a> WriteCss<'a> for NonStandardMozFunctionalPseudoElement {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		std::todo!()
	}
}

// https://searchfox.org/mozilla-central/source/xpcom/ds/StaticAtoms.py#2502
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NonStandardMozPseudoClass {
	#[atomizable("-moz-broken")]
	Broken,
	#[atomizable("-moz-drag-over")]
	DragOver,
	#[atomizable("-moz-first-node")]
	FirstNode,
	#[atomizable("-moz-focusring")]
	FocusRing,
	#[atomizable("-moz-handler-blocked")]
	HandlerBlocked,
	#[atomizable("-moz-handler-crashed")]
	HandlerCrashed,
	#[atomizable("-moz-handler-disabled")]
	HandlerDisabled,
	#[atomizable("-moz-last-node")]
	LastNode,
	#[atomizable("-moz-loading")]
	Loading,
	#[atomizable("-moz-only-whitespace")]
	OnlyWhitespace,
	#[atomizable("-moz-submit-invalid")]
	SubmitInvalid,
	#[atomizable("-moz-suppressed")]
	Suppressed,
	#[atomizable("-moz-user-disabled")]
	UserDisabled,
	#[atomizable("-moz-window-inactive")]
	WindowInactive,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum NonStandardMozFunctionalPseudoClass {
	LocaleDir(DirValue),
}

impl<'a> Parse<'a> for NonStandardMozFunctionalPseudoClass {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Function(_):
			atom!("-moz-locale-dir") => {
				if let Some(dir) = DirValue::from_token(&parser.next()) {
					expect!(parser.next(), Token::RightParen);
					Ok(Self::LocaleDir(dir))
				} else {
					unexpected!(parser)
				}
			}
		}
	}
}

impl<'a> WriteCss<'a> for NonStandardMozFunctionalPseudoClass {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::LocaleDir(dir) => {
				atom!("-moz-locale-dir").write_css(sink)?;
				sink.write_char('(')?;
				dir.write_css(sink)?;
				sink.write_char(')')
			}
		}
	}
}
