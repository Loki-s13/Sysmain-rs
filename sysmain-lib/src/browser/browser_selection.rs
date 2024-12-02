use std::path::PathBuf;

/// An enum used to hold the different chromium prefix paths. (e.g Chrome,
/// Chrome SxS, etc.)
pub enum ChromiumPrefix {
	Chrome,
	Edge,
	// Add more chromium browsers here
}

impl ChromiumPrefix {
	/// Returns a vector of strings representing the prefix paths for the
	/// browser.
	pub fn get_prefix(&self) -> Vec<&str> {
		match self {
			ChromiumPrefix::Chrome => vec!["Google\\Chrome", "Google\\Chrome SxS"],
			ChromiumPrefix::Edge => vec!["Microsoft\\Edge"],
		}
	}
}

/// An enum representing different browser types supported by the application.
/// Currently supports Chromium-based browsers (Chrome, Edge) and Firefox.
pub enum BrowserSelection {
	/// Chromium-based browsers, with specific prefix paths defined in
	/// [`ChromiumPrefix`]
	Chromium(ChromiumPrefix),
	/// Mozilla Firefox browser
	Firefox,
}

impl BrowserSelection {
	/// Gets the base directory for the browser storage location, depending upon
	/// OS.
	///
	/// Currently only supports Windows, using the LOCALAPPDATA environment
	/// variable. Returns a [`PathBuf`] pointing to the base storage directory.
	fn get_base_dir(&self) -> PathBuf {
		#[cfg(target_os = "windows")]
		{
			let home_dir = std::env::var("LOCALAPPDATA").expect("Failed to get APPDATA environment variable");
			PathBuf::from(home_dir)
		}
		// TODO: add support for other operating systems
	}

	/// Returns a vector of paths to the cache directories for the browser.
	///
	/// For Chromium-based browsers, returns paths to both the main Cache and
	/// Code Cache directories for each browser prefix (e.g. Chrome, Chrome
	/// SxS, Edge).
	///
	/// For Firefox, returns the path to the Profiles directory which contains
	/// cache data.
	///
	/// # Returns
	/// A [`Vec<PathBuf>`] containing all relevant cache directory paths for the
	/// selected browser
	pub fn get_cache_path(&self) -> Vec<PathBuf> {
		let base_dir = self.get_base_dir();
		match self {
			BrowserSelection::Chromium(prefix) => {
				let paths = prefix
					.get_prefix()
					.iter()
					.flat_map(|pre| {
						// Create the dir for the base_dir + prefix + Default profile location
						let profile_dir = base_dir.join(pre).join("User Data\\Default");
						// Return the Cache and Code Cache dirs for the profile
						vec![profile_dir.join("Cache"), profile_dir.join("Code Cache")]
					})
					.collect::<Vec<PathBuf>>();
				paths
			},
			BrowserSelection::Firefox => vec![base_dir.join("Mozilla\\Firefox\\Profiles")],
		}
	}

	/// Creates a new [`BrowserSelection`] for a Chromium-based browser with the
	/// given prefix.
	pub fn create_chromium(prefix: ChromiumPrefix) -> Self { BrowserSelection::Chromium(prefix) }

	/// Creates a new [`BrowserSelection`] for the Mozilla Firefox browser.
	pub fn create_firefox() -> Self { BrowserSelection::Firefox }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_cache_path() {
		let browser = BrowserSelection::create_chromium(ChromiumPrefix::Chrome);
		let cache_paths = browser.get_cache_path();
		assert!(!cache_paths.is_empty());
	}
}
