use crate::{AnsiTheme, DefaultAnsiTheme, SemanticDecoration, TokenHighlighter};
use css_ast::{CssAtomSet, StyleSheet, Visitable};
use css_lexer::{Lexer, SourceOffset, Span};
use miette::SpanContents;
use miette::highlighters::{Highlighter, HighlighterState};
use owo_colors::Style;

/// Miette highlighter for CSS syntax highlighting
pub struct CssHighlighter {
	source: String,
	token_colors: TokenHighlighter,
}

impl CssHighlighter {
	pub fn new(source: String, stylesheet: &StyleSheet) -> Self {
		let mut token_colors = TokenHighlighter::new();
		let _ = stylesheet.accept(&mut token_colors);
		Self { source, token_colors }
	}
}

impl Highlighter for CssHighlighter {
	fn start_highlighter_state<'h>(&'h self, source: &dyn SpanContents<'_>) -> Box<dyn HighlighterState + 'h> {
		Box::new(CssHighlighterState::new(&self.source, &self.token_colors, source.line()))
	}
}

struct CssHighlighterState<'a> {
	source: &'a str,
	token_colors: &'a TokenHighlighter,
	current_line: usize,
	theme: DefaultAnsiTheme,
}

impl<'a> CssHighlighterState<'a> {
	fn new(source: &'a str, token_colors: &'a TokenHighlighter, current_line: usize) -> Self {
		Self { source, token_colors, theme: DefaultAnsiTheme, current_line }
	}
}

impl<'a> HighlighterState for CssHighlighterState<'a> {
	fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>> {
		let mut result = Vec::new();

		// Find the byte offset of this line in the source
		let line_offset: usize = self.source.lines().take(self.current_line).map(|l| l.len() + 1).sum();
		self.current_line += 1;

		// Lex just this line
		let lexer = Lexer::new(&CssAtomSet::ATOMS, line);
		let mut last_end = 0;

		for cursor in lexer {
			let start = cursor.offset().0 as usize;
			let token_str = cursor.str_slice(line);
			let end = start + token_str.len();

			// Get the color for this token based on its global position
			let global_offset = line_offset + start;
			let span =
				Span::new(SourceOffset(global_offset as u32), SourceOffset((global_offset + token_str.len()) as u32));
			let color = self.token_colors.get(span);

			// Add any whitespace before this token
			if start > last_end {
				result.push(Style::default().style(&line[last_end..start]));
			}

			// Add the colored token - use csskit_highlight color if available
			let token_str = &line[start..end];

			// Check for background color decoration first (for color values)
			let style = if let Some(highlight) = color
				&& let SemanticDecoration::BackgroundColor(bg) = highlight.decoration()
			{
				use chromashift::{Named, Srgb, WcagColorContrast};

				// Choose contrasting foreground color
				let fg = if bg.wcag_contrast_ratio(Named::White) > bg.wcag_contrast_ratio(Named::Black) {
					Named::White
				} else {
					Named::Black
				};

				// Convert to Srgb to get RGB components
				let [bg_red, bg_green, bg_blue] = Srgb::from(bg).to_u8();
				let [fg_red, fg_green, fg_blue] = Srgb::from(fg).to_u8();

				Style::new().truecolor(fg_red, fg_green, fg_blue).on_truecolor(bg_red, bg_green, bg_blue)
			} else if let Some(highlight) = color {
				// Use the theme to get the owo-colors style
				self.theme.get_owo_style(highlight.kind(), highlight.modifier())
			} else {
				// Fallback: just return unstyled
				Style::default()
			};

			result.push(style.style(token_str));
			last_end = end;
		}

		// Add any remaining text
		if last_end < line.len() {
			result.push(Style::default().style(&line[last_end..]));
		}

		result
	}
}
