use tracing::{ Level};
use tracing_subscriber::EnvFilter;
use std::{fs::File, sync::Mutex, path::PathBuf};
use dirs::{desktop_dir};


/// Sets up logging for the application.
pub fn logs() {

    let desktop_path = desktop_dir();

    let mut log_file = desktop_path.unwrap();
    log_file.push("Temp_logs.log");


    let log_file = match  File::create(&log_file) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to log file!: {}", e);
            return;
        }


    };



    // Here you create an EnvFilter to a level of INFO or higher, which excludes trace levels.
    // However, later in the code you use .with_max_level() to set the max level to TRACE, which is overridden by your env filter.
    // This is not good practice
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Use init() instead of finish() to automatically setup this subscriber as the global.
    // See docs for init() function.
    tracing_subscriber::fmt()
        .with_writer(Mutex::new(log_file))
        .with_max_level(Level::TRACE)
        .with_env_filter(filter)
        .init();
}
