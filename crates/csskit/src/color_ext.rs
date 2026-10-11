#[cfg(not(any(feature = "anstyle", feature = "owo-colors")))]
compile_error!("At least one of 'anstyle' or 'owo-colors' features must be enabled");

use chromashift::Srgb;

/// Set foreground color on text using a chromashift color type
#[cfg(feature = "anstyle")]
pub fn fg<T, C>(text: T, color: C) -> String
where
	T: std::fmt::Display,
	C: Into<Srgb>,
{
	use anstyle::Style;
	let srgb: Srgb = color.into();
	let color = anstyle::RgbColor::from(srgb);
	let style = Style::new().fg_color(Some(color.into()));
	format!("{style}{text}{style:#}")
}

/// Set foreground color on text using a chromashift color type
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn fg<T, C>(text: T, color: C) -> String
where
	T: std::fmt::Display,
	C: Into<Srgb>,
{
	use owo_colors::OwoColorize;
	let [red, green, blue] = Into::<Srgb>::into(color).to_u8();
	format!("{}", text.truecolor(red, green, blue))
}

/// Set background color on text using a chromashift color type
#[cfg(feature = "anstyle")]
pub fn bg<T, C>(text: T, color: C) -> String
where
	T: std::fmt::Display,
	C: Into<Srgb>,
{
	use anstyle::Style;
	let srgb: Srgb = color.into();
	let color = anstyle::RgbColor::from(srgb);
	let style = Style::new().bg_color(Some(color.into()));
	format!("{style}{text}{style:#}")
}

/// Set background color on text using a chromashift color type
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn bg<T, C>(text: T, color: C) -> String
where
	T: std::fmt::Display,
	C: Into<Srgb>,
{
	use owo_colors::OwoColorize;
	let [red, green, blue] = Into::<Srgb>::into(color).to_u8();
	format!("{}", text.on_truecolor(red, green, blue))
}

#[cfg(feature = "anstyle")]
pub fn red<T: std::fmt::Display>(text: T) -> String {
	use anstyle::{AnsiColor, Style};
	let style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)));
	format!("{style}{text}{style:#}")
}

#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn red<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.red())
}

#[cfg(feature = "anstyle")]
pub fn bold_red<T: std::fmt::Display>(text: T) -> String {
	use anstyle::{AnsiColor, Style};
	let style = Style::new().bold().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)));
	format!("{style}{text}{style:#}")
}

#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn bold_red<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.bold().red())
}

#[cfg(feature = "anstyle")]
pub fn bold_green<T: std::fmt::Display>(text: T) -> String {
	use anstyle::{AnsiColor, Style};
	let style = Style::new().bold().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
	format!("{style}{text}{style:#}")
}

#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn bold_green<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.bold().green())
}

/// Color text magenta
#[cfg(feature = "anstyle")]
pub fn magenta<T: std::fmt::Display>(text: T) -> String {
	use anstyle::{AnsiColor, Style};
	let style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Magenta)));
	format!("{style}{text}{style:#}")
}

/// Color text magenta
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn magenta<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.magenta())
}

/// Color text green
#[cfg(feature = "anstyle")]
pub fn green<T: std::fmt::Display>(text: T) -> String {
	use anstyle::{AnsiColor, Style};
	let style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
	format!("{style}{text}{style:#}")
}

/// Color text green
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn green<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.green())
}

/// Make text bold
#[cfg(feature = "anstyle")]
pub fn bold<T: std::fmt::Display>(text: T) -> String {
	use anstyle::Style;
	let style = Style::new().bold();
	format!("{style}{text}{style:#}")
}

/// Make text bold
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn bold<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.bold())
}

/// Make text dimmed
#[cfg(feature = "anstyle")]
pub fn dimmed<T: std::fmt::Display>(text: T) -> String {
	use anstyle::Style;
	let style = Style::new().dimmed();
	format!("{style}{text}{style:#}")
}

/// Make text dimmed
#[cfg(feature = "owo-colors")]
#[cfg(not(feature = "anstyle"))]
pub fn dimmed<T: std::fmt::Display>(text: T) -> String {
	use owo_colors::OwoColorize;
	format!("{}", text.dimmed())
}
