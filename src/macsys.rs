use std::env;




#[cfg(target_os = "macos")]
pub fn tempclean_Mac(){
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
}