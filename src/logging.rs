use tracing_subscriber::{fmt, EnvFilter};
use tracing::{Level};


pub fn logs(){
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = fmt().with_max_level(Level::TRACE).with_env_filter(filter).finish();


    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}