use std::env;
use std::fs;

/// Cleans out the temporary directory and its contents.
pub fn tempclean() {
    let dir = env::temp_dir();
    info!("Temporary directory: {}", dir.display());

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(file) => {
                        let file_path = file.path();
                        match fs::remove_file(&file_path) {
                            Ok(_) => info!("Deleted file: {}", file_path.display()),
                            Err(e) => {
                                error!(
                                    "Failed to delete file: {} Error: {}",
                                    file_path.display(),
                                    e
                                )
                            }
                        }
                    }
                    Err(e) => error!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => error!("Error reading temporary directory: {}", e),
    }
}
