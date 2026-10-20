use crate::{BrowserVersion, NamedBrowserVersion};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BrowserSupport {
	pub chrome: BrowserVersion,
	pub chrome_android: BrowserVersion,
	pub edge: BrowserVersion,
	pub firefox: BrowserVersion,
	pub firefox_android: BrowserVersion,
	pub safari: BrowserVersion,
	pub safari_ios: BrowserVersion,
}

impl BrowserSupport {
	/// Reports whether `browser` supports the feature. A browser this data does not cover reports `false`.
	pub fn supports(&self, browser: NamedBrowserVersion) -> bool {
		let (version, since) = match browser {
			NamedBrowserVersion::Chrome(version) => (version, self.chrome),
			NamedBrowserVersion::ChromeAndroid(version) => (version, self.chrome_android),
			NamedBrowserVersion::Edge(version) => (version, self.edge),
			NamedBrowserVersion::Firefox(version) => (version, self.firefox),
			NamedBrowserVersion::FirefoxAndroid(version) => (version, self.firefox_android),
			NamedBrowserVersion::Safari(version) => (version, self.safari),
			NamedBrowserVersion::SafariIos(version) => (version, self.safari_ios),
			_ => return false,
		};
		since != BrowserVersion(0, 0) && version >= since
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const NEVER: BrowserVersion = BrowserVersion(0, 0);

	fn support() -> BrowserSupport {
		BrowserSupport {
			chrome: BrowserVersion(29, 0),
			chrome_android: BrowserVersion(29, 0),
			edge: BrowserVersion(12, 0),
			firefox: BrowserVersion(28, 0),
			firefox_android: BrowserVersion(28, 0),
			safari: BrowserVersion(9, 1),
			safari_ios: NEVER,
		}
	}

	#[test]
	fn supports_from_the_first_version() {
		assert!(support().supports(NamedBrowserVersion::Chrome(BrowserVersion(29, 0))));
		assert!(support().supports(NamedBrowserVersion::Safari(BrowserVersion(10, 0))));
		assert!(!support().supports(NamedBrowserVersion::Safari(BrowserVersion(9, 0))));
	}

	#[test]
	fn never_supported_stays_unsupported() {
		assert!(!support().supports(NamedBrowserVersion::SafariIos(BrowserVersion(18, 0))));
	}

	#[test]
	fn uncovered_browsers_are_unsupported() {
		assert!(!support().supports(NamedBrowserVersion::Opera(BrowserVersion(120, 0))));
	}
}
