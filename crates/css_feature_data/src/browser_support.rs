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
	pub fn supports(&self, version: NamedBrowserVersion) -> bool {
		match version {
			NamedBrowserVersion::Chrome(ver) => ver >= self.chrome,
			NamedBrowserVersion::ChromeAndroid(ver) => ver >= self.chrome_android,
			NamedBrowserVersion::Edge(ver) => ver >= self.edge,
			NamedBrowserVersion::Firefox(ver) => ver >= self.firefox,
			NamedBrowserVersion::FirefoxAndroid(ver) => ver >= self.firefox_android,
			NamedBrowserVersion::Safari(ver) => ver >= self.safari,
			NamedBrowserVersion::SafariIos(ver) => ver >= self.safari_ios,
			_ => {
				dbg!(version);
				false
			}
		}
	}
}
