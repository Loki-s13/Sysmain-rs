use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Sets up logging for the application.
pub fn logs() {
    // Here you create an EnvFilter to a level of INFO or higher, which excludes trace levels.
    // However, later in the code you use .with_max_level() to set the max level to TRACE, which is overridden by your env filter.
    // This is not good practice
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Use init() instead of finish() to automatically setup this subscriber as the global.
    // See docs for init() function.
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_env_filter(filter)
        .init();
}
