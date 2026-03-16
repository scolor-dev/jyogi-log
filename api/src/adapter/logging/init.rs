use tracing_subscriber::{EnvFilter, fmt};

pub fn init(rust_log: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(rust_log));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}