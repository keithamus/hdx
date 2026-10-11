use crate::{AnsiTheme, SemanticDecoration, TokenHighlighter};
use chromashift::{Named, WcagColorContrast};
use css_lexer::{ToSpan, Token};
use css_parse::{SourceCursor, SourceCursorSink};
use std::fmt::{Result, Write};

#[cfg(feature = "anstyle")]
use anstyle::Style;
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
use owo_colors::OwoColorize;

pub struct AnsiHighlightCursorStream<'h, W: Write, T: AnsiTheme> {
	writer: W,
	theme: T,
	highlighter: &'h TokenHighlighter,
	last_token: Option<Token>,
	err: Result,
}

impl<'h, W: Write, T: AnsiTheme> AnsiHighlightCursorStream<'h, W, T> {
	pub fn new(writer: W, highlighter: &'h TokenHighlighter, theme: T) -> Self {
		Self { writer, highlighter, theme, last_token: None, err: Ok(()) }
	}
}

impl<'a, 'h, W: Write, T: AnsiTheme> SourceCursorSink<'a> for AnsiHighlightCursorStream<'h, W, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		if self.err.is_err() {
			return;
		}
		if let Some(last) = self.last_token
			&& last.needs_separator_for(c.token())
		{
			self.err = self.writer.write_char(' ');
		}
		self.last_token = Some(c.token());
		if let Some(highlight) = self.highlighter.get(c.to_span()) {
			if let SemanticDecoration::BackgroundColor(bg) = highlight.decoration() {
				let fg = if bg.wcag_contrast_ratio(Named::White) > bg.wcag_contrast_ratio(Named::Black) {
					Named::White
				} else {
					Named::Black
				};
				#[cfg(feature = "anstyle")]
				{
					let style = Style::new().bg_color(Some(bg.into())).fg_color(Some(fg.into()));
					self.err = write!(&mut self.writer, "{style}{c}{style:#}");
				}
				#[cfg(feature = "owo-colors")]
				#[cfg(not(feature = "anstyle"))]
				{
					use chromashift::Srgb;
					let [bg_red, bg_green, bg_blue] = Srgb::from(bg).to_u8();
					let [fg_red, fg_green, fg_blue] = Srgb::from(fg).to_u8();
					self.err = write!(
						&mut self.writer,
						"{}",
						c.to_string().truecolor(fg_red, fg_green, fg_blue).on_truecolor(bg_red, bg_green, bg_blue)
					);
				}
			} else {
				#[cfg(feature = "anstyle")]
				{
					let style = self.theme.get_anstyle(highlight.kind(), highlight.modifier());
					self.err = write!(&mut self.writer, "{style}{c}{style:#}");
				}
				#[cfg(feature = "owo-colors")]
				#[cfg(not(feature = "anstyle"))]
				{
					let style = self.theme.get_owo_style(highlight.kind(), highlight.modifier());
					self.err = write!(&mut self.writer, "{}", style.style(c.to_string()));
				}
			}
		} else {
			self.err = write!(&mut self.writer, "{c}");
		}
	}
}
