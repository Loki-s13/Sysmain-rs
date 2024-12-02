use std::{
	fs::{self, DirEntry},
	path::PathBuf,
};

use super::BrowserSelection;

/// A struct representing browser cache data and operations
///
/// This struct holds paths to browser cache directories and provides
/// functionality to clear the cache contents.
pub struct BrowserCache {
	/// Vector of paths to browser cache directories
	cache_paths: Vec<PathBuf>,
}

impl BrowserCache {
	/// Creates a new `BrowserCache` instance for the specified browser type
	///
	/// # Arguments
	/// * `browser` - The browser type to create cache paths for, as a
	///   [`BrowserSelection`]
	///
	/// # Returns
	/// A new `BrowserCache` instance with paths initialized for the specified
	/// browser
	pub fn new(browser: BrowserSelection) -> Self {
		let cache_paths = browser.get_cache_path();
		BrowserCache { cache_paths }
	}

	// TODO: Change result type to `crate::Result<()>`
	/// Clears the cache for the browser, removing all files and subdirectories
	/// from the cache directories.
	///
	/// Only attempts to clear directories that exist. For each existing cache
	/// directory, recursively removes all contents.
	///
	/// # Returns
	/// * `Ok(())` if cache clearing was successful
	/// * `Err(std::io::Error)` if any filesystem operations failed
	pub fn clear(&self) -> Result<(), std::io::Error> {
		for path in self.cache_paths.iter().filter(|p| p.exists()) {
			info!("Clearing cache for {}", path.display());

			// Run the cache clearing operation in a closure, so we can log errors
			// appropriately TODO: Change result type to `crate::Result<()>`
			let ret: Result<(), std::io::Error> = (|| {
				let entries = fs::read_dir(path)?; // TODO: add map_err

				// Remove directory contents
				entries
					.filter_map(|e| e.inspect_err(|e| error!("Error reading directory entry: {e}")).ok())
					.for_each(|entry| {
						let path = entry.path();
						if let Err(why) = Self::process_clear_entry(entry) {
							warn!("Failed to clear cache file for {}: {}", path.display(), why);
						}
					});
				Ok(())
			})();

			// Log the error, but don't return to prevent short-circuting the loop
			if let Err(why) = ret {
				warn!("Failed to clear cache for {}: {}", path.display(), why);
			}
		}
		Ok(())
	}

	// TODO: Change result type to `crate::Result<()>`
	/// Processes a single entry in the cache directory, removing the directory
	/// or file.
	fn process_clear_entry(entry: DirEntry) -> Result<(), std::io::Error> {
		let path = entry.path();

		if path.is_dir() {
			fs::remove_dir_all(path)?; // TODO: add map_err
		} else if path.is_file() {
			fs::remove_file(path)?; // TODO: add map_err
		}
		Ok(())
	}
}
