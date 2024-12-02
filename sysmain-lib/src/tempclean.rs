use std::{env, fs, fs::DirEntry};

use crate::{error::Error, Result};

/// Process the directory entry, deleting the file or directory.
pub fn process_entry(entry: DirEntry) -> Result<()> {
	let file_path = entry.path();
	let file_type = entry
		.file_type()
		.map_err(|e| Error::FileTypeError(file_path.clone(), e))?;

	if file_type.is_dir() {
		fs::remove_dir_all(&file_path).map_err(|e| Error::DeleteDirectory(file_path, e))?;
	} else if file_type.is_file() {
		fs::remove_file(&file_path).map_err(|e| Error::DeleteFile(file_path, e))?;
	}

	Ok(())
}

/// Cleans out the temporary directory and its contents.
pub fn tempclean() -> Result<()> {
	let dir = env::temp_dir();
	info!("Temporary directory: {}", dir.display());

	let entries = fs::read_dir(&dir).map_err(|e| Error::DirReadError(dir, e))?;
	let mut successful_deletes = 0;
	let mut total_entries = 0;

	for entry_result in entries {
		total_entries += 1;
		// use inspect_err to log the error if the entry_result is of the Err variant
		if let Ok(entry) = entry_result.inspect_err(|e| error!("error reading directory entry: {e}")) {
			if process_entry(entry).inspect_err(|e| error!("{e}")).is_ok() {
				successful_deletes += 1;
			}
		}
	}
	info!("successfully deleted {} out of {} files", successful_deletes, total_entries);

	Ok(())
}

//TODO() : Ask for user validation to delete files after ennumerating all files
