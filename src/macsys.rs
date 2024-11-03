use std::env;

/// Cleans out the temporary directory and its contents.
pub fn tempclean() {
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
}
