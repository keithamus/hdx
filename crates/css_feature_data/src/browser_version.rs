#[cfg(feature = "browserslist")]
use browserslist::Distrib;
use core::num::ParseIntError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BrowserVersion(pub u16, pub u16);

impl BrowserVersion {
	pub fn from_string(version: &str) -> Result<Self, ParseIntError> {
		let parts: Vec<&str> = version.split('.').collect();
		if parts.is_empty() {
			return Ok(BrowserVersion(version.parse()?, 0));
		}
		let major: u16 = parts[0].parse()?;
		let minor: u16 = if parts.len() > 1 { parts[1].parse()? } else { 0 };
		Ok(Self(major, minor))
	}

	pub fn from_string_as_range(version: &str) -> Result<(Self, Self), ParseIntError> {
		if version == "all" {
			return Ok((BrowserVersion(0, 0), BrowserVersion(u16::MAX, u16::MAX)));
		}
		let parts: Vec<&str> = version.split('-').collect();
		if parts.is_empty() || parts.len() < 2 {
			let res = Self::from_string(version)?;
			return Ok((res, res));
		}
		let first = Self::from_string(parts[0])?;
		let second = Self::from_string(parts[1])?;
		if first > second {
			return Ok((second, first));
		}
		Ok((first, second))
	}

	pub fn in_range(self, range: (Self, Self)) -> bool {
		range.0 < self && self < range.1
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NamedBrowserVersion {
	Chrome(BrowserVersion),
	ChromeAndroid(BrowserVersion),
	Edge(BrowserVersion),
	Firefox(BrowserVersion),
	FirefoxAndroid(BrowserVersion),
	Safari(BrowserVersion),
	SafariIos(BrowserVersion),
	Samsung(BrowserVersion),
	Opera(BrowserVersion),
	OperaMini(BrowserVersion),
	OperaMobile(BrowserVersion),
	QQ(BrowserVersion),
	QQAndroid(BrowserVersion),
	UCBrowser(BrowserVersion),
	KaiOS(BrowserVersion),
	AndroidWebView(BrowserVersion),
	InternetExplorer(BrowserVersion),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamedBrowserVersionErr {
	ParseIntError(ParseIntError),
	UnknownNameError(String),
}

#[cfg(feature = "browserslist")]
impl TryFrom<Distrib> for NamedBrowserVersion {
	type Error = NamedBrowserVersionErr;

	fn try_from(value: Distrib) -> Result<Self, Self::Error> {
		let ver =
			BrowserVersion::from_string_as_range(value.version()).map_err(NamedBrowserVersionErr::ParseIntError)?.0;
		match value.name() {
			"chrome" => Ok(Self::Chrome(ver)),
			"chrome_android" | "and_chr" => Ok(Self::ChromeAndroid(ver)),
			"edge" => Ok(Self::Edge(ver)),
			"firefox" => Ok(Self::Firefox(ver)),
			"firefox_android" | "and_ff" => Ok(Self::FirefoxAndroid(ver)),
			"safari" => Ok(Self::Safari(ver)),
			"safari_ios" | "ios_saf" => Ok(Self::SafariIos(ver)),
			"samsung" => Ok(Self::Samsung(ver)),
			"opera" => Ok(Self::Opera(ver)),
			"op_mini" => Ok(Self::OperaMini(ver)),
			"op_mob" => Ok(Self::OperaMobile(ver)),
			"qq" => Ok(Self::QQ(ver)),
			"and_qq" => Ok(Self::QQAndroid(ver)),
			"kaios" => Ok(Self::KaiOS(ver)),
			"android" => Ok(Self::AndroidWebView(ver)),
			"and_uc" => Ok(Self::UCBrowser(ver)),
			"ie" => Ok(Self::InternetExplorer(ver)),
			name => Err(NamedBrowserVersionErr::UnknownNameError(name.to_owned())),
		}
	}
}

impl NamedBrowserVersion {
	/// Reports whether [crate::BrowserSupport] holds data for this browser.
	pub fn has_support_data(self) -> bool {
		matches!(
			self,
			Self::Chrome(_)
				| Self::ChromeAndroid(_)
				| Self::Edge(_)
				| Self::Firefox(_)
				| Self::FirefoxAndroid(_)
				| Self::Safari(_)
				| Self::SafariIos(_)
		)
	}

	/// Converts resolved browserslist targets, keeping the browsers that [crate::BrowserSupport] holds data for.
	#[cfg(feature = "browserslist")]
	pub fn from_distribs(distribs: impl IntoIterator<Item = Distrib>) -> Vec<Self> {
		distribs
			.into_iter()
			.filter_map(|distrib| Self::try_from(distrib).ok())
			.filter(|browser| browser.has_support_data())
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_browserversion_from_string() {
		assert_eq!(BrowserVersion::from_string("29").unwrap(), BrowserVersion(29, 0));
		assert_eq!(BrowserVersion::from_string("29.1").unwrap(), BrowserVersion(29, 1));
		assert_eq!(BrowserVersion::from_string("29.12").unwrap(), BrowserVersion(29, 12));
		assert_eq!(BrowserVersion::from_string("120").unwrap(), BrowserVersion(120, 0));
		assert_eq!(BrowserVersion::from_string("10.0").unwrap(), BrowserVersion(10, 0));
		assert_eq!(BrowserVersion::from_string("1.99").unwrap(), BrowserVersion(1, 99));
	}

	#[test]
	fn test_browserversion_from_string_errors() {
		assert!(BrowserVersion::from_string("15.2-15.3").is_err());
		assert!(BrowserVersion::from_string("").is_err());
		assert!(BrowserVersion::from_string("not_a_number").is_err());
	}

	#[test]
	fn test_browserversion_from_string_as_range() {
		assert_eq!(
			BrowserVersion::from_string_as_range("15.2-15.3").unwrap(),
			(BrowserVersion(15, 2), BrowserVersion(15, 3))
		);
		assert_eq!(
			BrowserVersion::from_string_as_range("110.5-108.0").unwrap(),
			(BrowserVersion(108, 0), BrowserVersion(110, 5))
		);
		assert!(BrowserVersion::from_string_as_range("").is_err());
		assert!(BrowserVersion::from_string_as_range("not_a_number").is_err());
	}

	#[test]
	fn test_browserversion_comparison() {
		assert!(BrowserVersion(29, 0) > BrowserVersion(28, 0));
		assert!(BrowserVersion(29, 0) < BrowserVersion(30, 0));
		assert!(BrowserVersion(29, 1) > BrowserVersion(29, 0));
		assert!(BrowserVersion(29, 1) < BrowserVersion(30, 1));
		assert!(BrowserVersion(29, 1) < BrowserVersion(29, 2));
		assert!(BrowserVersion(0, 0) < BrowserVersion(0, 1));
		assert!(BrowserVersion(0, 1) < BrowserVersion(0, 2));
		assert!(BrowserVersion(0, 2) > BrowserVersion(0, 1));
	}

	#[test]
	fn test_browserversion_in_range() {
		assert!(BrowserVersion(0, 2).in_range((BrowserVersion(0, 0), BrowserVersion(1, 0))));
	}
}
