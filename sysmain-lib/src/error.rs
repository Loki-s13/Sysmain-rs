use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to delete file: {0}")]
	DeleteFile(PathBuf, #[source] std::io::Error),
	#[error("Failed to delete directory: {0}")]
	DeleteDirectory(PathBuf, #[source] std::io::Error),
	#[error("Error reading file type: {0}")]
	FileTypeError(PathBuf, #[source] std::io::Error),
	#[error("Failed to read temporary directory: {0}")]
	DirReadError(PathBuf, #[source] std::io::Error),
}
